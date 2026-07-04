use crate::error::AppError;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};

/// Resolves `link`'s target to an absolute path, if `link` is a symlink at all.
pub fn current_target(link: &Path) -> Result<Option<PathBuf>, AppError> {
    match std::fs::symlink_metadata(link) {
        Ok(m) if m.file_type().is_symlink() => {
            let target = std::fs::read_link(link)?;
            let target = if target.is_absolute() {
                target
            } else {
                link.parent().unwrap_or_else(|| Path::new(".")).join(target)
            };
            Ok(Some(target))
        }
        _ => Ok(None),
    }
}

/// Repoint `link` at `target`: create a temp symlink next to it, then `rename()`
/// over `link` — atomic, never delete-then-create (design §4a).
pub fn atomic_repoint(link: &Path, target: &Path) -> Result<(), AppError> {
    if let Some(parent) = link.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let tmp = link.with_extension("keymgr-tmp");
    let _ = std::fs::remove_file(&tmp);
    symlink(target, &tmp)?;
    std::fs::rename(&tmp, link)?;
    Ok(())
}
