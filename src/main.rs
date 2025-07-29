use chacha20poly1305::{
    aead::{Aead, KeyInit, Nonce},
    XChaCha20Poly1305,
};
use rand::{rngs::OsRng, RngCore};

// use rand::{rngs::OsRng, RngCore};

struct CryptoEngine {
    cipher: XChaCha20Poly1305,
}


struct EncryptionKey {
    key: [u8; 32],
}



fn main() {
    println!("Hello, world!");
}
