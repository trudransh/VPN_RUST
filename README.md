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
- ✅ **Error Handling**: Comprehensive input validation and specific error messages
- ✅ **Testing Suite**: Extensive encryption/decryption validation with edge cases
- ✅ **AAD Authentication**: Tamper detection through AAD verification
- ✅ **AAD Variations Testing**: Multiple wrong AAD scenarios and case sensitivity

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

// Test error handling
match engine.encrypt("", aad) {  // Empty message
    Ok(_) => println!("Unexpected success!"),
    Err(e) => println!("Expected error: {}", e),
}

// Test AAD authentication with wrong AAD (will fail)
let wrong_aad = "wrong-auth";
match engine.decrypt(&encrypted_data, wrong_aad) {
    Ok(_) => println!("Unexpected success!"),
    Err(e) => println!("Expected failure: {}", e), // AAD authentication working
}

// Test invalid data length
let short_data = vec![1, 2, 3];  // Too short
match engine.decrypt(&short_data, aad) {
    Ok(_) => println!("Unexpected success!"),
    Err(e) => println!("Expected error: {}", e),
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
- [ ] Key derivation functions (PBKDF2, Argon2)
- [ ] Performance optimizations and benchmarking
- [ ] Network integration for VPN protocols
- [ ] Configuration management and settings
- [ ] Binary data support (beyond UTF-8 strings)
- [ ] Streaming encryption for large files
- [ ] Key rotation and management

### Future Enhancements
- [ ] Multiple cipher support
- [ ] Key exchange protocols
- [ ] Streaming encryption
- [ ] Compression integration

## Testing

The project includes comprehensive testing that validates:

### Core Functionality
- **Encryption/Decryption Cycle**: Full round-trip testing with various message lengths
- **Data Integrity**: Confirmation that decrypted data matches original
- **Unicode Support**: Testing with emoji and special characters

### Error Handling Validation
- **Empty Message Rejection**: Prevents encryption of empty strings
- **Invalid Data Length**: Catches data shorter than minimum required (24+ bytes)
- **Malformed Data**: Detects nonce-only data without ciphertext
- **UTF-8 Validation**: Specific error messages for invalid UTF-8 results

### AAD Authentication Testing
- **Correct AAD**: Successful decryption with matching AAD
- **Wrong AAD Values**: Multiple variations of incorrect AAD strings
- **Case Sensitivity**: Verification that AAD is case-sensitive
- **Empty AAD**: Testing with empty and partial AAD strings
- **Authentication Failure**: Proper error reporting when AAD doesn't match

### Edge Cases
- **Message Length Variations**: Single characters, long messages, unicode
- **Multiple AAD Scenarios**: Systematic testing of different wrong AAD values
- **Data Corruption Simulation**: Testing with various malformed input data

Run comprehensive tests with:
```bash
cargo run
```

**Expected Output**: The test suite will show ✅ for expected successes and ✅ for expected failures, demonstrating robust error handling.

## Development Notes

This README will be updated as new features are implemented. Each task completion will add relevant documentation sections.

**Latest Updates:**
- ✅ Added decryption functionality with AAD verification
- ✅ Implemented comprehensive testing suite with 8 test categories
- ✅ Added robust error handling with specific error messages
- ✅ Validated AEAD authentication mechanisms
- ✅ Extensive AAD variation testing including case sensitivity
- ✅ Edge case testing with unicode, empty messages, and malformed data

---

**Security Warning**: This implementation is for educational/development purposes. Production use requires thorough security review and proper key management practices. 