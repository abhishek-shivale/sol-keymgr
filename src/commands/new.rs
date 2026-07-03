use bip39::{Language, Mnemonic};
use solana_keypair::{keypair_from_seed, write_keypair_file, Keypair, Signer};
use std::error::Error;
use std::path::PathBuf;

pub fn run(outfile: PathBuf) -> Result<Keypair, Box<dyn Error>> {
    let mnemonic = Mnemonic::generate_in(Language::English, 12)?;
    let seed = mnemonic.to_seed("");
    let keypair = keypair_from_seed(&seed)?;

    write_keypair_file(&keypair, &outfile)?;

    println!("Wrote new keypair to {}", outfile.display());
    println!("pubkey: {}", keypair.pubkey());
    println!(
        "Save this seed phrase to recover your new keypair:\n{}",
        mnemonic
    );

    Ok(keypair)
}

#[cfg(test)]
mod test {
    use super::*;
    use solana_keypair::read_keypair_file;

    #[test]
    fn test_new_pubkey() {
        let path = std::env::temp_dir().join("keymgr_test_new_keypair.json");

        let keypair = run(path.clone()).unwrap();
        let read_back = read_keypair_file(&path).unwrap();

        assert_eq!(keypair.pubkey(), read_back.pubkey());
        std::fs::remove_file(&path).unwrap();
    }
}
