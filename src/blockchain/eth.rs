use ethers::prelude::*;
use ethers::contract::abigen;
use ethers::types::{Address, U256};
use std::convert::TryFrom;
use ethers::prelude::*;
use ethers::types::Bytes;

// Auto-generate the bindings for the MessagingStorage contract
abigen!(
    MessagingStorage,
    r#"[function storeMessageHash(string memory ipfsHash) public]
        [function getMessages(address user) public view returns (tuple(string ipfsHash, uint256 timestamp)[])]
    "#
);

// Function to interact with the smart contract and store an IPFS hash
pub async fn store_message_hash(
    signer: LocalWallet,
    contract_address: Address,
    ipfs_hash: String,
) -> Result<TxHash, Box<dyn std::error::Error>> {
    // Connect to the provider (Ethereum mainnet or testnet)
    let provider = Provider::<Http>::try_from("https://rinkeby.infura.io/v3/YOUR_INFURA_PROJECT_ID")?;
    let client = SignerMiddleware::new(provider, signer.clone());

    // Load the contract at the deployed address
    let contract = MessagingStorage::new(contract_address, client.clone());

    // Send the transaction to store the IPFS hash
    let tx = contract.store_message_hash(ipfs_hash).send().await?;
    let receipt = tx.confirmations(1).await?;

    Ok(receipt.unwrap().transaction_hash)
}

// Function to retrieve messages for a given address
pub async fn get_messages(
    provider: Provider<Http>,
    contract_address: Address,
    user_address: Address,
) -> Result<Vec<(String, u64)>, Box<dyn std::error::Error>> {
    // Load the contract at the deployed address
    let contract = MessagingStorage::new(contract_address, provider);

    // Retrieve the messages for the user
    let messages = contract.get_messages(user_address).call().await?;
    Ok(messages)
}



// Function to verify a message signature
pub async fn verify_message_signature(
    message: String,
    signature: Bytes,
    signer_address: Address,
) -> Result<bool, Box<dyn std::error::Error>> {
    // Recover the signer address from the message and signature
    let recovered = ethers::utils::recover_signer(message.clone(), &signature)?;

    // Check if the recovered address matches the given signer address
    Ok(recovered == signer_address)
}

// Function to fetch IPFS hashes stored on-chain for a specific user
pub async fn get_messages_on_chain(
    provider: Provider<Http>,
    contract_address: Address,
    user_address: Address,
) -> Result<Vec<(String, u64)>, Box<dyn std::error::Error>> {
    // Load the contract
    let contract = MessagingStorage::new(contract_address, provider);

    // Fetch the messages stored on-chain for the user
    let messages = contract.get_messages(user_address).call().await?;
    Ok(messages)
}

// Function to cross-check if the message hash matches the on-chain data
pub async fn verify_message_on_chain(
    provider: Provider<Http>,
    contract_address: Address,
    user_address: Address,
    ipfs_hash: String,
) -> Result<bool, Box<dyn std::error::Error>> {
    // Fetch the on-chain messages for the user
    let messages = get_messages_on_chain(provider, contract_address, user_address).await?;

    // Check if the IPFS hash exists in the user's on-chain messages
    for (stored_hash, _) in messages {
        if stored_hash == ipfs_hash {
            return Ok(true);
        }
    }
    Ok(false)
}
