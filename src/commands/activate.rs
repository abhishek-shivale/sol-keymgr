use crate::error::AppError;
use crate::vault::env::KeyAddr;
use crate::vault::index::Index;
use crate::vault::paths::Paths;
use crate::vault::store::{load_decrypt, write_0600};
use crate::vault::verifier;
use dialoguer::Confirm;

pub fn run(addr: String) -> Result<(), AppError> {
    let paths = Paths::resolve()?;
    let addr = KeyAddr::parse(&addr)?;
    let index = Index::load(&paths)?;

    if !index.contains(&addr) {
        return Err(AppError::KeyNotFound(addr.to_string()));
    }

    let active_path = paths.active_entry(&addr);
    if active_path.exists() {
        let existing = std::fs::read(&active_path)?;
        write_0600(&active_path, &existing)?;
        println!("{addr} already active (refreshed)");
        return Ok(());
    }

    if addr.env.is_prod() {
        confirm_prod(&addr)?;
    }

    let pw = verifier::unlock(&paths)?;
    let plaintext = load_decrypt(&paths, &addr, pw.as_bytes())?;
    write_0600(&active_path, &plaintext)?;
    println!("activated {addr}");
    Ok(())
}

fn confirm_prod(addr: &KeyAddr) -> Result<(), AppError> {
    eprintln!("\x1b[1;31m[PROD]\x1b[0m activating {addr}");
    let ok = Confirm::new()
        .with_prompt("this is a prod key — continue?")
        .default(false)
        .interact()
        .map_err(|e| AppError::Other(e.to_string()))?;
    if !ok {
        return Err(AppError::Other("aborted".into()));
    }
    Ok(())
}
