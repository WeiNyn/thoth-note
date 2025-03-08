pub mod error;
pub mod fs;

use crate::models::note::Note;
use error::StorageResult;

/// Storage trait defines the interface for note persistence
pub trait Storage {
    /// Initialize the storage (create directories, etc.)
    fn init(&self) -> StorageResult<()>;

    /// List all available notes
    fn list_notes(&self) -> StorageResult<Vec<Note>>;

    /// Read a specific note by title
    fn read_note(&self, title: &str) -> StorageResult<Note>;

    /// Write a note to storage
    fn write_note(&self, note: &Note) -> StorageResult<()>;

    /// Delete a note from storage
    fn delete_note(&self, title: &str) -> StorageResult<()>;

    /// Rename a note in storage
    fn rename_note(&self, old_title: &str, note: &Note) -> StorageResult<()> {
        self.delete_note(old_title)?;
        self.write_note(note)
    }
}
