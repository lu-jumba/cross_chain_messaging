use ipfs_api::IpfsClient;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

pub async fn store_message_to_ipfs(message: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    let client = IpfsClient::default();
    let res = client.add(message).await?;
    Ok(res.hash)
}

pub async fn retrieve_message_from_ipfs(hash: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let client = IpfsClient::default();
    let res = client.cat(hash).await?;
    Ok(res.to_vec())
}



#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ipfs_storage() {
        let message = b"Blockchain Message to IPFS!";
        let hash = store_message_to_ipfs(message).await.unwrap();
        println!("Stored to IPFS with hash: {}", hash);

        let retrieved_message = retrieve_message_from_ipfs(&hash).await.unwrap();
        assert_eq!(message.to_vec(), retrieved_message);
    }
}
