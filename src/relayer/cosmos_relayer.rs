use cosmos_sdk::tx::{BroadcastMode, Transaction};
use cosmos_sdk::rpc::Client;
use cosmos_sdk::signer::LocalSigner;

struct CosmosRelayer {
    client: Client,
    signer: LocalSigner,
}

impl CosmosRelayer {
    // Relay message to Cosmos zone
    pub async fn relay_message(&self, recipient: String, message: String) -> Result<(), Box<dyn std::error::Error>> {
        let tx = Transaction::new().add_message(message);
        self.client.broadcast_transaction(tx, BroadcastMode::Sync).await?;
        Ok(())
    }
}
