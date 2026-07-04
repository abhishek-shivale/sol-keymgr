use crate::error::AppError;
use crate::vault::env::KeyAddr;
use crate::vault::index::Index;
use crate::vault::paths::Paths;
use crate::vault::prompt;
use crate::vault::store::{encode_keypair_json, store};
use crate::vault::verifier;
use solana_keypair::{read_keypair_file, Signer};
use std::path::PathBuf;

pub fn run(file: PathBuf, addr: Option<String>) -> Result<(), AppError> {
    let paths = Paths::resolve()?;
    paths.ensure_dirs()?;

    let addr = match addr {
        Some(s) => KeyAddr::parse(&s)?,
        None => prompt::ask_new_addr()?,
    };

    let keypair =
        read_keypair_file(&file).map_err(|e| AppError::InvalidKeypairFile(e.to_string()))?;
    let plaintext = encode_keypair_json(keypair.to_bytes())?;

    let pw = verifier::unlock(&paths)?;
    let mut index = Index::load(&paths)?;
    store(&paths, &mut index, &addr, &keypair.pubkey().to_string(), &plaintext, pw.as_bytes())?;

    println!("stored {} — pubkey: {}", addr, keypair.pubkey());
    Ok(())
}
