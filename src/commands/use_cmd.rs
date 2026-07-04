use crate::error::AppError;
use crate::vault::env::KeyAddr;
use crate::vault::index::Index;
use crate::vault::paths::Paths;
use crate::vault::store::{load_decrypt, write_0600};
use crate::vault::{binding, prompt, symlink, verifier};

pub fn run(addr: Option<String>, no_toml: bool) -> Result<(), AppError> {
    let paths = Paths::resolve()?;
    paths.ensure_dirs()?;
    let index = Index::load(&paths)?;
    let cwd = std::env::current_dir()?;

    let addr = match addr {
        Some(s) => KeyAddr::parse(&s)?,
        None => match binding::find_binding(&cwd)? {
            Some(a) => a,
            None => prompt::pick_from_index(&index, "use which key?")?,
        },
    };

    let meta = index.find(&addr).ok_or_else(|| AppError::KeyNotFound(addr.to_string()))?;

    let solana_id = paths.solana_id()?;
    let active_path = paths.active_entry(&addr);
    let already_active = active_path.exists();
    let symlink_points_here = symlink::current_target(&solana_id)?.as_deref() == Some(active_path.as_path());

    if !already_active {
        let pw = verifier::unlock(&paths)?;
        let plaintext = load_decrypt(&paths, &addr, pw.as_bytes())?;
        write_0600(&active_path, &plaintext)?;
    }
    if !symlink_points_here {
        symlink::atomic_repoint(&solana_id, &active_path)?;
    }

    if !no_toml {
        binding::bind(&cwd, &addr)?;
    }

    if addr.env.is_prod() {
        eprintln!("\x1b[1;31m[PROD]\x1b[0m");
    }
    println!("using {addr}  pubkey: {}", meta.pubkey);
    Ok(())
}
