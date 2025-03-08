use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use crate::models::note::Note;
use crate::storage::error::{StorageError, StorageResult};
use crate::storage::Storage;

/// Metadata for a note stored in the file system
#[derive(Debug, Serialize, Deserialize)]
struct NoteMetadata {
    title: String,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}

/// File system implementation of the Storage trait
pub struct FSStorage {
    root_dir: PathBuf,
}

impl Default for FSStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl FSStorage {
    /// Create a new FSStorage with the default root directory (~/.rnote)
    pub fn new() -> Self {
        let home_dir = dirs::home_dir().expect("Failed to get home directory");
        let root_dir = home_dir.join(".rnote");
        Self { root_dir }
    }

    /// Create a new FSStorage with a custom root directory
    pub fn with_root_dir<P: AsRef<Path>>(root_dir: P) -> Self {
        Self {
            root_dir: root_dir.as_ref().to_path_buf(),
        }
    }

    /// Get the path to a note file
    fn get_note_path(&self, title: &str) -> PathBuf {
        // Sanitize the title to be a valid filename
        let sanitized = title.replace("/", "_").replace("\\", "_");
        self.root_dir.join(format!("{}.md", sanitized))
    }

    /// Get the path to a note's metadata file
    fn get_metadata_path(&self, title: &str) -> PathBuf {
        // Sanitize the title to be a valid filename
        let sanitized = title.replace("/", "_").replace("\\", "_");
        self.root_dir.join(format!("{}.meta.json", sanitized))
    }

    /// Read metadata for a note
    fn read_metadata(&self, title: &str) -> StorageResult<NoteMetadata> {
        let path = self.get_metadata_path(title);
        let mut file =
            File::open(&path).map_err(|_| StorageError::NoteNotFound(title.to_string()))?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        serde_json::from_str(&contents).map_err(|e| StorageError::MetadataParse(e.to_string()))
    }

    /// Write metadata for a note
    fn write_metadata(&self, metadata: &NoteMetadata) -> StorageResult<()> {
        let path = self.get_metadata_path(&metadata.title);

        // Create a temporary file for atomic write
        let temp_path = path.with_extension("meta.json.tmp");
        let mut file = File::create(&temp_path)?;

        let json = serde_json::to_string_pretty(metadata)
            .map_err(|e| StorageError::MetadataParse(e.to_string()))?;

        file.write_all(json.as_bytes())?;
        file.flush()?;

        // Rename for atomic write
        fs::rename(temp_path, path)?;

        Ok(())
    }
}

impl Storage for FSStorage {
    fn init(&self) -> StorageResult<()> {
        if !self.root_dir.exists() {
            fs::create_dir_all(&self.root_dir)
                .map_err(|_| StorageError::DirectoryCreation(self.root_dir.clone()))?;
        }
        Ok(())
    }

    fn list_notes(&self) -> StorageResult<Vec<Note>> {
        self.init()?;

        let mut notes = Vec::new();

        for entry in fs::read_dir(&self.root_dir)? {
            let entry = entry?;
            let path = entry.path();

            // Only process .md files
            if path.extension().is_some_and(|ext| ext == "md") {
                if let Some(filename) = path.file_stem() {
                    let title = filename.to_string_lossy().to_string();

                    // Try to read the note
                    if let Ok(note) = self.read_note(&title) {
                        notes.push(note);
                    }
                }
            }
        }

        // Sort notes by updated_at (newest first)
        notes.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

        Ok(notes)
    }

    fn read_note(&self, title: &str) -> StorageResult<Note> {
        let path = self.get_note_path(title);
        let metadata_path = self.get_metadata_path(title);

        // Check if both files exist
        if !path.exists() || !metadata_path.exists() {
            return Err(StorageError::NoteNotFound(title.to_string()));
        }

        // Read content
        let mut file = File::open(&path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        // Read metadata
        let metadata = self.read_metadata(title)?;

        Ok(Note {
            title: metadata.title,
            content,
            created_at: metadata.created_at,
            updated_at: metadata.updated_at,
            selected: false,
        })
    }

    fn write_note(&self, note: &Note) -> StorageResult<()> {
        self.init()?;

        let path = self.get_note_path(&note.title);

        // Create a temporary file for atomic write
        let temp_path = path.with_extension("md.tmp");
        let mut file = File::create(&temp_path)?;

        // Write content
        file.write_all(note.content.as_bytes())?;
        file.flush()?;

        // Rename for atomic write
        fs::rename(&temp_path, &path)?;

        // Write metadata
        let metadata = NoteMetadata {
            title: note.title.clone(),
            created_at: note.created_at,
            updated_at: note.updated_at,
        };

        self.write_metadata(&metadata)?;

        Ok(())
    }

    fn delete_note(&self, title: &str) -> StorageResult<()> {
        let path = self.get_note_path(title);
        let metadata_path = self.get_metadata_path(title);

        // Check if the note exists
        if !path.exists() {
            return Err(StorageError::NoteNotFound(title.to_string()));
        }

        // Delete the files
        if path.exists() {
            fs::remove_file(path)?;
        }

        if metadata_path.exists() {
            fs::remove_file(metadata_path)?;
        }

        Ok(())
    }
}
