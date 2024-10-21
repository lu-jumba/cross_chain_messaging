use ethers::prelude::*;
use substrate_api_client::Api;

// Verify the message signature before relaying
pub async fn verify_message_signature(
    message: &str,
    signature: Bytes,
    signer_address: Address,
) -> Result<bool, Box<dyn std::error::Error>> {
    // Recover the signer address from the message and signature
    let recovered = ethers::utils::recover_signer(message.clone(), &signature)?;

    // Ensure the recovered address matches the signer address
    Ok(recovered == signer_address)
}

// Verify message integrity before relaying across chains
pub async fn verify_message_integrity(
    api: Api<(), XtStatus>,
    evm_provider: Provider<Http>,
    message: &str,
    signature: Bytes,
    signer_address: Address,
) -> Result<(), Box<dyn std::error::Error>> {
    let valid_signature = verify_message_signature(message, signature, signer_address).await?;
    if !valid_signature {
        return Err("Invalid message signature".into());
    }

    // Additional checks like message hash integrity (optional)
    println!("Message integrity verified successfully");
    Ok(())
}
