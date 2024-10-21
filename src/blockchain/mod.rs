use ethers::prelude::*;
use ethers::signers::{LocalWallet, Signer};
use ethers::types::TransactionRequest;
use std::convert::TryFrom;
use async_trait::async_trait;
use ethers::prelude::*;
use substrate_api_client::{Api, XtStatus};

pub async fn send_transaction(signer: LocalWallet, ipfs_hash: String) -> Result<TxHash, Box<dyn std::error::Error>> {
    let provider = Provider::<Http>::try_from("https://mainnet.infura.io/v3/INFURA_PROJECT_ID")?;
    let client = SignerMiddleware::new(provider, signer.clone());

    // Assuming the IPFS hash is stored as a simple string in a smart contract
    let tx = TransactionRequest::new()
        .to("YOUR_SMART_CONTRACT_ADDRESS")
        .data(ipfs_hash.into_bytes())
        .from(signer.address());

    let pending_tx = client.send_transaction(tx, None).await?;
    let receipt = pending_tx.confirmations(1).await?;

    Ok(receipt.unwrap().transaction_hash)
}



// Define a common trait for blockchain interactions
#[async_trait]
pub trait BlockchainClient {
    async fn send_message(&self, recipient: String, message: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn verify_message(&self, message_hash: String) -> Result<bool, Box<dyn std::error::Error>>;
}

// EVM-compatible client using ethers-rs
pub struct EVMClient {
    pub signer: LocalWallet,
}

#[async_trait]
impl BlockchainClient for EVMClient {
    async fn send_message(&self, recipient: String, message: String) -> Result<(), Box<dyn std::error::Error>> {
        // Send message or hash to an EVM-compatible chain
        let recipient_address = recipient.parse::<Address>()?;
        let tx = TransactionRequest::new().to(recipient_address).data(message.into_bytes());

        // Sign and send the transaction
        let provider = Provider::<Http>::try_from("https://mainnet.infura.io/v3/YOUR_INFURA_PROJECT_ID")?;
        let client = SignerMiddleware::new(provider, self.signer.clone());
        client.send_transaction(tx, None).await?;
        Ok(())
    }

    async fn verify_message(&self, message_hash: String) -> Result<bool, Box<dyn std::error::Error>> {
        // Verify message stored on EVM chain (e.g., using event logs)
        // Implementation depends on contract used for message storage
        Ok(true)
    }
}

// Substrate/Polkadot client using substrate-api-client
pub struct SubstrateClient {
    pub api: Api<(), XtStatus>,
}

#[async_trait]
impl BlockchainClient for SubstrateClient {
    async fn send_message(&self, recipient: String, message: String) -> Result<(), Box<dyn std::error::Error>> {
        // Sending message to a Substrate-based chain
        let recipient_account = substrate_api_client::AccountId::from_ss58check(&recipient)?;
        let xt = self.api.balance_transfer(recipient_account.clone(), 1_000_000); // Example transfer call
        let _ = self.api.send_extrinsic(xt.hex_encode(), XtStatus::InBlock)?;
        Ok(())
    }

    async fn verify_message(&self, message_hash: String) -> Result<bool, Box<dyn std::error::Error>> {
        // Verify message in a Substrate-based chain using storage queries or logs
        // This would involve querying storage for the message hash
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethers::utils::parse_ether;

    #[tokio::test]
    async fn test_send_transaction() {
        let wallet = "PRIVATE_KEY".parse::<LocalWallet>().unwrap();
        let recipient = "0xRecipientAddress".parse::<Address>().unwrap();
        let value = parse_ether(0.1).unwrap();

        let tx_hash = send_transaction(wallet, recipient, value).await.unwrap();
        println!("Transaction sent: {:?}", tx_hash);
    }
}
