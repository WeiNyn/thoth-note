use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to create note directory at {0}")]
    DirectoryCreation(PathBuf),

    #[error("Note with title {0} not found")]
    NoteNotFound(String),

    #[error("Failed to parse note metadata: {0}")]
    MetadataParse(String),
}

pub type StorageResult<T> = Result<T, StorageError>;
