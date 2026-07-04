use crate::encryption::{decrypt, encrypt, EncFile};
use crate::error::AppError;
use crate::vault::env::KeyAddr;
use crate::vault::index::Index;
use crate::vault::paths::Paths;
use std::fs::OpenOptions;
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;
use zeroize::Zeroizing;

/// Write `bytes` to `path`, created 0600 before any bytes land (never write-then-chmod).
pub fn write_0600(path: &Path, bytes: &[u8]) -> Result<(), AppError> {
    let mut f = OpenOptions::new().write(true).create(true).truncate(true).mode(0o600).open(path)?;
    f.write_all(bytes)?;
    Ok(())
}

/// solana-keygen format: JSON array of the 64 keypair bytes (§12).
pub fn encode_keypair_json(bytes: [u8; 64]) -> Result<Vec<u8>, AppError> {
    Ok(serde_json::to_vec(&bytes.to_vec())?)
}

/// Encrypt `plaintext` under `pw` and add it to the vault + index. Refuses duplicates.
pub fn store(
    paths: &Paths,
    index: &mut Index,
    addr: &KeyAddr,
    pubkey: &str,
    plaintext: &[u8],
    pw: &[u8],
) -> Result<(), AppError> {
    if index.contains(addr) {
        return Err(AppError::DuplicateKey(addr.to_string()));
    }
    let enc = encrypt(pw, plaintext)?;
    std::fs::write(paths.vault_entry(addr), serde_json::to_string_pretty(&enc)?)?;
    index.insert(addr, pubkey.to_string());
    index.save(paths)?;
    Ok(())
}

/// Decrypt a vault entry's plaintext keypair JSON.
pub fn load_decrypt(paths: &Paths, addr: &KeyAddr, pw: &[u8]) -> Result<Zeroizing<Vec<u8>>, AppError> {
    let path = paths.vault_entry(addr);
    if !path.exists() {
        return Err(AppError::KeyNotFound(addr.to_string()));
    }
    let raw = std::fs::read_to_string(path)?;
    let enc: EncFile = serde_json::from_str(&raw)?;
    Ok(decrypt(pw, &enc)?)
}

/// Remove a key from the vault + index.
pub fn delete(paths: &Paths, index: &mut Index, addr: &KeyAddr) -> Result<(), AppError> {
    if !index.contains(addr) {
        return Err(AppError::KeyNotFound(addr.to_string()));
    }
    std::fs::remove_file(paths.vault_entry(addr))?;
    index.remove(addr);
    index.save(paths)?;
    Ok(())
}
