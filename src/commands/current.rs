use crate::error::AppError;
use crate::vault::index::Index;
use crate::vault::paths::Paths;
use crate::vault::symlink;

pub fn run() -> Result<(), AppError> {
    let paths = Paths::resolve()?;
    let index = Index::load(&paths)?;
    let solana_id = paths.solana_id()?;

    match symlink::current_target(&solana_id)? {
        Some(target) if target.exists() => {
            let stem = target.file_stem().and_then(|s| s.to_str()).unwrap_or("?");
            println!("default: {stem} (-> {})", target.display());
        }
        Some(_) => println!("default: none (dangling symlink)"),
        None if solana_id.exists() => println!("default: none (regular file, not managed by keymgr)"),
        None => println!("default: none (missing)"),
    }

    println!("\nactive:");
    if !paths.active_dir().exists() {
        return Ok(());
    }
    let mut any = false;
    for entry in std::fs::read_dir(paths.active_dir())? {
        let path = entry?.path();
        let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else { continue };
        let pubkey = index
            .keys
            .iter()
            .find(|k| format!("{}_{}", k.env, k.name) == stem)
            .map(|k| k.pubkey.clone())
            .unwrap_or_else(|| "?(orphan)".to_string());
        let age_secs = std::fs::metadata(&path)?
            .modified()?
            .elapsed()
            .map(|d| d.as_secs())
            .unwrap_or(0);
        println!("  {stem}  {pubkey}  active {age_secs}s");
        any = true;
    }
    if !any {
        println!("  (none)");
    }
    Ok(())
}
