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

// --- vault layout (~/.sol-keymgr) ---
pub const APP_DIR_NAME: &str = ".sol-keymgr";
pub const VAULT_DIR_NAME: &str = "vault";
pub const ACTIVE_DIR_NAME: &str = "active";
pub const BACKUP_DIR_NAME: &str = "backup";
pub const INDEX_FILE_NAME: &str = "index.json";
pub const VERIFIER_FILE_NAME: &str = "verifier.enc";
pub const CONFIG_FILE_NAME: &str = "config.toml";
pub const SOLANA_ID_BACKUP_NAME: &str = "id.json.bak";

// --- verifier canary ---
pub const VERIFIER_PLAINTEXT: &[u8] = b"sol-keymgr-v1";

// --- key naming ---
pub const ENVS: [&str; 3] = ["local", "dev", "prod"];
pub const NAME_MAX_LEN: usize = 32;

// --- project binding ---
pub const PROJECT_TOML_NAME: &str = ".keymgr.toml";
