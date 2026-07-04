use crate::error::AppError;
use crate::vault::env::KeyAddr;
use crate::vault::paths::Paths;
use crate::vault::store::{load_decrypt, write_0600};
use crate::vault::verifier;
use std::path::PathBuf;

pub fn run(addr: String, outfile: Option<PathBuf>) -> Result<(), AppError> {
    let paths = Paths::resolve()?;
    let addr = KeyAddr::parse(&addr)?;

    let pw = verifier::unlock(&paths)?;
    let plaintext = load_decrypt(&paths, &addr, pw.as_bytes())?;

    let outfile = outfile.unwrap_or_else(|| PathBuf::from(format!("{}.json", addr.stem())));
    write_0600(&outfile, &plaintext)?;

    eprintln!(
        "WARNING: {} now contains an UNENCRYPTED private key. Handle with care, delete when done.",
        outfile.display()
    );
    println!("exported {addr} -> {}", outfile.display());
    Ok(())
}
