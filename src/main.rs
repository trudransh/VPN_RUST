use chacha20poly1305::{
    aead::{Aead, KeyInit, Nonce},
    XChaCha20Poly1305,
    Key,
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
}
