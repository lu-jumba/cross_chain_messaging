use ethers::prelude::*;
use substrate_api_client::{Api, XtStatus};
use futures::stream::StreamExt;
use std::convert::TryFrom;

// Define supported chain types
#[derive(Debug)]
enum ChainType {
    EVM,
    Substrate,
    Other(String), // For future chains
}

// Relayer structure with dynamic support for multiple chains
struct CrossChainRelayer {
    evm_provider: Provider<Http>,
    substrate_api: Api<(), XtStatus>,
    signer: LocalWallet,
}

impl CrossChainRelayer {
    // Initialize the relayer with chain-specific clients
    pub fn new(evm_endpoint: &str, substrate_endpoint: &str, signer: LocalWallet) -> Self {
        let evm_provider = Provider::<Http>::try_from(evm_endpoint).expect("Invalid EVM endpoint");
        let substrate_api = Api::new(substrate_endpoint).expect("Invalid Substrate endpoint");
        Self {
            evm_provider,
            substrate_api,
            signer,
        }
    }

    // Listen for cross-chain message events on the source chain (e.g., EVM)
    pub async fn listen_and_relay(&self, contract_address: Address) {
        let client = SignerMiddleware::new(self.evm_provider.clone(), self.signer.clone());

        let filter = Filter::new()
            .address(contract_address)
            .event("CrossChainMessage(address,string,string,address,uint256)");

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

            // Dynamically handle different target chains
            match target_chain.as_str() {
                "Polkadot" => self.relay_to_substrate(ipfs_hash, target_recipient).await.unwrap(),
                "EVM" => self.relay_to_evm(ipfs_hash, target_recipient).await.unwrap(),
                _ => println!("Unsupported target chain: {}", target_chain),
            }
        }
    }

    // Relay message to Substrate chain
    async fn relay_to_substrate(
        &self,
        ipfs_hash: String,
        recipient: Address,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let recipient_account = substrate_api_client::AccountId::from_ss58check(&recipient.to_string())
            .expect("Invalid recipient account for Substrate");

        // Example of sending IPFS hash to Substrate
        let xt = self.substrate_api.balance_transfer(recipient_account, 1_000_000); // Adjust accordingly
        let _ = self
            .substrate_api
            .send_extrinsic(xt.hex_encode(), XtStatus::InBlock)
            .expect("Failed to send extrinsic to Substrate");

        println!("Message successfully relayed to Substrate with IPFS hash: {}", ipfs_hash);
        Ok(())
    }

    // Relay message to EVM chain
    async fn relay_to_evm(
        &self,
        ipfs_hash: String,
        recipient: Address,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let tx = TransactionRequest::new()
            .to(recipient)
            .data(ipfs_hash.into_bytes());

        let pending_tx = self
            .evm_provider
            .send_transaction(tx, None)
            .await?
            .await
            .expect("Failed to send transaction to EVM");

        println!("Message successfully relayed to EVM with IPFS hash: {}", ipfs_hash);
        Ok(())
    }
}
