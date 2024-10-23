use solana_client::rpc_client::RpcClient;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::transaction::Transaction;

struct SolanaRelayer {
    client: RpcClient,
    signer: Keypair,
}

impl SolanaRelayer {
    // Relay message to Solana program
    pub async fn relay_message(&self, program_id: String, message: String) -> Result<(), Box<dyn std::error::Error>> {
        let instruction = solana_program::instruction::Instruction::new_with_bincode(
            program_id.parse()?,
            &message.into_bytes(),
            vec![],
        );
        let tx = Transaction::new_signed_with_payer(&[instruction], Some(&self.signer.pubkey()), &[&self.signer], self.client.get_latest_blockhash()?);
        self.client.send_and_confirm_transaction(&tx).await?;
        Ok(())
    }
}
