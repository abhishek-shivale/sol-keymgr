use crate::error::AppError;
use crate::vault::env::KeyAddr;
use crate::vault::paths::Paths;
use directories::BaseDirs;
use std::path::PathBuf;

const DEFAULT_SOLANA_CONFIG: &str = "json_rpc_url: \"https://api.mainnet-beta.solana.com\"\nwebsocket_url: \"\"\naddress_labels:\n  \"11111111111111111111111111111111\": System Program\ncommitment: confirmed\n";

pub fn run(addr: String) -> Result<(), AppError> {
    let paths = Paths::resolve()?;
    let addr = KeyAddr::parse(&addr)?;
    let active_path = paths.active_entry(&addr);

    if !active_path.exists() {
        return Err(AppError::NotActive(format!(
            "{addr} (run: keymgr activate {addr})"
        )));
    }

    let cfg_path = per_project_solana_config(&paths, &addr)?;
    println!("export ANCHOR_WALLET={}", active_path.display());
    println!("export SOLANA_CONFIG_FILE={}", cfg_path.display());
    Ok(())
}

fn per_project_solana_config(paths: &Paths, addr: &KeyAddr) -> Result<PathBuf, AppError> {
    let default_cfg_path = BaseDirs::new()
        .ok_or(AppError::NoHomeDir)?
        .home_dir()
        .join(".config")
        .join("solana")
        .join("cli")
        .join("config.yml");
    let base = if default_cfg_path.exists() {
        std::fs::read_to_string(&default_cfg_path)?
    } else {
        DEFAULT_SOLANA_CONFIG.to_string()
    };

    let keypair_line = format!("keypair_path: {}", paths.active_entry(addr).display());
    let mut found = false;
    let mut out_lines: Vec<String> = Vec::new();
    for line in base.lines() {
        if line.trim_start().starts_with("keypair_path:") {
            out_lines.push(keypair_line.clone());
            found = true;
        } else {
            out_lines.push(line.to_string());
        }
    }
    if !found {
        out_lines.push(keypair_line);
    }

    let out_dir = paths.root().join("shell-config");
    std::fs::create_dir_all(&out_dir)?;
    let out_path = out_dir.join(format!("{}.yml", addr.stem()));
    std::fs::write(&out_path, out_lines.join("\n") + "\n")?;
    Ok(out_path)
}
