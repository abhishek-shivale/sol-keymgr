use crate::constant::ENVS;
use crate::error::AppError;
use crate::vault::env::{validate_name, Env, KeyAddr};
use crate::vault::index::{Index, KeyMeta};
use crate::vault::paths::Paths;
use dialoguer::{Input, Select};
use std::str::FromStr;

pub fn ask_new_addr() -> Result<KeyAddr, AppError> {
    let env_idx = Select::new()
        .with_prompt("env")
        .items(&ENVS)
        .default(0)
        .interact()
        .map_err(|e| AppError::Other(e.to_string()))?;
    let env = Env::from_str(ENVS[env_idx])?;

    let name: String = Input::new()
        .with_prompt("name")
        .validate_with(|s: &String| validate_name(s).map_err(|e| e.to_string()))
        .interact_text()
        .map_err(|e| AppError::Other(e.to_string()))?;

    Ok(KeyAddr { env, name })
}

pub fn pick_from_index(index: &Index, prompt: &str) -> Result<KeyAddr, AppError> {
    if index.keys.is_empty() {
        return Err(AppError::Other("vault is empty".into()));
    }
    let labels: Vec<String> = index.keys.iter().map(label).collect();
    let choice = Select::new()
        .with_prompt(prompt)
        .items(&labels)
        .default(0)
        .interact()
        .map_err(|e| AppError::Other(e.to_string()))?;
    let meta = &index.keys[choice];
    Ok(KeyAddr { env: Env::from_str(&meta.env)?, name: meta.name.clone() })
}

pub fn pick_active(paths: &Paths, index: &Index, prompt: &str) -> Result<KeyAddr, AppError> {
    let actives: Vec<&KeyMeta> = index
        .keys
        .iter()
        .filter(|k| {
            let addr = KeyAddr { env: Env::from_str(&k.env).unwrap(), name: k.name.clone() };
            paths.active_entry(&addr).exists()
        })
        .collect();
    if actives.is_empty() {
        return Err(AppError::Other("no active keys".into()));
    }
    let labels: Vec<String> = actives.iter().map(|k| label(k)).collect();
    let choice = Select::new()
        .with_prompt(prompt)
        .items(&labels)
        .default(0)
        .interact()
        .map_err(|e| AppError::Other(e.to_string()))?;
    let meta = actives[choice];
    Ok(KeyAddr { env: Env::from_str(&meta.env)?, name: meta.name.clone() })
}

fn label(k: &KeyMeta) -> String {
    format!("{}/{} ({})", k.env, k.name, k.pubkey)
}
