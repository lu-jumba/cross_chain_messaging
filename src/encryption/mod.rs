use rsa::{RsaPrivateKey, RsaPublicKey, PaddingScheme};
use rand::rngs::OsRng;

pub fn encrypt_message(public_key: &RsaPublicKey, message: &[u8]) -> Vec<u8> {
    let mut rng = OsRng;
    public_key
        .encrypt(&mut rng, PaddingScheme::new_pkcs1v15_encrypt(), message)
        .expect("failed to encrypt")
}

pub fn decrypt_message(private_key: &RsaPrivateKey, encrypted_message: &[u8]) -> Vec<u8> {
    private_key
        .decrypt(PaddingScheme::new_pkcs1v15_encrypt(), encrypted_message)
        .expect("failed to decrypt")
}


#[cfg(test)]
mod tests {
    use super::*;
    use rsa::{RsaPrivateKey, RsaPublicKey};

    #[test]
    fn test_encryption_decryption() {
        let mut rng = OsRng;
        let bits = 2048;
        let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate key");
        let public_key = RsaPublicKey::from(&private_key);

        let message = b"Hello, Blockchain!";
        let encrypted = encrypt_message(&public_key, message);
        let decrypted = decrypt_message(&private_key, &encrypted);

        assert_eq!(message.to_vec(), decrypted);
    }
}
