use crate::util::errors::CorgiResult;
use cargo_util::paths;
use std::path::{Path, PathBuf};

/// Finds the root `Cargo.toml`.
pub fn find_root_manifest_for_wd(cwd: &Path) -> CorgiResult<PathBuf> {
    let valid_corgi_toml_file_name = "Cargo.toml";
    let invalid_corgi_toml_file_name = "corgi.toml";
    let mut invalid_corgi_toml_path_exists = false;

    for current in paths::ancestors(cwd, None) {
        let manifest = current.join(valid_corgi_toml_file_name);
        if manifest.exists() {
            return Ok(manifest);
        }
        if current.join(invalid_corgi_toml_file_name).exists() {
            invalid_corgi_toml_path_exists = true;
        }
    }

    if invalid_corgi_toml_path_exists {
        anyhow::bail!(
        "could not find `{}` in `{}` or any parent directory, but found corgi.toml please try to rename it to Cargo.toml",
        valid_corgi_toml_file_name,
        cwd.display()
    )
    } else {
        anyhow::bail!(
            "could not find `{}` in `{}` or any parent directory",
            valid_corgi_toml_file_name,
            cwd.display()
        )
    }
}

/// Returns the path to the `file` in `pwd`, if it exists.
pub fn find_project_manifest_exact(pwd: &Path, file: &str) -> CorgiResult<PathBuf> {
    let manifest = pwd.join(file);

    if manifest.exists() {
        Ok(manifest)
    } else {
        anyhow::bail!("Could not find `{}` in `{}`", file, pwd.display())
    }
}
