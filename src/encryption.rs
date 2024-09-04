use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use aes_gcm::AeadCore;
use base64::{engine::general_purpose, Engine as _};
use rand::RngCore;

#[derive(Debug)]
struct AesGcmError(aes_gcm::Error);

impl std::fmt::Display for AesGcmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AES-GCM error: {:?}", self.0)
    }
}

impl std::error::Error for AesGcmError {}

pub fn generate_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);
    key
}

pub fn encrypt(plaintext: &str, key: &[u8; 32]) -> Result<String, Box<dyn std::error::Error>> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher.encrypt(&nonce, plaintext.as_bytes().as_ref())
        .map_err(|e| Box::new(AesGcmError(e)) as Box<dyn std::error::Error>)?;
    let mut combined = nonce.to_vec();
    combined.extend_from_slice(&ciphertext);
    Ok(general_purpose::URL_SAFE_NO_PAD.encode(combined))
}

pub fn decrypt(ciphertext: &str, key: &[u8; 32]) -> Result<String, Box<dyn std::error::Error>> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let decoded = general_purpose::URL_SAFE_NO_PAD.decode(ciphertext)?;
    let nonce = Nonce::from_slice(&decoded[..12]);
    let plaintext = cipher.decrypt(nonce, decoded[12..].as_ref())
        .map_err(|e| Box::new(AesGcmError(e)) as Box<dyn std::error::Error>)?;
    Ok(String::from_utf8(plaintext)?)
}