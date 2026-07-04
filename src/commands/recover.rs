use crate::error::AppError;
use crate::vault::env::KeyAddr;
use crate::vault::index::Index;
use crate::vault::paths::Paths;
use crate::vault::prompt;
use crate::vault::store::{encode_keypair_json, store};
use crate::vault::verifier;
use bip39::Mnemonic;
use rpassword::prompt_password;
use solana_keypair::{keypair_from_seed_phrase_and_passphrase, Signer};

pub fn run(addr: Option<String>) -> Result<(), AppError> {
    let paths = Paths::resolve()?;
    paths.ensure_dirs()?;

    let addr = match addr {
        Some(s) => KeyAddr::parse(&s)?,
        None => prompt::ask_new_addr()?,
    };

    let phrase = prompt_password("Recovery seed phrase: ")?;
    let bip39_passphrase = prompt_password("BIP39 passphrase (optional, enter to skip): ")?;
    let keypair = recover_from_phrase(phrase.trim(), bip39_passphrase.trim())?;
    let plaintext = encode_keypair_json(keypair.to_bytes())?;

    let pw = verifier::unlock(&paths)?;
    let mut index = Index::load(&paths)?;
    store(&paths, &mut index, &addr, &keypair.pubkey().to_string(), &plaintext, pw.as_bytes())?;

    println!("stored {} — pubkey: {}", addr, keypair.pubkey());
    Ok(())
}

fn recover_from_phrase(
    phrase: &str,
    bip39_passphrase: &str,
) -> Result<solana_keypair::Keypair, AppError> {
    Mnemonic::parse(phrase)?;
    keypair_from_seed_phrase_and_passphrase(phrase, bip39_passphrase)
        .map_err(|e| AppError::Other(e.to_string()))
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
