use chacha20poly1305::{
    aead::{Aead, KeyInit, Nonce, Payload}, Key, XChaCha20Poly1305
};
use rand::{rngs::OsRng, RngCore};

struct CryptoEngine {
    cipher: XChaCha20Poly1305,
}

impl CryptoEngine {
    fn new(key : &[u8;32]) -> Self {
            let cipher = XChaCha20Poly1305::new(Key::from_slice(key));
            Self { cipher }
    }
    fn encrypt (&self, message : &str, AAD : &str) -> Result<Vec<u8>, &'static str>{
        let nonce = generate_nonce();
        let payload = Payload {
            msg: message.as_bytes(),
            aad: AAD.as_bytes(),
        };
        // Use self.cipher instead of CryptoEngine::cipher
        let ciphertext = self.cipher.encrypt(&nonce.into(), payload)
            .map_err(|_| "Encryption failed")?;
        
        // Combine nonce and ciphertext as requested
        let mut result = nonce.to_vec();
        result.extend_from_slice(&ciphertext);
        Ok(result)
    }
}

fn generate_nonce() -> [u8;24] {
    let mut nonce = [0;24];
    OsRng.fill_bytes(&mut nonce);
    nonce
}




fn main() {
    println!("Hello, world!");
    let nonce = generate_nonce();
    println!("Nonce: {:?}", nonce);
    let key = [1u8; 32];
    let engine = CryptoEngine::new(&key);
    let result = engine.encrypt("Hello", "additional_data");
    println!("Result: {:?}", result);
}
