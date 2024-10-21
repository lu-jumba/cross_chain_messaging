use super::{BlockchainClient, EVMClient, SubstrateClient};
use ethers::prelude::*;
use substrate_api_client::Api;
use std::sync::Arc;

// Define an enum for chain types
pub enum ChainType {
    EVM,
    Substrate,
}

// Struct to dynamically select the appropriate blockchain client
pub struct ChainSelector {
    pub chain_type: ChainType,
}

impl ChainSelector {
    pub async fn get_client(&self) -> Arc<dyn BlockchainClient + Send + Sync> {
        match self.chain_type {
            ChainType::EVM => {
                // Create EVM client (e.g., Ethereum)
                let signer: LocalWallet = "YOUR_PRIVATE_KEY".parse().unwrap();
                Arc::new(EVMClient { signer })
            }
            ChainType::Substrate => {
                // Create Substrate client (e.g., Polkadot)
                let api = Api::new("wss://rpc.polkadot.io").unwrap();
                Arc::new(SubstrateClient { api })
            }
        }
    }
}
