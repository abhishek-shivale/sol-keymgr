use crate::constant::PROJECT_TOML_NAME;
use crate::error::AppError;
use crate::vault::env::KeyAddr;
use std::path::{Path, PathBuf};

/// Walk up from `start` toward `/`; nearest `.keymgr.toml` wins (design §6).
pub fn find_binding(start: &Path) -> Result<Option<KeyAddr>, AppError> {
    let mut cur = Some(start);
    while let Some(dir) = cur {
        let toml_path = dir.join(PROJECT_TOML_NAME);
        if toml_path.exists() {
            let raw = std::fs::read_to_string(&toml_path)?;
            let table: toml::value::Table = toml::from_str(&raw)?;
            return Ok(match table.get("key") {
                Some(toml::Value::String(s)) => Some(KeyAddr::parse(s)?),
                _ => None,
            });
        }
        cur = dir.parent();
    }
    Ok(None)
}

/// Create/update `.keymgr.toml` in `dir`, touching only the `key` field (other
/// user-added fields are preserved). First creation inside a git repo appends
/// the file to `.gitignore` (idempotent).
pub fn bind(dir: &Path, addr: &KeyAddr) -> Result<(), AppError> {
    let toml_path = dir.join(PROJECT_TOML_NAME);
    let existed = toml_path.exists();

    let mut table: toml::value::Table = if existed {
        toml::from_str(&std::fs::read_to_string(&toml_path)?)?
    } else {
        toml::value::Table::new()
    };
    table.insert("key".into(), toml::Value::String(addr.to_string()));
    std::fs::write(&toml_path, toml::to_string_pretty(&table)?)?;

    if !existed {
        ensure_gitignored(dir)?;
    }
    Ok(())
}

fn find_git_root(dir: &Path) -> Option<PathBuf> {
    let mut cur = Some(dir);
    while let Some(d) = cur {
        if d.join(".git").exists() {
            return Some(d.to_path_buf());
        }
        cur = d.parent();
    }
    None
}

fn ensure_gitignored(dir: &Path) -> Result<(), AppError> {
    if find_git_root(dir).is_none() {
        return Ok(());
    }
    let gitignore = dir.join(".gitignore");
    let existing = if gitignore.exists() { std::fs::read_to_string(&gitignore)? } else { String::new() };
    if existing.lines().any(|l| l.trim() == PROJECT_TOML_NAME) {
        return Ok(());
    }
    let mut content = existing;
    if !content.is_empty() && !content.ends_with('\n') {
        content.push('\n');
    }
    content.push_str(PROJECT_TOML_NAME);
    content.push('\n');
    std::fs::write(&gitignore, content)?;
    Ok(())
}
