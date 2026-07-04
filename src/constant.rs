pub const DEFAULT_PATH: &str = "~/.config/solana/id.json";

pub const SALT_LEN: usize = 16;
pub const NONCE_LEN: usize = 12;
pub const KEY_LEN: usize = 32;

pub const ARGON2_M: u32 = 65536;
pub const ARGON2_T: u32 = 3;
pub const ARGON2_P: u32 = 1;

pub const KDF_ALGO_ARGON2ID: &str = "argon2id";
pub const ENC_FILE_VERSION: u32 = 1;

pub const ERR_ENCRYPT_FAILED: &str = "encryption failed";
pub const ERR_BAD_SALT: &str = "bad salt";
pub const ERR_BAD_NONCE: &str = "bad nonce";
pub const ERR_BAD_CIPHERTEXT: &str = "bad ciphertext";
pub const ERR_BAD_NONCE_LEN: &str = "bad nonce length";
