use secp256k1::{Secp256k1, SecretKey, PublicKey};

pub fn generate_wallet() -> (SecretKey, PublicKey) {
    let secp = Secp256k1::new();
    let (secret_key, public_key) = secp.generate_keypair(&mut rand::thread_rng());
    (secret_key, public_key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_generation() {
        let (secret_key, public_key) = generate_wallet();
        println!("Generated Secret Key: {:?}", secret_key);
        println!("Generated Public Key: {:?}", public_key);
    }
}
