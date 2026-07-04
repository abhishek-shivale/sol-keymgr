use crate::error::AppError;
use crate::vault::env::{Env, KeyAddr};
use crate::vault::paths::Paths;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyMeta {
    pub name: String,
    pub env: String,
    pub pubkey: String,
    pub created_at: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Index {
    #[serde(default)]
    pub keys: Vec<KeyMeta>,
}

impl Index {
    pub fn load(paths: &Paths) -> Result<Self, AppError> {
        let path = paths.index_file();
        if !path.exists() {
            return Ok(Index::default());
        }
        let raw = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&raw)?)
    }

    pub fn save(&self, paths: &Paths) -> Result<(), AppError> {
        let raw = serde_json::to_string_pretty(self)?;
        std::fs::write(paths.index_file(), raw)?;
        Ok(())
    }

    pub fn find(&self, addr: &KeyAddr) -> Option<&KeyMeta> {
        self.keys.iter().find(|k| k.env == addr.env.to_string() && k.name == addr.name)
    }

    pub fn contains(&self, addr: &KeyAddr) -> bool {
        self.find(addr).is_some()
    }

    pub fn insert(&mut self, addr: &KeyAddr, pubkey: String) {
        self.keys.push(KeyMeta {
            name: addr.name.clone(),
            env: addr.env.to_string(),
            pubkey,
            created_at: now_rfc3339(),
        });
    }

    pub fn remove(&mut self, addr: &KeyAddr) {
        self.keys.retain(|k| !(k.env == addr.env.to_string() && k.name == addr.name));
    }

    pub fn for_env(&self, env: Env) -> impl Iterator<Item = &KeyMeta> {
        let env = env.to_string();
        self.keys.iter().filter(move |k| k.env == env)
    }
}

fn now_rfc3339() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    secs.to_string()
}
