use chacha20poly1305::{
    aead::{Aead, KeyInit, Nonce},
    XChaCha20Poly1305,
    Key,
};


struct CryptoEngine {
    cipher: XChaCha20Poly1305,
}

impl CryptoEngine {
    fn new(_key : &[u8;32]) -> Self {
            let cipher = XChaCha20Poly1305::new(Key::from_slice(&[0; 32]));
            Self { cipher }
    }
}


fn main() {
    println!("Hello, world!");
}
