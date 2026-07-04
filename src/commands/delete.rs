use crate::error::AppError;
use crate::vault::env::KeyAddr;
use crate::vault::index::Index;
use crate::vault::paths::Paths;
use crate::vault::store;
use dialoguer::Input;

pub fn run(addr: String) -> Result<(), AppError> {
    let paths = Paths::resolve()?;
    let addr = KeyAddr::parse(&addr)?;
    let mut index = Index::load(&paths)?;

    if !index.contains(&addr) {
        return Err(AppError::KeyNotFound(addr.to_string()));
    }
    if paths.active_entry(&addr).exists() {
        return Err(AppError::StillActive(addr.to_string()));
    }

    let confirm: String = Input::new()
        .with_prompt(format!("retype '{addr}' to confirm delete"))
        .interact_text()
        .map_err(|e| AppError::Other(e.to_string()))?;
    if confirm != addr.to_string() {
        return Err(AppError::Other("confirmation did not match, aborted".into()));
    }

    store::delete(&paths, &mut index, &addr)?;
    println!("deleted {addr}");
    Ok(())
}
