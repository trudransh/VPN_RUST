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
        // Validate input: check for empty message
        if message.is_empty() {
            return Err("Message cannot be empty");
        }
        
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
        // Validate input: ensure data has minimum length for nonce + ciphertext
        if data.len() < 24 {
            return Err("Invalid data: must be at least 24 bytes (nonce + ciphertext)");
        }
        
        let nonce = &data[..24];
        let ciphertext = &data[24..];
        
        // Additional validation: ensure there's actual ciphertext beyond the nonce
        if ciphertext.is_empty() {
            return Err("Invalid data: no ciphertext found after nonce");
        }
        
        let payload = Payload {
            msg: ciphertext,
            aad: AAD.as_bytes(),
        };
        let plaintext = self.cipher.decrypt(nonce.into(), payload)
            .map_err(|_| "Decryption failed: invalid ciphertext or wrong AAD")?;
        
        // Convert to UTF-8 with specific error message
        String::from_utf8(plaintext)
            .map_err(|_| "Decryption succeeded but result is not valid UTF-8")
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
    
    // Test 1: Normal Encryption/Decryption
    println!("1. Testing Normal Encryption/Decryption:");
    println!("   Message: \"{}\"", message);
    println!("   AAD: \"{}\"", aad);
    
    match engine.encrypt(message, aad) {
        Ok(encrypted_data) => {
            println!("Encryption successful!");
            println!("Encrypted data length: {} bytes", encrypted_data.len());
            println!("First 10 bytes: {:?}", &encrypted_data[..10.min(encrypted_data.len())]);
            
            // Test normal decryption
            match engine.decrypt(&encrypted_data, aad) {
                Ok(decrypted_message) => {
                    println!("Decryption successful!");
                    println!("Decrypted message: \"{}\"", decrypted_message);
                    println!("Match original: {}", decrypted_message == message);
                }
                Err(e) => {
                    println!("Unexpected decryption failure: {}", e);
                }
            }
            
            // Test 2: Error Handling - Empty Message
            println!("\n2. Testing Error Handling - Empty Message:");
            match engine.encrypt("", aad) {
                Ok(_) => println!("Unexpected success with empty message"),
                Err(e) => println!("Expected error: {}", e),
            }
            
            // Test 3: Error Handling - Invalid Data Length
            println!("\n3. Testing Error Handling - Invalid Data Length:");
            let short_data = vec![1, 2, 3]; // Only 3 bytes, need at least 24
            match engine.decrypt(&short_data, aad) {
                Ok(_) => println!("Unexpected success with short data"),
                Err(e) => println!("Expected error: {}", e),
            }
            
            // Test 4: Error Handling - Data with only nonce (no ciphertext)
            println!("\n4. Testing Error Handling - Nonce Only (No Ciphertext):");
            let nonce_only = vec![0u8; 24]; // Exactly 24 bytes (nonce only)
            match engine.decrypt(&nonce_only, aad) {
                Ok(_) => println!("Unexpected success with nonce-only data"),
                Err(e) => println!("Expected error: {}", e),
            }
            
            // Test 5: AAD Variations - Wrong AAD
            println!("\n5. Testing AAD Variations - Wrong AAD:");
            let wrong_aad = "wrong-auth";
            println!("Original AAD: \"{}\"", aad);
            println!("Testing with: \"{}\"", wrong_aad);
            match engine.decrypt(&encrypted_data, wrong_aad) {
                Ok(decrypted_message) => {
                    println!("Unexpected success: \"{}\"", decrypted_message);
                }
                Err(e) => {
                    println!("Expected authentication failure: {}", e);
                    println!("This confirms AAD authentication is working properly!");
                }
            }
            
            // Test 6: AAD Variations - Multiple Wrong AADs
            println!("\n6. Testing Multiple Wrong AAD Values:");
            let wrong_aads = vec!["", "vpn", "vpn-auth-wrong", "123", "VPN-AUTH"];
            for (i, test_aad) in wrong_aads.iter().enumerate() {
                println!("   Test {}: AAD = \"{}\"", i + 1, test_aad);
                match engine.decrypt(&encrypted_data, test_aad) {
                    Ok(_) => println!("Unexpected success!"),
                    Err(e) => println!("Failed as expected: {}", e),
                }
            }
            
            // Test 7: AAD Variations - Case Sensitivity
            println!("\n7. Testing AAD Case Sensitivity:");
            let case_variations = vec!["VPN-AUTH", "vpn-AUTH", "Vpn-Auth"];
            for case_aad in case_variations {
                println!("Testing AAD: \"{}\"", case_aad);
                match engine.decrypt(&encrypted_data, case_aad) {
                    Ok(_) => println!("Unexpected success - AAD should be case sensitive!"),
                    Err(e) => println!("Correctly rejected: {}", e),
                }
            }
            
        }
        Err(e) => {
            println!("Initial encryption failed: {}", e);
        }
    }
    
    // Test 8: Edge Cases - Different Message Lengths
    println!("\n8. Testing Different Message Lengths:");
    let test_messages = vec![
        ("a", "Single character"),
        ("Hello, VPN World! This is a longer message to test encryption.", "Long message"),
        ("🔒🔑💻", "Unicode characters"),
    ];
    
    for (test_msg, description) in test_messages {
        println!("   Testing {}: \"{}\"", description, test_msg);
        match engine.encrypt(test_msg, aad) {
            Ok(encrypted) => {
                match engine.decrypt(&encrypted, aad) {
                    Ok(decrypted) => {
                        if decrypted == test_msg {
                            println!("Success: matches original");
                        } else {
                            println!("Error: doesn't match original");
                        }
                    }
                    Err(e) => println!("Decryption failed: {}", e),
                }
            }
            Err(e) => println!("Encryption failed: {}", e),
        }
    }
    
    println!("\n=== All Tests Complete ===");
    println!("Summary: Tested encryption, decryption, error handling, and AAD authentication");
}
