use crate::error::AppError;
use crate::vault::env::KeyAddr;
use crate::vault::index::Index;
use crate::vault::paths::Paths;
use crate::vault::prompt;
use crate::vault::symlink;

pub fn run(addr: Option<String>, all: bool) -> Result<(), AppError> {
    let paths = Paths::resolve()?;
    let solana_id = paths.solana_id()?;
    let symlink_target = symlink::current_target(&solana_id)?;

    if all {
        let mut any_dangled = false;
        if paths.active_dir().exists() {
            for entry in std::fs::read_dir(paths.active_dir())? {
                let path = entry?.path();
                if Some(&path) == symlink_target.as_ref() {
                    any_dangled = true;
                }
                std::fs::remove_file(&path)?;
            }
        }
        if any_dangled {
            println!("{} left dangling (its key was in the killed set)", solana_id.display());
        }
        println!("killed entire active set");
        return Ok(());
    }

    let addr = match addr {
        Some(s) => KeyAddr::parse(&s)?,
        None => {
            let index = Index::load(&paths)?;
            prompt::pick_active(&paths, &index, "kill which key?")?
        }
    };

    let active_path = paths.active_entry(&addr);
    if !active_path.exists() {
        println!("{addr} not active");
        return Ok(());
    }
    std::fs::remove_file(&active_path)?;
    if Some(&active_path) == symlink_target.as_ref() {
        println!("{} left dangling ({addr} was its target)", solana_id.display());
    }
    println!("killed {addr}");
    Ok(())
}
