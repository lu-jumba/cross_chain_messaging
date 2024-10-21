use ethers::prelude::*;
use substrate_api_client::{Api, XtStatus};
use std::convert::TryFrom;

// Function to listen for cross-chain messages on EVM
pub async fn listen_for_cross_chain_messages(signer: LocalWallet, contract_address: Address, substrate_api: Api<(), XtStatus>) {
    let provider = Provider::<Http>::try_from("https://mainnet.infura.io/v3/YOUR_INFURA_PROJECT_ID").unwrap();
    let client = SignerMiddleware::new(provider, signer);

    // Subscribe to the CrossChainMessage event on EVM
    let filter = Filter::new().address(contract_address).event("CrossChainMessage(address,string,string,address,uint256)");
    let logs = client.subscribe_logs(&filter).await.unwrap();

    while let Some(log) = logs.next().await {
        let parsed_log = ethers::abi::decode(&["address", "string", "string", "address", "uint256"], &log.data).unwrap();

        // Parse the message details
        let sender: Address = parsed_log[0].into_address().unwrap();
        let ipfs_hash: String = parsed_log[1].into_string().unwrap();
        let target_chain: String = parsed_log[2].into_string().unwrap();
        let target_recipient: Address = parsed_log[3].into_address().unwrap();

        // If target chain is Substrate-based (Polkadot)
        if target_chain == "Polkadot" {
            println!("Relaying message to Substrate chain: {}", ipfs_hash);
            relay_to_substrate(ipfs_hash, target_recipient, substrate_api.clone()).await.unwrap();
        }
    }
}

// Function to relay message to Substrate chain
pub async fn relay_to_substrate(ipfs_hash: String, recipient: Address, api: Api<(), XtStatus>) -> Result<(), Box<dyn std::error::Error>> {
    // Send a cross-chain message to the recipient on Substrate
    let recipient_account = substrate_api_client::AccountId::from_ss58check(&recipient.to_string()).unwrap();
    let xt = api.balance_transfer(recipient_account.clone(), 1_000_000); // Example call (adjust accordingly)
    let _ = api.send_extrinsic(xt.hex_encode(), XtStatus::InBlock)?;
    Ok(())
}
