use ethers::prelude::*;
use substrate_api_client::{Api, XtStatus};

// Batch relaying for Substrate-based chains
pub async fn batch_relay_to_substrate(
    api: Api<(), XtStatus>,
    messages: Vec<(String, Address)>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Batch all messages into a single extrinsic
    let mut batch_xts = vec![];

    for (ipfs_hash, recipient) in messages {
        let recipient_account =
            substrate_api_client::AccountId::from_ss58check(&recipient.to_string())
                .expect("Invalid recipient account for Substrate");

        let xt = api.balance_transfer(recipient_account, 1_000_000); // Adjust accordingly
        batch_xts.push(xt);
    }

    // Send batch extrinsic
    let batch = api.batch(batch_xts);
    let _ = api
        .send_extrinsic(batch.hex_encode(), XtStatus::InBlock)
        .expect("Failed to send batch extrinsic");

    println!("Successfully relayed {} messages in batch to Substrate", messages.len());
    Ok(())
}

// Batch relaying for EVM-based chains
pub async fn batch_relay_to_evm(
    provider: Provider<Http>,
    signer: LocalWallet,
    messages: Vec<(String, Address)>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create transactions for all messages
    let mut batch_txs = vec![];

    for (ipfs_hash, recipient) in messages {
        let tx = TransactionRequest::new().to(recipient).data(ipfs_hash.into_bytes());
        batch_txs.push(tx);
    }

    // Send transactions in a batch
    for tx in batch_txs {
        let pending_tx = provider
            .send_transaction(tx, None)
            .await?
            .await
            .expect("Failed to send transaction");
        println!("Relayed message in batch with TX hash: {:?}", pending_tx);
    }

    println!("Successfully relayed {} messages in batch to EVM", messages.len());
    Ok(())
}

// Optimized batch relaying for EVM-based chains
pub async fn optimized_batch_relay_to_evm(
    provider: Provider<Http>,
    signer: LocalWallet,
    messages: Vec<(String, Address)>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut batch_data = vec![];

    for (ipfs_hash, recipient) in messages {
        // Create the transaction data
        let tx_data = format!("{}:{}", ipfs_hash, recipient);
        batch_data.push(tx_data);
    }

    // Send the batch transaction with all the message data
    let tx = TransactionRequest::new().to(signer.address()).data(batch_data.concat().into_bytes());

    let pending_tx = provider.send_transaction(tx, None).await?;
    let receipt = pending_tx.await?;
    println!("Batch of {} messages relayed with TX hash: {:?}", batch_data.len(), receipt.transaction_hash);

    Ok(())
}