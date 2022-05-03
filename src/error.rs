pub type Result<T> = std::result::Result<T, FileRotationError>;

#[non_exhaustive]
#[derive(Debug)]
pub enum FileRotationError {
    Io(std::io::Error),
}

impl std::error::Error for FileRotationError {}

impl std::fmt::Display for FileRotationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FileRotationError::Io(err) => {
                write!(f, "FileRotation io error: {err}")
            }
        }
    }
}

impl From<std::io::Error> for FileRotationError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}
