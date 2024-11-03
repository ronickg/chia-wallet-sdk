use bip39::{Language, Mnemonic};
use clvmr::sha2::Sha256;

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum GeneralError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

// Compare two byte arrays
#[uniffi::export]
pub fn compare_bytes(a: Vec<u8>, b: Vec<u8>) -> bool {
    a == b
}

// Calculate SHA-256 hash of the input bytes
#[uniffi::export]
pub fn sha256(bytes: Vec<u8>) -> Result<Vec<u8>, GeneralError> {
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    Ok(hasher.finalize().to_vec())
}

// Convert a hex string to raw bytes
#[uniffi::export]
pub fn from_hex_raw(hex: String) -> Result<Vec<u8>, GeneralError> {
    hex::decode(hex).map_err(|error| GeneralError::InvalidInput(error.to_string()))
}

// Convert a hex string (with or without "0x" prefix) to raw bytes
#[uniffi::export]
pub fn from_hex(hex: String) -> Result<Vec<u8>, GeneralError> {
    let hex_str = hex.strip_prefix("0x").unwrap_or(&hex);
    hex::decode(hex_str).map_err(|error| GeneralError::InvalidInput(error.to_string()))
}

// Convert bytes to a hex string
#[uniffi::export]
pub fn to_hex(bytes: Vec<u8>) -> String {
    hex::encode(&bytes)
}

// New BIP39 functions
#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum Bip39Error {
    #[error("Invalid word count: {0}")]
    BadWordCount(u32),
    #[error("Unknown word at index {0}")]
    UnknownWord(u32),
    #[error("Invalid entropy bit count: {0}")]
    BadEntropyBitCount(u32),
    #[error("Invalid checksum")]
    InvalidChecksum,
    #[error("Language ambiguity")]
    AmbiguousLanguages,
    #[error("Failed to generate mnemonic")]
    GenerationFailed,
    #[error("Failed to parse mnemonic")]
    ParseFailed,
}

impl From<bip39::Error> for Bip39Error {
    fn from(err: bip39::Error) -> Self {
        match err {
            bip39::Error::BadWordCount(c) => Self::BadWordCount(c as u32),
            bip39::Error::UnknownWord(i) => Self::UnknownWord(i as u32),
            bip39::Error::BadEntropyBitCount(c) => Self::BadEntropyBitCount(c as u32),
            bip39::Error::InvalidChecksum => Self::InvalidChecksum,
            bip39::Error::AmbiguousLanguages(_) => Self::AmbiguousLanguages,
        }
    }
}

// Generate a mnemonic phrase with the given word count
#[uniffi::export]
pub fn generate_mnemonic(word_count: u32) -> Result<String, Bip39Error> {
    Mnemonic::generate_in(Language::English, word_count as usize)
        .map(|m| m.to_string())
        .map_err(|e| Bip39Error::from(e))
}

// Create a mnemonic from entropy bytes
#[uniffi::export]
pub fn mnemonic_from_entropy(entropy: Vec<u8>) -> Result<String, Bip39Error> {
    Mnemonic::from_entropy(&entropy)
        .map(|m| m.to_string())
        .map_err(|e| Bip39Error::from(e))
}

// Validate a mnemonic phrase
#[uniffi::export]
pub fn validate_mnemonic(phrase: String) -> Result<bool, Bip39Error> {
    match Mnemonic::parse_normalized(&phrase) {
        Ok(_) => Ok(true),
        Err(e) => match e {
            bip39::Error::InvalidChecksum => Ok(false),
            bip39::Error::BadWordCount(_) => Ok(false),
            bip39::Error::UnknownWord(_) => Ok(false),
            e => Err(Bip39Error::from(e)),
        },
    }
}

// Convert a mnemonic phrase to a seed with an optional passphrase
#[uniffi::export]
pub fn mnemonic_to_seed(phrase: String, passphrase: Option<String>) -> Result<Vec<u8>, Bip39Error> {
    let mnemonic = Mnemonic::parse_normalized(&phrase).map_err(|e| Bip39Error::from(e))?;

    Ok(match passphrase {
        Some(pass) => mnemonic.to_seed_normalized(&pass).to_vec(),
        None => mnemonic.to_seed_normalized("").to_vec(),
    })
}

// Convert a mnemonic phrase to entropy bytes
#[uniffi::export]
pub fn mnemonic_to_entropy(phrase: String) -> Result<Vec<u8>, Bip39Error> {
    let mnemonic = Mnemonic::parse_normalized(&phrase).map_err(|e| Bip39Error::from(e))?;

    let (entropy_array, length) = mnemonic.to_entropy_array();
    Ok(entropy_array[..length].to_vec())
}
