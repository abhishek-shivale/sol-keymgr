use crate::constant::{
    ACTIVE_DIR_NAME, APP_DIR_NAME, BACKUP_DIR_NAME, CONFIG_FILE_NAME, INDEX_FILE_NAME,
    SOLANA_ID_BACKUP_NAME, VAULT_DIR_NAME, VERIFIER_FILE_NAME,
};
use crate::error::AppError;
use crate::vault::env::KeyAddr;
use directories::BaseDirs;
use std::path::PathBuf;

pub struct Paths {
    root: PathBuf,
}

impl Paths {
    pub fn resolve() -> Result<Self, AppError> {
        let home = BaseDirs::new().ok_or(AppError::NoHomeDir)?.home_dir().to_path_buf();
        Ok(Paths { root: home.join(APP_DIR_NAME) })
    }

    pub fn root(&self) -> &PathBuf {
        &self.root
    }

    pub fn vault_dir(&self) -> PathBuf {
        self.root.join(VAULT_DIR_NAME)
    }

    pub fn active_dir(&self) -> PathBuf {
        self.root.join(ACTIVE_DIR_NAME)
    }

    pub fn backup_dir(&self) -> PathBuf {
        self.root.join(BACKUP_DIR_NAME)
    }

    pub fn index_file(&self) -> PathBuf {
        self.root.join(INDEX_FILE_NAME)
    }

    pub fn verifier_file(&self) -> PathBuf {
        self.root.join(VERIFIER_FILE_NAME)
    }

    pub fn config_file(&self) -> PathBuf {
        self.root.join(CONFIG_FILE_NAME)
    }

    pub fn id_backup_file(&self) -> PathBuf {
        self.backup_dir().join(SOLANA_ID_BACKUP_NAME)
    }

    pub fn vault_entry(&self, addr: &KeyAddr) -> PathBuf {
        self.vault_dir().join(format!("{}.enc", addr.stem()))
    }

    pub fn active_entry(&self, addr: &KeyAddr) -> PathBuf {
        self.active_dir().join(format!("{}.json", addr.stem()))
    }

    /// Create vault/, active/, backup/ if missing. Idempotent.
    pub fn ensure_dirs(&self) -> Result<(), AppError> {
        std::fs::create_dir_all(self.vault_dir())?;
        std::fs::create_dir_all(self.active_dir())?;
        std::fs::create_dir_all(self.backup_dir())?;
        Ok(())
    }
}
