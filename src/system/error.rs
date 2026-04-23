use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FsError {
    NotFound(String),
    AlreadyExists(String),
    NotDirectory(String),
    NotFile(String),
    PermissionDenied { path: String, action: &'static str },
    DirectoryNotEmpty(String),
    InvalidPath(String),
    InvalidPermissions(String),
    CannotRemoveRoot,
}

impl fmt::Display for FsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound(path) => write!(f, "path not found: {path}"),
            Self::AlreadyExists(path) => write!(f, "path already exists: {path}"),
            Self::NotDirectory(path) => write!(f, "not a directory: {path}"),
            Self::NotFile(path) => write!(f, "not a file: {path}"),
            Self::PermissionDenied { path, action } => {
                write!(f, "permission denied for {action} on {path}")
            }
            Self::DirectoryNotEmpty(path) => write!(f, "directory is not empty: {path}"),
            Self::InvalidPath(path) => write!(f, "invalid path: {path}"),
            Self::InvalidPermissions(input) => write!(f, "invalid chmod mode: {input}"),
            Self::CannotRemoveRoot => write!(f, "cannot remove root directory"),
        }
    }
}

impl std::error::Error for FsError {}
