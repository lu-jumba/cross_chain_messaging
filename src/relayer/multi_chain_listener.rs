use ethers::prelude::*;
use substrate_api_client::{Api, XtStatus};
use futures::stream::StreamExt;
use tokio::task;
use std::convert::TryFrom;

struct MultiChainRelayer {
    evm_providers: Vec<Provider<Http>>,
    substrate_apis: Vec<Api<(), XtStatus>>,
    signer: LocalWallet,
    consensus_contract: Contract<Provider<Http>>,
}

impl MultiChainRelayer {
    // Initialize the relayer with multiple EVM and Substrate endpoints
    pub fn new(
        evm_endpoints: Vec<&str>,
        substrate_endpoints: Vec<&str>,
        signer: LocalWallet,
        consensus_contract_address: Address,
    ) -> Self {
        let evm_providers = evm_endpoints
            .iter()
            .map(|&url| Provider::<Http>::try_from(url).expect("Invalid EVM endpoint"))
            .collect();
        let substrate_apis = substrate_endpoints
            .iter()
            .map(|&url| Api::new(url).expect("Invalid Substrate endpoint"))
            .collect();
        let abi = include_bytes!("RelayerConsensus.abi");
        let consensus_contract = Contract::from_json(evm_providers[0].clone(), consensus_contract_address, abi).unwrap();
        Self {
            evm_providers,
            substrate_apis,
            signer,
            consensus_contract,
        }
    }

    // Start relaying across multiple chains
    pub async fn start_relaying(&self) {
        let mut tasks = vec![];

        // Start listening for events on each EVM chain in parallel
        for provider in self.evm_providers.clone() {
            let signer = self.signer.clone();
            let consensus_contract = self.consensus_contract.clone();
            tasks.push(task::spawn(async move {
                listen_for_cross_chain_messages(provider, signer, consensus_contract).await;
            }));
        }

        // Start listening for events on each Substrate chain in parallel
        for api in self.substrate_apis.clone() {
            tasks.push(task::spawn(async move {
                listen_for_substrate_messages(api).await;
            }));
        }

        // Wait for all tasks to complete
        futures::future::join_all(tasks).await;
    }
}

// EVM event listener
async fn listen_for_cross_chain_messages(
    provider: Provider<Http>,
    signer: LocalWallet,
    consensus_contract: Contract<Provider<Http>>,
) {
    // Filter to listen for CrossChainMessage event on EVM chains
    let filter = Filter::new().event("CrossChainMessage(address,string,string,address,uint256)");

    let logs = provider.subscribe_logs(&filter).await.unwrap();
    while let Some(log) = logs.next().await {
        // Process and relay the message here
    }
}

// Substrate event listener
async fn listen_for_substrate_messages(api: Api<(), XtStatus>) {
    // Add logic to listen for Substrate-based cross-chain messages
}
