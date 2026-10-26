use std::{
    fs, io,
    path::{Component, Path, PathBuf},
};

fn invalid(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message)
}

/// Resolve existing prefixes, preserving a nonexistent suffix without writes.
/// A dangling link is an error, not a nonexistent suffix. Never normalize '..'
/// before following an existing symlink: that would change filesystem meaning.
pub(crate) fn resolve(root: &Path, raw: &str, inspect: bool) -> io::Result<PathBuf> {
    if raw.trim().is_empty() {
        return Err(invalid("path must not be empty"));
    }
    let mut result = root.to_path_buf();
    for component in Path::new(raw).components() {
        match component {
            Component::Prefix(_) | Component::RootDir => {
                return Err(invalid("absolute paths are not allowed"));
            }
            Component::CurDir => {}
            Component::ParentDir => {
                if result == root {
                    return Err(invalid("path escapes its declared root"));
                }
                if inspect {
                    ensure_directory_or_missing(&result)?;
                }
                result.pop();
            }
            Component::Normal(name) => {
                if inspect {
                    ensure_directory_or_missing(&result)?;
                }
                result.push(name);
                if inspect && missing_is_none(fs::symlink_metadata(&result))?.is_some() {
                    result = fs::canonicalize(&result)?;
                }
            }
        }
        if !result.starts_with(root) {
            return Err(invalid("path escapes its declared root through a symlink"));
        }
    }
    Ok(result)
}

fn ensure_directory_or_missing(path: &Path) -> io::Result<()> {
    match missing_is_none(fs::metadata(path))? {
        Some(meta) if !meta.is_dir() => Err(invalid("a path ancestor is not a directory")),
        _ => Ok(()),
    }
}

// Shared by metadata observations: absence is data, other failures are errors.
// Keeping this classification in one place makes error semantics consistent
// across source state, ancestor inspection, and candidate resolution.
pub(super) fn missing_is_none<T>(observation: io::Result<T>) -> io::Result<Option<T>> {
    match observation {
        Ok(value) => Ok(Some(value)),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn absence_is_data_but_other_observation_failures_are_preserved() {
        assert_eq!(missing_is_none(Ok(42)).unwrap(), Some(42));
        assert_eq!(
            missing_is_none::<()>(Err(io::Error::from(io::ErrorKind::NotFound))).unwrap(),
            None
        );
        for kind in [
            io::ErrorKind::PermissionDenied,
            io::ErrorKind::InvalidInput,
            io::ErrorKind::Interrupted,
            io::ErrorKind::Other,
        ] {
            let error =
                missing_is_none::<()>(Err(io::Error::new(kind, "original failure"))).unwrap_err();
            assert_eq!(error.kind(), kind);
            assert_eq!(error.to_string(), "original failure");
        }
    }
}
