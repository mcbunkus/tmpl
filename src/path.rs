use std::path::{Component, Path};

/// This validates a path for safety, in the context of this tool. path_is_safe will return false
/// if any of the following are true:
///
/// - path is absolute
/// - path leaves the current working directory, meaning it escapes out the parent
///
/// This function is used to ensure that a path is either the CWD itself, or nested inside it.
pub fn path_is_safe(path: &Path) -> bool {
    // never in a million years
    if path.is_absolute() {
        return false;
    }

    let mut depth = 0;
    for component in path.components() {
        match component {
            Component::Normal(_) => depth += 1, // yep
            // maybe
            Component::ParentDir => {
                depth -= 1;
                if depth < 0 {
                    return false;
                }
            }
            Component::CurDir => {} // means "." so that's fine
            _ => return false,      // anything else is just no good
        }
    }

    true
}
