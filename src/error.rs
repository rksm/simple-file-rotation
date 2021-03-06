use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, FileRotationError>;

#[non_exhaustive]
#[derive(Debug)]
pub enum FileRotationError {
    NotAFile(PathBuf),
    Io(std::io::Error),
}

impl std::error::Error for FileRotationError {}

impl std::fmt::Display for FileRotationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FileRotationError::Io(err) => {
                write!(f, "FileRotation io error: {err}")
            }
            FileRotationError::NotAFile(path) => {
                write!(f, "path {path:?} is not a file")
            }
        }
    }
}

impl From<std::io::Error> for FileRotationError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}
