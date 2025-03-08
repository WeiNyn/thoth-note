use chrono::Local;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use edtui::{EditorEventHandler, EditorState};
use edtui_jagged::Jagged;
use ratatui::{DefaultTerminal, Frame};
use tui_widget_list::ListState;

use crate::commands::Command;
use crate::models::note::Note;
use crate::storage::{fs::FSStorage, Storage};
use crate::theme::AppTheme;
use crate::ui;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum View {
    List,
    Editor,
    Preview,
    Rename,
    LivePreview,
    DeleteConfirm,
}

pub struct AppState {
    pub notes: Vec<Note>,
    pub list_state: ListState,
    pub editor_state: EditorState,
    pub preview_scroll_offset: usize,
    pub current_view: View,
    pub theme: AppTheme,
    pub rename_buffer: String,
    pub creating_new_note: bool,
    pub confirm_delete: bool,
}

pub struct App {
    state: AppState,
    editor_event_handler: EditorEventHandler,
    storage: Box<dyn Storage>,
    running: bool,
}

impl Default for AppState {
    fn default() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            notes: Vec::new(),
            list_state,
            editor_state: EditorState::default(),
            preview_scroll_offset: 0,
            current_view: View::LivePreview,
            theme: AppTheme::default(),
            rename_buffer: String::new(),
            creating_new_note: false,
            confirm_delete: false,
        }
    }
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        let mut state = AppState::default();

        // Create storage
        let storage = Box::new(FSStorage::new());

        // Initialize storage
        if let Err(e) = storage.init() {
            eprintln!("Failed to initialize storage: {}", e);
        }

        // Try to load notes from storage and initialize order if needed
        let mut loaded_notes = Vec::new();
        if let Ok(mut notes) = storage.list_notes() {
            // If notes exist but don't have proper order (all 0), initialize them
            let all_zero = notes.iter().all(|note| note.order == 0);
            if all_zero && !notes.is_empty() {
                // Sort by updated_at first to maintain previous order
                notes.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
                
                // Assign orders based on position
                for (i, note) in notes.iter_mut().enumerate() {
                    note.order = i;
                    if let Err(e) = storage.write_note(note) {
                        eprintln!("Failed to update note order: {}", e);
                    }
                }
            }
            loaded_notes = notes;
        }

        // If no notes were loaded, create example notes
        if loaded_notes.is_empty() {
            loaded_notes = Self::create_example_notes();

            // Save example notes to storage
            for note in &loaded_notes {
                if let Err(e) = storage.write_note(note) {
                    eprintln!("Failed to save note '{}': {}", note.title, e);
                }
            }
        }

        state.notes = loaded_notes;

        // Set initial editor content
        if !state.notes.is_empty() {
            let content = state.notes[0].content.clone();
            state.editor_state.lines = Jagged::from(content);
        }

        Self {
            state,
            editor_event_handler: EditorEventHandler::default(),
            storage,
            running: false,
        }
    }

    fn create_example_notes() -> Vec<Note> {
        let welcome_content = include_str!("welcome.md");
        vec![Note {
            title: "Welcome to RNote".to_string(),
            content: welcome_content.to_string(),
            created_at: Local::now(),
            updated_at: Local::now(),
            selected: false,
            order: 0,
        }]
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        // Initialize app state
        if self.state.notes.is_empty() {
            self.create_new_note();
        }

        self.running = true;
        while self.running {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    /// Renders the user interface.
    fn draw(&mut self, frame: &mut Frame) {
        ui::render(frame, &mut self.state);
    }

    fn handle_events(&mut self) -> Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                if let Some(command) = self.key_to_command(key) {
                    self.execute_command(command);
                } else {
                    match self.state.current_view {
                        View::Editor | View::LivePreview => {
                            self.editor_event_handler
                                .on_event(Event::Key(key), &mut self.state.editor_state);
                        }
                        View::Rename => {
                            self.handle_rename_input(key);
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }

    fn key_to_command(&self, key: KeyEvent) -> Option<Command> {
        if self.state.current_view == View::DeleteConfirm {
            match key.code {
                KeyCode::Enter => Some(Command::DeleteNote),
                KeyCode::Esc => Some(Command::CancelRename), // Reuse CancelRename as it does the same thing
                _ => None,
            }
        } else {
            match (key.modifiers, key.code) {
                (KeyModifiers::CONTROL, KeyCode::Char('q')) => Some(Command::Quit),
                (KeyModifiers::CONTROL, KeyCode::Down) => Some(Command::NextNote),
                (KeyModifiers::CONTROL, KeyCode::Up) => Some(Command::PreviousNote),
                (KeyModifiers::ALT, KeyCode::Up) => Some(Command::MoveNoteUp),
                (KeyModifiers::ALT, KeyCode::Down) => Some(Command::MoveNoteDown),
                (KeyModifiers::CONTROL, KeyCode::Char('e')) => Some(Command::SwitchView(View::Editor)),
                (KeyModifiers::CONTROL, KeyCode::Char('l')) => {
                    Some(Command::SwitchView(View::LivePreview))
                }
                (KeyModifiers::CONTROL, KeyCode::Char('p')) => Some(Command::SwitchView(View::Preview)),
                (KeyModifiers::CONTROL, KeyCode::Char('n')) => Some(Command::NewNote),
                (KeyModifiers::CONTROL, KeyCode::Char('s')) => Some(Command::SaveNote),
                (KeyModifiers::CONTROL, KeyCode::Char('d')) => Some(Command::DeleteNote),
                (KeyModifiers::CONTROL, KeyCode::Char('j')) => Some(Command::ScrollDown),
                (KeyModifiers::CONTROL, KeyCode::Char('k')) => Some(Command::ScrollUp),
                (KeyModifiers::CONTROL, KeyCode::Char('r')) => Some(Command::RenameNote),
                (KeyModifiers::NONE, KeyCode::Enter)
                    if matches!(self.state.current_view, View::Rename) => Some(Command::SubmitRename),
                (KeyModifiers::NONE, KeyCode::Esc)
                    if matches!(self.state.current_view, View::Rename) => Some(Command::CancelRename),
                _ => None,
            }
        }
    }

    fn handle_rename_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(c) => {
                self.state.rename_buffer.push(c);
            }
            KeyCode::Backspace => {
                self.state.rename_buffer.pop();
            }
            _ => {}
        }
    }

    fn start_delete(&mut self) {
        self.state.current_view = View::DeleteConfirm;
        self.state.confirm_delete = false;
    }

    fn execute_command(&mut self, command: Command) {
        match command {
            Command::Quit => self.quit(),
            Command::NextNote => self.select_next_note(),
            Command::PreviousNote => self.select_previous_note(),
            Command::SwitchView(view) => self.state.current_view = view,
            Command::NewNote => self.create_new_note(),
            Command::SaveNote => self.save_current_note(),
            Command::DeleteNote => {
                if self.state.current_view == View::DeleteConfirm {
                    self.state.confirm_delete = true;
                    self.delete_current_note();
                    self.state.current_view = View::LivePreview;
                } else {
                    self.state.current_view = View::DeleteConfirm;
                    self.state.confirm_delete = false;
                    self.state.current_view = View::DeleteConfirm;
                }
            }
            Command::ScrollDown => {
                if let View::Preview | View::LivePreview = self.state.current_view {
                    self.state.preview_scroll_offset += 5;
                }
            }
            Command::ScrollUp => {
                if let View::Preview | View::LivePreview = self.state.current_view {
                    if self.state.preview_scroll_offset > 0 {
                        self.state.preview_scroll_offset -= 5;
                    }
                }
            }
            Command::RenameNote => self.start_rename(),
            Command::SubmitRename => self.submit_rename(),
            Command::CancelRename => {
                if self.state.current_view == View::DeleteConfirm {
                    self.state.current_view = View::LivePreview;
                } else {
                    self.cancel_rename()
                }
            }
            Command::MoveNoteUp => self.move_note_up(),
            Command::MoveNoteDown => self.move_note_down(),
        }
    }

    fn move_note_up(&mut self) {
        if let Some(selected) = self.state.list_state.selected {
            if selected > 0 {
                // Get the orders
                let current_order = self.state.notes[selected].order;
                let prev_order = self.state.notes[selected - 1].order;

                // Swap orders
                self.state.notes[selected].order = prev_order;
                self.state.notes[selected - 1].order = current_order;

                // Save both notes to persist order changes
                if let Err(e) = self.storage.write_note(&self.state.notes[selected]) {
                    eprintln!("Failed to save note order: {}", e);
                }
                if let Err(e) = self.storage.write_note(&self.state.notes[selected - 1]) {
                    eprintln!("Failed to save note order: {}", e);
                }

                // Resort the notes
                self.state.notes.sort_by_key(|note| note.order);

                // Update selection
                self.state.list_state.select(Some(selected - 1));
            }
        }
    }

    fn move_note_down(&mut self) {
        if let Some(selected) = self.state.list_state.selected {
            if selected < self.state.notes.len() - 1 {
                // Get the orders
                let current_order = self.state.notes[selected].order;
                let next_order = self.state.notes[selected + 1].order;

                // Swap orders
                self.state.notes[selected].order = next_order;
                self.state.notes[selected + 1].order = current_order;

                // Save both notes to persist order changes
                if let Err(e) = self.storage.write_note(&self.state.notes[selected]) {
                    eprintln!("Failed to save note order: {}", e);
                }
                if let Err(e) = self.storage.write_note(&self.state.notes[selected + 1]) {
                    eprintln!("Failed to save note order: {}", e);
                }

                // Resort the notes
                self.state.notes.sort_by_key(|note| note.order);

                // Update selection
                self.state.list_state.select(Some(selected + 1));
            }
        }
    }

    fn select_next_note(&mut self) {
        self.save_editor_content_to_current_note();
        self.state.list_state.next();
        let to_index = self.state.list_state.selected.unwrap_or(0);
        self.load_note_to_editor(to_index);
    }

    fn select_previous_note(&mut self) {
        self.save_editor_content_to_current_note();
        self.state.list_state.previous();
        let to_index = self.state.list_state.selected.unwrap_or(0);
        self.load_note_to_editor(to_index);
    }

    fn save_editor_content_to_current_note(&mut self) {
        if let Some(selected) = self.state.list_state.selected {
            if let Some(note) = self.state.notes.get_mut(selected) {
                let content_rows = self
                    .state
                    .editor_state
                    .lines
                    .flatten(&Some('\n'))
                    .iter()
                    .map(|row| row.to_string())
                    .collect::<Vec<String>>();
                note.content = content_rows.join("");
                note.updated_at = Local::now();
            }
        }
    }

    fn load_note_to_editor(&mut self, index: usize) {
        if let Some(note) = self.state.notes.get_mut(index) {
            let content = note.content.clone();
            self.state.editor_state.lines = Jagged::from(content);
        }
    }

    fn create_new_note(&mut self) {
        self.state.current_view = View::Rename;
        self.state.rename_buffer = String::new();
        self.state.creating_new_note = true;
    }

    fn save_current_note(&mut self) {
        self.save_editor_content_to_current_note();

        // Save to storage
        if let Some(selected) = self.state.list_state.selected {
            if let Some(note) = self.state.notes.get(selected) {
                if let Err(e) = self.storage.write_note(note) {
                    eprintln!("Failed to save note '{}': {}", note.title, e);
                }
            }
        }
    }

    fn delete_current_note(&mut self) {
        // Only proceed if confirmed
        if !self.state.confirm_delete {
            return;
        }
        self.state.confirm_delete = false;

        if let Some(selected) = self.state.list_state.selected {
            if !self.state.notes.is_empty() {
                // Get the title before removing from memory
                let title = self.state.notes[selected].title.clone();
                let order = self.state.notes[selected].order;

                // Remove from memory
                self.state.notes.remove(selected);

                // Remove from storage
                if let Err(e) = self.storage.delete_note(&title) {
                    eprintln!("Failed to delete note '{}' from storage: {}", title, e);
                }

                // Adjust remaining notes' order
                for note in self.state.notes.iter_mut() {
                    if note.order > order {
                        note.order -= 1;
                        if let Err(e) = self.storage.write_note(note) {
                            eprintln!("Failed to update note order: {}", e);
                        }
                    }
                }

                // Resort the notes after order adjustment
                self.state.notes.sort_by_key(|note| note.order);

                // Adjust selection if needed
                if self.state.notes.is_empty() {
                    self.create_new_note();
                } else if selected >= self.state.notes.len() {
                    self.state
                        .list_state
                        .select(Some(self.state.notes.len() - 1));
                }

                if let Some(new_selected) = self.state.list_state.selected {
                    self.load_note_to_editor(new_selected);
                }
            }
        }
    }

    fn start_rename(&mut self) {
        if let Some(selected) = self.state.list_state.selected {
            if let Some(note) = self.state.notes.get(selected) {
                self.state.rename_buffer = note.title.clone();
                self.state.current_view = View::Rename;
            }
        }
    }

    fn submit_rename(&mut self) {
        if self.state.rename_buffer.is_empty() {
            return;
        }

        let new_title = self.state.rename_buffer.clone();
        self.state.rename_buffer.clear();

        if let View::Rename = self.state.current_view {
            if let Some(selected) = self.state.list_state.selected {
                // If we're creating a new note
                if self.state.creating_new_note {
                    // Find the maximum order and add 1 for the new note
                    let max_order = self.state.notes.iter()
                        .map(|note| note.order)
                        .max()
                        .unwrap_or(0);

                    let new_note = Note {
                        title: new_title,
                        content: String::new(),
                        created_at: Local::now(),
                        updated_at: Local::now(),
                        selected: false,
                        order: max_order + 1,
                    };

                    // Save to storage
                    if let Err(e) = self.storage.write_note(&new_note) {
                        eprintln!("Failed to save new note: {}", e);
                    }

                    self.state.notes.push(new_note);
                    self.state
                        .list_state
                        .select(Some(self.state.notes.len() - 1));
                } else {
                    // If we're renaming an existing note
                    if let Some(note) = self.state.notes.get_mut(selected) {
                        let old_title = note.title.clone();
                        note.title = new_title;
                        note.updated_at = Local::now();

                        // Update in storage
                        if let Err(e) = self.storage.rename_note(&old_title, note) {
                            eprintln!("Failed to rename note: {}", e);
                            // Revert on failure
                            note.title = old_title;
                        }
                    }
                }
            }
            self.state.current_view = View::LivePreview;
        }
    }

    fn cancel_rename(&mut self) {
        self.state.rename_buffer.clear();
        self.state.current_view = View::LivePreview;
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
