//! This is an implementation of simple [FileRotation](FileRotation) mechanism using only std.
//! Given a file like `my.log`, it will copy that file to `my.1.log`, renaming a
//! potentially pre-existing `my.1.log` to `my.2.log`. It accepts an optional
//! number of max filesto keep. It will only rotate files when invoked, it will
//! /not/ watch any files or do any kind of background processing.
//!
//! ```no_run
//! use simple_file_rotation::{FileRotation};
//! # fn example() -> simple_file_rotation::Result<()> {
//! FileRotation::new("my.log")
//!     .max_old_files(2)
//!     .rotate()?;
//! # Ok(())
//! # }
//! ```
//!
//! Why yet another file rotation library?
//! - No additional dependencies.
//! - No features I don't need.

pub use error::{FileRotationError, Result};
use std::path::{is_separator, Path, PathBuf};

mod error;

pub struct FileRotation {
    max_old_files: Option<usize>,
    file: PathBuf,
    extension: String,
}

/// See module documentation.
impl FileRotation {
    pub fn new(file: impl AsRef<Path>) -> Self {
        Self {
            file: file.as_ref().to_path_buf(),
            max_old_files: None,
            extension: "log".to_string(),
        }
    }

    /// Set a maximum of how many files to keep.
    #[must_use]
    pub fn max_old_files(mut self, max_old_files: usize) -> Self {
        self.max_old_files = Some(max_old_files);
        self
    }

    /// Set a file extension to use if none is present in the original filename
    pub fn file_extension(mut self, extension: impl ToString) -> Self {
        self.extension = extension.to_string();
        self
    }

    pub fn rotate(self) -> Result<()> {
        let Self {
            max_old_files,
            file,
            extension,
        } = self;

        let is_dir = file
            .to_str()
            .and_then(|path| path.chars().last())
            .map(is_separator)
            .unwrap_or(false);

        if is_dir {
            return Err(FileRotationError::NotAFile(file));
        }

        // enforce the file to have an extension
        let data_file = match file.extension() {
            Some(_) => file,
            None => file.with_extension(&extension),
        };

        let data_file_name = match data_file.file_name() {
            Some(data_file_name) => data_file_name,
            _ => return Err(FileRotationError::NotAFile(data_file)),
        };

        let data_file_dir = data_file
            .parent()
            .and_then(|p| {
                let dir = p.to_path_buf();
                if dir.to_string_lossy().is_empty() {
                    None
                } else {
                    Some(dir)
                }
            })
            .unwrap_or_else(|| PathBuf::from("."));

        let mut rotations = Vec::new();
        for entry in (std::fs::read_dir(&data_file_dir)?).flatten() {
            let direntry_pathbuf = entry.path();

            let file_name = entry.file_name();
            if file_name == data_file_name {
                rotations.push((
                    direntry_pathbuf.clone(),
                    data_file_name
                        .to_string_lossy()
                        .replace(&format!(".{}", &extension), &format!(".1.{}", &extension)),
                ));
            }

            let data_file_name = data_file_name.to_string_lossy();
            let file_name = file_name.to_string_lossy();
            let parts = file_name.split('.').collect::<Vec<_>>();
            match parts[..] {
                [prefix, n, ext] if !prefix.is_empty() && data_file_name.starts_with(prefix) => {
                    if let Ok(n) = n.parse::<usize>() {
                        let new_name = format!("{prefix}.{}.{ext}", n + 1);
                        rotations.push((direntry_pathbuf, new_name));
                    }
                }
                _ => continue,
            }
        }

        rotations.sort_by_key(|(_, new_name)| new_name.to_string());

        if let Some(max_old_files) = max_old_files {
            while rotations.len() > max_old_files {
                if let Some((data_file, _)) = rotations.pop() {
                    if let Err(err) = std::fs::remove_file(data_file.clone()) {
                        eprintln!(
                            "Rotating logs: cannot remove file {}: {err}",
                            data_file.display()
                        );
                    }
                }
            }
        }

        for (entry, new_file_name) in rotations.into_iter().rev() {
            if let Err(err) = std::fs::rename(entry.clone(), data_file_dir.join(new_file_name)) {
                eprintln!("Error rotating file {entry:?}: {err}");
            }
        }

        Ok(())
    }
}
