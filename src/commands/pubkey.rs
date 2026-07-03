use solana_keypair::{read_keypair_file, Keypair, Signer};
use std::error::Error;
use std::path::PathBuf;

pub fn run(keypair: PathBuf) -> Result<Keypair, Box<dyn Error>> {
    let keypair = read_keypair_file(&keypair)?;
    println!("{}", keypair.pubkey());
    Ok(keypair)
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_keypair::{write_keypair_file, Keypair};

    #[test]
    fn reads_back_the_pubkey_it_wrote() {
        let path = std::env::temp_dir().join("keymgr_test_keypair.json");
        let keypair = Keypair::new();
        write_keypair_file(&keypair, &path).unwrap();

        let read_back = run(path.clone()).unwrap();

        assert_eq!(keypair.pubkey(), read_back.pubkey());
        std::fs::remove_file(&path).unwrap();
    }
}
