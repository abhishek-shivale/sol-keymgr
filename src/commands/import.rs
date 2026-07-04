use crate::error::AppError;
use crate::vault::env::KeyAddr;
use crate::vault::index::Index;
use crate::vault::paths::Paths;
use crate::vault::prompt;
use crate::vault::store::{encode_keypair_json, store};
use crate::vault::verifier;
use solana_keypair::{read_keypair_file, Keypair, Signer};
use std::path::PathBuf;
use zeroize::Zeroizing;

pub fn run(file: Option<PathBuf>, key: bool, addr: Option<String>) -> Result<(), AppError> {
    let paths = Paths::resolve()?;
    paths.ensure_dirs()?;

    let addr = match addr {
        Some(s) => KeyAddr::parse(&s)?,
        None => prompt::ask_new_addr()?,
    };

    let keypair = if key {
        let raw = Zeroizing::new(rpassword::prompt_password("Private key (base58, e.g. Phantom export): ")?);
        Keypair::try_from_base58_string(raw.trim())
            .map_err(|e| AppError::InvalidPrivateKey(e.to_string()))?
    } else {
        let file = file.expect("clap enforces file or --key");
        read_keypair_file(&file).map_err(|e| AppError::InvalidKeypairFile(e.to_string()))?
    };
    let plaintext = encode_keypair_json(keypair.to_bytes())?;

    let pw = verifier::unlock(&paths)?;
    let mut index = Index::load(&paths)?;
    store(&paths, &mut index, &addr, &keypair.pubkey().to_string(), &plaintext, pw.as_bytes())?;

    println!("stored {} — pubkey: {}", addr, keypair.pubkey());
    Ok(())
}
