use argon2::{Algorithm, Argon2, Params, Version};
use base64ct::{Base64, Encoding};
use chacha20poly1305::aead::{Aead, KeyInit};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use rand::Rng;
use serde::{Deserialize, Serialize};
use zeroize::Zeroizing;
use crate::{constant::*, error::EncError};

#[derive(Serialize, Deserialize)]
struct KdfParams {
    algo: String,
    m: u32,
    t: u32,
    p: u32,
}

#[derive(Serialize, Deserialize)]
pub struct EncFile {
    version: u32,
    kdf: KdfParams,
    salt: String,
    nonce: String,
    ciphertext: String,
}

fn derive_key(passphrase: &[u8], salt: &[u8]) -> Result<Zeroizing<[u8; KEY_LEN]>, EncError> {
    let params = Params::new(ARGON2_M, ARGON2_T, ARGON2_P, Some(KEY_LEN))
        .map_err(|e| EncError::Crypto(e.to_string()))?;
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    let mut key = Zeroizing::new([0u8; KEY_LEN]);
    argon2
        .hash_password_into(passphrase, salt, key.as_mut())
        .map_err(|e| EncError::Crypto(e.to_string()))?;
    Ok(key)
}

pub fn encrypt(passphrase: &[u8], plaintext: &[u8]) -> Result<EncFile, EncError> {
    let mut salt = [0u8; SALT_LEN];
    let mut nonce_bytes = [0u8; NONCE_LEN];
    rand::rng().fill_bytes(&mut salt);
    rand::rng().fill_bytes(&mut nonce_bytes);

    let key = derive_key(passphrase, &salt)?;
    let cipher = ChaCha20Poly1305::new(&Key::from(*key));
    let ciphertext = cipher
        .encrypt(&Nonce::from(nonce_bytes), plaintext)
        .map_err(|_| EncError::Crypto(ERR_ENCRYPT_FAILED.into()))?;

    Ok(EncFile {
        version: ENC_FILE_VERSION,
        kdf: KdfParams {
            algo: KDF_ALGO_ARGON2ID.into(),
            m: ARGON2_M,
            t: ARGON2_T,
            p: ARGON2_P,
        },
        salt: Base64::encode_string(&salt),
        nonce: Base64::encode_string(&nonce_bytes),
        ciphertext: Base64::encode_string(&ciphertext),
    })
}

pub fn decrypt(passphrase: &[u8], file: &EncFile) -> Result<Zeroizing<Vec<u8>>, EncError> {
    let salt =
        Base64::decode_vec(&file.salt).map_err(|_| EncError::Corrupt(ERR_BAD_SALT.into()))?;
    let nonce_bytes =
        Base64::decode_vec(&file.nonce).map_err(|_| EncError::Corrupt(ERR_BAD_NONCE.into()))?;
    let ciphertext = Base64::decode_vec(&file.ciphertext)
        .map_err(|_| EncError::Corrupt(ERR_BAD_CIPHERTEXT.into()))?;

    let nonce_bytes: [u8; NONCE_LEN] = nonce_bytes
        .try_into()
        .map_err(|_| EncError::Corrupt(ERR_BAD_NONCE_LEN.into()))?;

    let key = derive_key(passphrase, &salt)?;
    let cipher = ChaCha20Poly1305::new(&Key::from(*key));

    cipher
        .decrypt(&Nonce::from(nonce_bytes), ciphertext.as_ref())
        .map(Zeroizing::new)
        .map_err(|_| EncError::WrongPassphrase)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let enc = encrypt(b"correct horse", b"top secret bytes").unwrap();
        let out = decrypt(b"correct horse", &enc).unwrap();
        assert_eq!(out.as_slice(), b"top secret bytes");
    }

    #[test]
    fn wrong_passphrase_fails_clean() {
        let enc = encrypt(b"correct horse", b"top secret bytes").unwrap();
        let err = decrypt(b"wrong horse", &enc).unwrap_err();
        assert!(matches!(err, EncError::WrongPassphrase));
    }

    #[test]
    fn tampered_ciphertext_fails_clean() {
        let mut enc = encrypt(b"correct horse", b"top secret bytes").unwrap();
        enc.ciphertext = Base64::encode_string(b"not the real ciphertext bytes!!");
        let err = decrypt(b"correct horse", &enc).unwrap_err();
        assert!(matches!(err, EncError::WrongPassphrase));
    }

    #[test]
    fn salt_and_nonce_are_fresh_each_call() {
        let a = encrypt(b"pw", b"data").unwrap();
        let b = encrypt(b"pw", b"data").unwrap();
        assert_ne!(a.salt, b.salt);
        assert_ne!(a.nonce, b.nonce);
    }
}
