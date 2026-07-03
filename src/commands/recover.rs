use bip39::Mnemonic;
use solana_keypair::{keypair_from_seed_phrase_and_passphrase, write_keypair_file, Keypair, Signer};
use std::error::Error;
use std::io::{self, Write};
use std::path::PathBuf;

pub fn run(outfile: PathBuf) -> Result<Keypair, Box<dyn Error>> {
    print!("Recovery seed phrase: ");
    io::stdout().flush()?;
    let mut phrase = String::new();
    io::stdin().read_line(&mut phrase)?;

    print!("Passphrase (optional, press enter to skip): ");
    io::stdout().flush()?;
    let mut passphrase = String::new();
    io::stdin().read_line(&mut passphrase)?;

    let keypair = recover_from_phrase(phrase.trim(), passphrase.trim())?;

    write_keypair_file(&keypair, &outfile)?;

    println!("Wrote recovered keypair to {}", outfile.display());
    println!("pubkey: {}", keypair.pubkey());

    Ok(keypair)
}

fn recover_from_phrase(phrase: &str, passphrase: &str) -> Result<Keypair, Box<dyn Error>> {
    Mnemonic::parse(phrase)?;
    keypair_from_seed_phrase_and_passphrase(phrase, passphrase)
}

#[cfg(test)]
mod test {
    use super::*;
    use bip39::Language;

    #[test]
    fn recovers_same_keypair_new_would_generate() {
        let mnemonic = Mnemonic::generate_in(Language::English, 12).unwrap();
        let seed = mnemonic.to_seed("");
        let expected = solana_keypair::keypair_from_seed(&seed).unwrap();

        let recovered = recover_from_phrase(&mnemonic.to_string(), "").unwrap();

        assert_eq!(expected.pubkey(), recovered.pubkey());
    }

    #[test]
    fn rejects_invalid_seed_phrase() {
        assert!(recover_from_phrase("not a real seed phrase at all", "").is_err());
    }
}