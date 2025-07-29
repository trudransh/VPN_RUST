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
    fn decrypt (&self, data : &[u8], AAD : &str) -> Result<String, &'static str>{
        let nonce = &data[..24];
        let ciphertext = &data[24..];
        let payload = Payload {
            msg: ciphertext,
            aad: AAD.as_bytes(),
        };
        let plaintext = self.cipher.decrypt(nonce.into(), payload)  // Fix: use nonce.into()
            .map_err(|_| "Decryption failed")?;
        Ok(String::from_utf8(plaintext).map_err(|_| "Invalid UTF-8")?)
    }
}

fn generate_nonce() -> [u8;24] {
    let mut nonce = [0;24];
    OsRng.fill_bytes(&mut nonce);
    nonce
}




fn main() {
    println!("=== VPN Encryption Testing ===\n");
    
    // Create a placeholder key (in production, use proper key derivation)
    let key = [0u8; 32];
    let engine = CryptoEngine::new(&key);
    
    // Test data
    let message = "Hello, VPN!";
    let aad = "vpn-auth";
    
    // Test 1: Encryption
    println!("1. Testing Encryption:");
    println!("   Message: \"{}\"", message);
    println!("   AAD: \"{}\"", aad);
    
    match engine.encrypt(message, aad) {
        Ok(encrypted_data) => {
            println!("Encryption successful!");
            println!("Encrypted data length: {} bytes", encrypted_data.len());
            println!("First 10 bytes: {:?}", &encrypted_data[..10.min(encrypted_data.len())]);
            
            // Test 2: Decryption with correct AAD
            println!("\n2. Testing Decryption (correct AAD):");
            match engine.decrypt(&encrypted_data, aad) {
                Ok(decrypted_message) => {
                    println!("Decryption successful!");
                    println!("Decrypted message: \"{}\"", decrypted_message);
                    println!("Match original: {}", decrypted_message == message);
                }
                Err(e) => {
                    println!("Decryption failed: {}", e);
                }
            }
            
            // Test 3: Decryption with invalid AAD (should fail)
            println!("\n3. Testing Decryption (invalid AAD):");
            let invalid_aad = "wrong-auth";
            println!("   Using invalid AAD: \"{}\"", invalid_aad);
            match engine.decrypt(&encrypted_data, invalid_aad) {
                Ok(decrypted_message) => {
                    println!("Unexpected success: \"{}\"", decrypted_message);
                }
                Err(e) => {
                    println!("Expected failure: {}", e);
                    println!("This confirms AAD authentication is working!");
                }
            }
            
        }
        Err(e) => {
            println!("Encryption failed: {}", e);
        }
    }
    
    println!("\n=== Testing Complete ===");
}
