# VPN Encryption Project

A Rust-based encryption implementation using XChaCha20Poly1305 AEAD cipher for secure VPN communications.

## Overview

This project implements modern authenticated encryption using the XChaCha20Poly1305 cipher, providing both confidentiality and authenticity for data transmission in VPN contexts.

## Cryptographic Concepts

### AEAD (Authenticated Encryption with Associated Data)
- **Purpose**: Provides both encryption (confidentiality) AND authentication (integrity)
- **Benefit**: Prevents tampering and ensures data hasn't been modified
- **Implementation**: XChaCha20Poly1305 cipher

### XChaCha20Poly1305 Cipher
- **Algorithm**: ChaCha20 (stream cipher) + Poly1305 (MAC)
- **Key Size**: 32 bytes (256 bits)
- **Nonce Size**: 24 bytes (192 bits) - extended nonce variant
- **Features**: 
  - Fast performance
  - Resistant to timing attacks
  - No special hardware requirements

### Nonce (Number Used Once)
- **Critical Rule**: Must be unique for each encryption with the same key
- **Size**: 24 bytes for XChaCha20Poly1305
- **Generation**: Cryptographically secure random number generator
- **Storage**: Typically prepended to ciphertext (not secret)

### AAD (Additional Authenticated Data)
- **Purpose**: Data that's authenticated but NOT encrypted
- **Use Cases**: Headers, metadata, routing information
- **Security**: Tamper detection without revealing content

## Current Implementation

### CryptoEngine Structure
```rust
struct CryptoEngine {
    cipher: XChaCha20Poly1305,  // AEAD cipher instance
}
```

### Key Features Implemented
- ✅ **Constructor**: Initialize with 32-byte key
- ✅ **Encryption**: AEAD encryption with message + AAD
- ✅ **Decryption**: AEAD decryption with authentication verification
- ✅ **Nonce Generation**: Cryptographically secure random nonces
- ✅ **Output Format**: Nonce + ciphertext combined for storage
- ✅ **Testing Suite**: Comprehensive encryption/decryption validation
- ✅ **AAD Authentication**: Tamper detection through AAD verification

### API Methods
- `CryptoEngine::new(key: &[u8; 32])` - Initialize with encryption key
- `encrypt(&self, message: &str, aad: &str)` - Encrypt with AAD support
- `decrypt(&self, data: &[u8], aad: &str)` - Decrypt and verify AAD authenticity

## Security Considerations

### Key Management
- Keys should be derived using proper key derivation functions
- Never hardcode keys in production
- Consider key rotation policies

### Nonce Handling
- **Critical**: Never reuse nonces with the same key
- Use cryptographically secure random generation
- Store nonces with ciphertext for decryption

### Output Format
- Current: `[nonce][ciphertext+tag]`
- Nonce is prepended for easy extraction during decryption

## Dependencies

```toml
[dependencies]
chacha20poly1305 = "0.10"  # AEAD cipher implementation
rand = "0.8"                # Secure random number generation
```

## Usage Example

```rust
// Initialize with encryption key
let key = [0u8; 32];  // Use proper key derivation in production
let engine = CryptoEngine::new(&key);

// Encrypt message with additional authenticated data
let message = "Hello, VPN!";
let aad = "vpn-auth";

match engine.encrypt(message, aad) {
    Ok(encrypted_data) => {
        // encrypted_data contains: [24-byte nonce][ciphertext+auth_tag]
        println!("Encrypted successfully: {} bytes", encrypted_data.len());
        
        // Decrypt the data
        match engine.decrypt(&encrypted_data, aad) {
            Ok(decrypted_message) => {
                println!("Decrypted: {}", decrypted_message);
                println!("Match original: {}", decrypted_message == message);
            }
            Err(e) => println!("Decryption failed: {}", e),
        }
    }
    Err(e) => println!("Encryption failed: {}", e),
}

// Test authentication with wrong AAD (will fail)
let wrong_aad = "wrong-auth";
match engine.decrypt(&encrypted_data, wrong_aad) {
    Ok(_) => println!("Unexpected success!"),
    Err(e) => println!("Expected failure: {}", e), // AAD authentication working
}
```

## Project Structure

```
src/
├── main.rs          # Core encryption implementation
└── ...              # Additional modules (to be added)
```

## Roadmap

### Planned Features
- [ ] Key derivation functions
- [ ] Error handling improvements
- [ ] Performance optimizations
- [ ] Network integration
- [ ] Configuration management
- [ ] Binary data support (beyond UTF-8 strings)

### Future Enhancements
- [ ] Multiple cipher support
- [ ] Key exchange protocols
- [ ] Streaming encryption
- [ ] Compression integration

## Testing

The project includes comprehensive testing that validates:

- **Encryption/Decryption Cycle**: Full round-trip testing
- **AAD Authentication**: Verification that wrong AAD fails decryption
- **Data Integrity**: Confirmation that decrypted data matches original
- **Error Handling**: Proper error reporting for authentication failures

Run tests with:
```bash
cargo run
```

## Development Notes

This README will be updated as new features are implemented. Each task completion will add relevant documentation sections.

**Latest Updates:**
- ✅ Added decryption functionality with AAD verification
- ✅ Implemented comprehensive testing suite
- ✅ Validated AEAD authentication mechanisms

---

**Security Warning**: This implementation is for educational/development purposes. Production use requires thorough security review and proper key management practices. 