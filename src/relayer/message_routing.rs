impl MultiChainRelayer {
    // Route messages based on the target chain
    async fn route_message(
        &self,
        target_chain: String,
        ipfs_hash: String,
        target_recipient: Address,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if target_chain == "Polkadot" {
            // Relay to Substrate-based chain
            self.relay_to_substrate(ipfs_hash, target_recipient).await?;
        } else if target_chain == "BSC" || target_chain == "Avalanche" || target_chain == "EVM" {
            // Relay to EVM-based chain
            self.relay_to_evm(ipfs_hash, target_recipient).await?;
        } else {
            println!("Unsupported target chain: {}", target_chain);
        }
        Ok(())
    }

    // Relay to Substrate-based chain
    async fn relay_to_substrate(&self, ipfs_hash: String, recipient: Address) -> Result<(), Box<dyn std::error::Error>> {
        let recipient_account = substrate_api_client::AccountId::from_ss58check(&recipient.to_string())?;
        let api = &self.substrate_apis[0]; // Choose the correct Substrate API
        let xt = api.balance_transfer(recipient_account, 1_000_000);
        api.send_extrinsic(xt.hex_encode(), XtStatus::InBlock)?;
        Ok(())
    }

    // Relay to EVM-based chain
    async fn relay_to_evm(&self, ipfs_hash: String, recipient: Address) -> Result<(), Box<dyn std::error::Error>> {
        let tx = TransactionRequest::new().to(recipient).data(ipfs_hash.into_bytes());
        let provider = &self.evm_providers[0]; // Choose the correct EVM provider
        provider.send_transaction(tx, None).await?;
        Ok(())
    }
}
