use crate::error::AppError;
use crate::vault::binding;
use crate::vault::env::KeyAddr;
use crate::vault::index::Index;
use crate::vault::paths::Paths;
use crate::vault::store::{encode_keypair_json, store};
use crate::vault::{prompt, verifier};
use bip39::{Language, Mnemonic};
use dialoguer::Confirm;
use solana_keypair::{keypair_from_seed, Signer};

pub fn run(addr: Option<String>, no_toml: bool) -> Result<(), AppError> {
    let paths = Paths::resolve()?;
    paths.ensure_dirs()?;

    let addr = match addr {
        Some(s) => KeyAddr::parse(&s)?,
        None => prompt::ask_new_addr()?,
    };

    let mnemonic = Mnemonic::generate_in(Language::English, 12)?;
    let seed = mnemonic.to_seed("");
    let keypair = keypair_from_seed(&seed).map_err(|e| AppError::Other(e.to_string()))?;
    let plaintext = encode_keypair_json(keypair.to_bytes())?;

    let pw = verifier::unlock(&paths)?;
    let mut index = Index::load(&paths)?;
    store(&paths, &mut index, &addr, &keypair.pubkey().to_string(), &plaintext, pw.as_bytes())?;

    println!("stored {} — pubkey: {}", addr, keypair.pubkey());
    println!("recovery seed phrase (write this down, it is shown only once):\n{mnemonic}");

    if !no_toml {
        maybe_bind(&addr)?;
    }
    Ok(())
}

fn maybe_bind(addr: &KeyAddr) -> Result<(), AppError> {
    let cwd = std::env::current_dir()?;
    if binding::find_binding(&cwd)?.is_some() {
        return Ok(());
    }
    let bind = Confirm::new()
        .with_prompt(format!("bind this project to {addr}?"))
        .default(false)
        .interact()
        .map_err(|e| AppError::Other(e.to_string()))?;
    if bind {
        binding::bind(&cwd, addr)?;
        println!("wrote .keymgr.toml");
    }
    Ok(())
}
