use thiserror::Error;

#[derive(Debug, Error)]
pub enum EncError {
    #[error("wrong passphrase")]
    WrongPassphrase,
    #[error("corrupt vault entry: {0}")]
    Corrupt(String),
    #[error("crypto error: {0}")]
    Crypto(String),
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Enc(#[from] EncError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    TomlDe(#[from] toml::de::Error),

    #[error(transparent)]
    TomlSer(#[from] toml::ser::Error),

    #[error(transparent)]
    Bip39(#[from] bip39::Error),

    #[error("invalid name '{0}': must be a-z, 0-9, '-', 1-32 chars")]
    InvalidName(String),

    #[error("unknown env '{0}': expected one of local, dev, prod")]
    InvalidEnv(String),

    #[error("key '{0}' already exists")]
    DuplicateKey(String),

    #[error("key '{0}' not found in vault")]
    KeyNotFound(String),

    #[error("key '{0}' is not active")]
    NotActive(String),

    #[error("key '{0}' is currently active; kill it first")]
    StillActive(String),

    #[error("could not resolve home directory")]
    NoHomeDir,

    #[error("invalid keypair file: {0}")]
    InvalidKeypairFile(String),

    #[error("invalid private key: {0}")]
    InvalidPrivateKey(String),

    #[error("{0} exists and is not managed by keymgr")]
    ForeignSymlink(String),

    #[error("no .keymgr.toml found in this directory or any parent")]
    NoProjectBinding,

    #[error("{0}")]
    Other(String),
}
