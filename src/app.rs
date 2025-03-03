use chrono::Local;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use edtui::{EditorEventHandler, EditorState};
use edtui_jagged::Jagged;
use ratatui::{DefaultTerminal, Frame};
use tui_widget_list::ListState;

use crate::commands::Command;
use crate::models::note::Note;
use crate::theme::AppTheme;
use crate::ui;

pub struct AppState {
    pub notes: Vec<Note>,
    pub list_state: ListState,
    pub editor_state: EditorState,
    pub current_view: View,
    pub theme: AppTheme,
}

pub enum View {
    List,
    Editor,
    Preview,
}

pub struct App {
    state: AppState,
    editor_event_handler: EditorEventHandler,
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
            current_view: View::List,
            theme: AppTheme::default(),
        }
    }
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        let mut state = AppState::default();
        // Initialize with example notes
        state.notes = Self::create_example_notes();

        // Set initial editor content
        if !state.notes.is_empty() {
            let content = state.notes[0].content.clone();
            state.editor_state.lines = Jagged::from(content);
        }

        Self {
            state,
            editor_event_handler: EditorEventHandler::default(),
            running: false,
        }
    }

    fn create_example_notes() -> Vec<Note> {
        (1..=10)
            .map(|i| Note {
                title: format!("Note {}", i),
                content: format!("Content {}", i),
                created_at: Local::now(),
                updated_at: Local::now(),
                selected: false,
            })
            .collect()
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        // Initialize app state
        if self.state.notes.is_empty() {
            let mut new_note = Note::default();
            new_note.title = "New Note".to_string();
            self.state.notes.push(new_note);
            self.state.editor_state.lines = Jagged::from("Start your note here.");
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

    /// Handle events
    fn handle_events(&mut self) -> Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                if let Some(command) = self.key_to_command(key) {
                    self.execute_command(command);
                } else {
                    // Pass event to editor if we're in editor view
                    match self.state.current_view {
                        View::Editor => {
                            self.editor_event_handler
                                .on_event(Event::Key(key), &mut self.state.editor_state);
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }

    fn key_to_command(&self, key: KeyEvent) -> Option<Command> {
        match (key.modifiers, key.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('q')) => Some(Command::Quit),
            (KeyModifiers::CONTROL, KeyCode::Down) => Some(Command::NextNote),
            (KeyModifiers::CONTROL, KeyCode::Up) => Some(Command::PreviousNote),
            (KeyModifiers::CONTROL, KeyCode::Char('e')) => Some(Command::SwitchView(View::Editor)),
            (KeyModifiers::CONTROL, KeyCode::Char('l')) => Some(Command::SwitchView(View::List)),
            (KeyModifiers::CONTROL, KeyCode::Char('p')) => Some(Command::SwitchView(View::Preview)),
            (KeyModifiers::CONTROL, KeyCode::Char('n')) => Some(Command::NewNote),
            (KeyModifiers::CONTROL, KeyCode::Char('s')) => Some(Command::SaveNote),
            (KeyModifiers::CONTROL, KeyCode::Char('d')) => Some(Command::DeleteNote),
            _ => None,
        }
    }

    fn execute_command(&mut self, command: Command) {
        match command {
            Command::Quit => self.quit(),
            Command::NextNote => self.select_next_note(),
            Command::PreviousNote => self.select_previous_note(),
            Command::SwitchView(view) => self.state.current_view = view,
            Command::NewNote => self.create_new_note(),
            Command::SaveNote => self.save_current_note(),
            Command::DeleteNote => self.delete_current_note(),
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
        let mut new_note = Note::default();
        new_note.title = format!("New Note {}", self.state.notes.len() + 1);
        new_note.content = "Start writing...".to_string();
        new_note.created_at = Local::now();
        new_note.updated_at = Local::now();

        self.state.notes.push(new_note);
        self.state
            .list_state
            .select(Some(self.state.notes.len() - 1));
        self.load_note_to_editor(self.state.notes.len() - 1);
    }

    fn save_current_note(&mut self) {
        self.save_editor_content_to_current_note();
        // Here we would add actual persistence logic
    }

    fn delete_current_note(&mut self) {
        if let Some(selected) = self.state.list_state.selected {
            if !self.state.notes.is_empty() {
                self.state.notes.remove(selected);

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

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }
}
