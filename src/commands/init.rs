use crate::error::AppError;
use crate::vault::env::KeyAddr;
use crate::vault::index::Index;
use crate::vault::paths::Paths;
use crate::vault::store::{encode_keypair_json, store, write_0600};
use crate::vault::symlink;
use crate::vault::verifier;
use solana_keypair::{read_keypair_file, Signer};
use std::path::Path;

enum State {
    Missing,
    OurSymlink,
    ForeignSymlink,
    RegularFile,
}

fn detect_state(solana_id: &Path, active_dir: &Path) -> Result<State, AppError> {
    if let Err(e) = std::fs::symlink_metadata(solana_id) {
        return if e.kind() == std::io::ErrorKind::NotFound {
            Ok(State::Missing)
        } else {
            Err(e.into())
        };
    }
    match symlink::current_target(solana_id)? {
        Some(target) if target.starts_with(active_dir) => Ok(State::OurSymlink),
        Some(_) => Ok(State::ForeignSymlink),
        None if solana_id.is_file() => Ok(State::RegularFile),
        None => Ok(State::ForeignSymlink),
    }
}

pub fn run(undo: bool) -> Result<(), AppError> {
    let paths = Paths::resolve()?;
    paths.ensure_dirs()?;
    let solana_id = paths.solana_id()?;

    if undo {
        return undo_init(&paths, &solana_id);
    }

    match detect_state(&solana_id, &paths.active_dir())? {
        State::OurSymlink => {
            println!("already set up: {} -> keymgr active set", solana_id.display());
            Ok(())
        }
        State::ForeignSymlink => Err(AppError::ForeignSymlink(format!(
            "{} is a symlink keymgr does not own",
            solana_id.display()
        ))),
        State::Missing => {
            let addr = KeyAddr::parse("local/default").expect("static addr is valid");
            symlink::atomic_repoint(&solana_id, &paths.active_entry(&addr))?;
            println!(
                "created dangling symlink {} (no key active yet — run `keymgr use local/default` after creating one)",
                solana_id.display()
            );
            Ok(())
        }
        State::RegularFile => convert_regular_file(&paths, &solana_id),
    }
}

fn convert_regular_file(paths: &Paths, solana_id: &Path) -> Result<(), AppError> {
    let addr = KeyAddr::parse("local/default").expect("static addr is valid");

    let mut index = Index::load(paths)?;
    if index.contains(&addr) {
        return Err(AppError::DuplicateKey(addr.to_string()));
    }

    let keypair = read_keypair_file(solana_id)
        .map_err(|e| AppError::InvalidKeypairFile(e.to_string()))?;
    let plaintext = encode_keypair_json(keypair.to_bytes())?;

    std::fs::create_dir_all(paths.backup_dir())?;
    std::fs::copy(solana_id, paths.id_backup_file())?;

    let pw = verifier::unlock(paths)?;
    store(paths, &mut index, &addr, &keypair.pubkey().to_string(), &plaintext, pw.as_bytes())?;
    write_0600(&paths.active_entry(&addr), &plaintext)?;
    symlink::atomic_repoint(solana_id, &paths.active_entry(&addr))?;

    println!("backed up original to {}", paths.id_backup_file().display());
    println!("imported as local/default, pubkey: {}", keypair.pubkey());
    println!("{} now points into keymgr's active set", solana_id.display());
    Ok(())
}

fn undo_init(paths: &Paths, solana_id: &Path) -> Result<(), AppError> {
    let backup = paths.id_backup_file();
    if !backup.exists() {
        return Err(AppError::Other(format!("no backup found at {}", backup.display())));
    }
    let _ = std::fs::remove_file(solana_id);
    std::fs::rename(&backup, solana_id)?;
    println!("restored {} from backup", solana_id.display());
    Ok(())
}
