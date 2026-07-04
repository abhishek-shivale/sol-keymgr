use thiserror::Error;

#[derive(Debug, Error)]
pub enum EncError {
    #[error("wrong passphrase")]
    WrongPassphrase,
    #[error("corrupt vault entry: {0}")]
    Corrupt(String),
    #[error("crypto error: {0}")]
    Crypto(String)
}
