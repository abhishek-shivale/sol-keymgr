use crate::error::AppError;
use crate::vault::binding;
use crate::vault::index::Index;
use crate::vault::paths::Paths;
use crate::vault::prompt;

pub fn run(all: bool) -> Result<(), AppError> {
    let paths = Paths::resolve()?;
    let index = Index::load(&paths)?;

    let addr = if all {
        prompt::pick_from_index(&index, "pick a key")?
    } else {
        let cwd = std::env::current_dir()?;
        binding::find_binding(&cwd)?.ok_or(AppError::NoProjectBinding)?
    };

    let meta = index.find(&addr).ok_or_else(|| AppError::KeyNotFound(addr.to_string()))?;
    println!("{}", meta.pubkey);
    Ok(())
}
