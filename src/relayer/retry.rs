use std::{time::Duration, future::Future};
use tokio::time::sleep;

// Retry logic for failed relays
pub async fn retry<F, T, E>(operation: F, max_retries: usize) -> Result<T, E>
where
    F: Future<Output = Result<T, E>>,
    E: std::fmt::Debug,
{
    let mut retries = 0;
    loop {
        match operation.await {
            Ok(result) => return Ok(result),
            Err(err) => {
                retries += 1;
                if retries >= max_retries {
                    return Err(err);
                }
                println!("Retry {}/{} failed: {:?}", retries, max_retries, err);
                sleep(Duration::from_secs(5)).await; // Wait before retrying
            }
        }
    }
}
