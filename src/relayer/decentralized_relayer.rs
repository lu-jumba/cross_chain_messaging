use ethers::prelude::*;
use substrate_api_client::{Api, XtStatus};
use futures::stream::StreamExt;
use std::convert::TryFrom;
use tokio::time::sleep;
use std::time::Duration;

struct DecentralizedRelayer {
    evm_provider: Provider<Http>,
    substrate_api: Api<(), XtStatus>,
    signer: LocalWallet,
    consensus_contract: Contract<Provider<Http>>,
}

impl DecentralizedRelayer {
    // Initialize the decentralized relayer
    pub fn new(
        evm_endpoint: &str,
        substrate_endpoint: &str,
        signer: LocalWallet,
        consensus_contract_address: Address,
    ) -> Self {
        let evm_provider = Provider::<Http>::try_from(evm_endpoint).expect("Invalid EVM endpoint");
        let substrate_api = Api::new(substrate_endpoint).expect("Invalid Substrate endpoint");
        let abi = include_bytes!("RelayerConsensus.abi");
        let consensus_contract = Contract::from_json(evm_provider.clone(), consensus_contract_address, abi).unwrap();
        Self {
            evm_provider,
            substrate_api,
            signer,
            consensus_contract,
        }
    }

    // Listen for cross-chain messages and relay them
    pub async fn listen_and_relay(&self, contract_address: Address) {
        let client = SignerMiddleware::new(self.evm_provider.clone(), self.signer.clone());
        let filter = Filter::new().address(contract_address).event("CrossChainMessage(address,string,string,address,uint256)");

        let logs = client.subscribe_logs(&filter).await.unwrap();

        while let Some(log) = logs.next().await {
            let parsed_log = ethers::abi::decode(
                &["address", "string", "string", "address", "uint256"],
                &log.data,
            )
            .unwrap();

            // Parse log details
            let sender: Address = parsed_log[0].into_address().unwrap();
            let ipfs_hash: String = parsed_log[1].into_string().unwrap();
            let target_chain: String = parsed_log[2].into_string().unwrap();
            let target_recipient: Address = parsed_log[3].into_address().unwrap();

            println!("Relaying message from sender: {} to target chain: {}", sender, target_chain);

            // Relay to the appropriate chain
            if target_chain == "Polkadot" {
                self.relay_to_substrate(ipfs_hash, target_recipient).await.unwrap();
            } else if target_chain == "EVM" {
                self.relay_to_evm(ipfs_hash, target_recipient).await.unwrap();
            }

            // Submit proof of relay to the consensus contract
            let message_hash = keccak256(&ipfs_hash);
            self.submit_proof(message_hash).await.unwrap();
        }
    }

    // Submit proof of relay to the consensus contract
    async fn submit_proof(&self, message_hash: [u8; 32]) -> Result<(), Box<dyn std::error::Error>> {
        let tx = self.consensus_contract
            .method::<_, H256>("submitProof", message_hash)
            .unwrap()
            .send()
            .await?;
        println!("Proof of relay submitted with TX hash: {:?}", tx);
        Ok(())
    }

    // Relay to Substrate
    async fn relay_to_substrate(
        &self,
        ipfs_hash: String,
        recipient: Address,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let recipient_account =
            substrate_api_client::AccountId::from_ss58check(&recipient.to_string())
                .expect("Invalid recipient account for Substrate");
        let xt = self.substrate_api.balance_transfer(recipient_account, 1_000_000);
        self.substrate_api
            .send_extrinsic(xt.hex_encode(), XtStatus::InBlock)
            .expect("Failed to send extrinsic");
        println!("Message successfully relayed to Substrate with IPFS hash: {}", ipfs_hash);
        Ok(())
    }

    // Relay to EVM
    async fn relay_to_evm(
        &self,
        ipfs_hash: String,
        recipient: Address,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let tx = TransactionRequest::new().to(recipient).data(ipfs_hash.into_bytes());
        let pending_tx = self.evm_provider.send_transaction(tx, None).await?;
        println!("Message successfully relayed to EVM with IPFS hash: {:?}", pending_tx);
        Ok(())
    }
}
