use std::{
    env::current_dir,
    path::{Component, Path},
};

use anyhow::{Context, Result, bail};

/// This validates a path for safety, in the context of this tool. path_is_safe will return false
/// if any of the following are true:
///
/// - path is absolute
/// - path leaves the current working directory, meaning it escapes out the parent
///
/// This function is used to ensure that a path is either the CWD itself, or nested inside it.
pub fn check_path_is_valid(path: &Path) -> Result<()> {
    // never in a million years
    if path.is_absolute() {
        bail!(
            "{} is an absolute path. Absolute paths are forbidden for security reasons.",
            path.display()
        );
    }

    let mut depth = 0;
    for component in path.components() {
        match component {
            Component::Normal(_) => depth += 1, // yep
            // maybe
            Component::ParentDir => {
                depth -= 1;
                if depth < 0 {
                    let cwd = current_dir().context(format!("Attempted to report that {} would escape the current working directory, but an error occurred getting the current working directory.", path.display()))?;
                    bail!(
                        "{} escaped outside the current directory ({})",
                        path.display(),
                        cwd.display()
                    );
                }
            }
            Component::CurDir => {} // means "." so that's fine
            _ => bail!("{} is not a valid path", path.display()),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn valid_path() {
        let good_path = PathBuf::from("nested/paths/are/fine.txt");
        assert!(check_path_is_valid(&good_path).is_ok());
    }

    #[test]
    fn escaping_cwd_returns_error() {
        let path = PathBuf::from("uh/oh/../../../../..");
        assert!(check_path_is_valid(&path).is_err());
    }

    #[test]
    fn curdir_is_ok() {
        let cwd = PathBuf::from(".");
        assert!(check_path_is_valid(&cwd).is_ok());

        let down_and_back_again = PathBuf::from("nested/then/back/../../..");
        assert!(check_path_is_valid(&down_and_back_again).is_ok());
    }

    #[test]
    fn absolute_path_returns_error() {
        let absolute_path = PathBuf::from("/this/is/a/no/no.md");
        assert!(check_path_is_valid(&absolute_path).is_err());
    }
}
