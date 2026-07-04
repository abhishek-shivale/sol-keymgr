use crate::constant::VERIFIER_PLAINTEXT;
use crate::encryption::{decrypt, encrypt, EncFile};
use crate::error::AppError;
use crate::vault::paths::Paths;
use zeroize::Zeroizing;

pub fn exists(paths: &Paths) -> bool {
    paths.verifier_file().exists()
}

pub fn create(passphrase: &[u8], paths: &Paths) -> Result<(), AppError> {
    let enc = encrypt(passphrase, VERIFIER_PLAINTEXT)?;
    let raw = serde_json::to_string_pretty(&enc)?;
    std::fs::write(paths.verifier_file(), raw)?;
    Ok(())
}

fn check(passphrase: &[u8], paths: &Paths) -> Result<(), AppError> {
    let raw = std::fs::read_to_string(paths.verifier_file())?;
    let enc: EncFile = serde_json::from_str(&raw)?;
    let plain = decrypt(passphrase, &enc)?;
    if plain.as_slice() == VERIFIER_PLAINTEXT {
        Ok(())
    } else {
        Err(AppError::Enc(crate::error::EncError::WrongPassphrase))
    }
}

/// Every passphrase-taking command calls this first (design §5, §8):
/// - verifier present -> just check it.
/// - verifier missing, vault empty -> first run, create it.
/// - verifier missing, vault non-empty -> heal: test-decrypt every entry; all-succeed
///   recreates the verifier, any failure names exactly which keys are sealed under a
///   different passphrase.
pub fn ensure_unlocked(passphrase: &[u8], paths: &Paths) -> Result<(), AppError> {
    if exists(paths) {
        return check(passphrase, paths);
    }

    let entries = vault_enc_files(paths)?;
    if entries.is_empty() {
        return create(passphrase, paths);
    }

    let mut mismatched = Vec::new();
    for path in &entries {
        let raw = std::fs::read_to_string(path)?;
        let enc: EncFile = serde_json::from_str(&raw)?;
        if decrypt(passphrase, &enc).is_err() {
            let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("?").to_string();
            mismatched.push(stem);
        }
    }

    if mismatched.is_empty() {
        create(passphrase, paths)
    } else {
        Err(AppError::Other(format!(
            "verifier missing and passphrase does not match: {} (mixed-passphrase vault, cannot heal)",
            mismatched.join(", ")
        )))
    }
}

fn vault_is_empty(paths: &Paths) -> Result<bool, AppError> {
    Ok(vault_enc_files(paths)?.is_empty())
}

/// Prompt for the master passphrase and unlock the vault (design §5 lifecycle):
/// - verifier exists, or vault has entries -> prompt once, verify/heal.
/// - truly first run (no verifier, empty vault) -> prompt twice to confirm, create verifier.
pub fn unlock(paths: &Paths) -> Result<Zeroizing<String>, AppError> {
    if exists(paths) || !vault_is_empty(paths)? {
        let pw = Zeroizing::new(rpassword::prompt_password("Master passphrase: ")?);
        ensure_unlocked(pw.as_bytes(), paths)?;
        Ok(pw)
    } else {
        let pw1 = Zeroizing::new(rpassword::prompt_password("Create master passphrase: ")?);
        let pw2 = Zeroizing::new(rpassword::prompt_password("Confirm master passphrase: ")?);
        if pw1.as_str() != pw2.as_str() {
            return Err(AppError::Other("passphrases did not match".into()));
        }
        create(pw1.as_bytes(), paths)?;
        Ok(pw1)
    }
}

fn vault_enc_files(paths: &Paths) -> Result<Vec<std::path::PathBuf>, AppError> {
    let dir = paths.vault_dir();
    if !dir.exists() {
        return Ok(Vec::new());
    }
    let mut out = Vec::new();
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        if path.extension().and_then(|e| e.to_str()) == Some("enc") {
            out.push(path);
        }
    }
    Ok(out)
}
