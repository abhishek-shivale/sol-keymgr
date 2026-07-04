use crate::error::AppError;
use crate::vault::env::Env;
use crate::vault::index::Index;
use crate::vault::paths::Paths;
use std::str::FromStr;

pub fn run(env_filter: Option<String>, json: bool) -> Result<(), AppError> {
    let paths = Paths::resolve()?;
    let index = Index::load(&paths)?;
    let env_filter = env_filter.map(|s| Env::from_str(&s)).transpose()?;

    let keys: Vec<_> = index
        .keys
        .iter()
        .filter(|k| env_filter.map(|e| k.env == e.to_string()).unwrap_or(true))
        .collect();

    if json {
        println!("{}", serde_json::to_string_pretty(&keys)?);
        return Ok(());
    }

    if keys.is_empty() {
        println!("(empty)");
        return Ok(());
    }
    for k in keys {
        let active = paths.active_dir().join(format!("{}_{}.json", k.env, k.name)).exists();
        let tag = if active { " [active]" } else { "" };
        println!("{}/{}  {}  created {}{tag}", k.env, k.name, k.pubkey, k.created_at);
    }
    Ok(())
}
