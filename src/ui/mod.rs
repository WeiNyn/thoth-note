mod editor;
mod layout;
mod note_list;
mod preview;
mod rename;
mod delete_confirm;

use ratatui::Frame;

use crate::app::{AppState, View};

pub use editor::render_editor;
use layout::create_layout;
pub use note_list::render_note_list;
pub use preview::render_preview;
pub use rename::render_rename;
pub use delete_confirm::render_delete_confirm;

pub fn render(frame: &mut Frame, state: &mut AppState) {
    let areas = create_layout(frame.area(), state.current_view);

    // Render the different components
    render_note_list(frame, state, areas.note_list);

    match state.current_view {
        View::Editor => render_editor(frame, state, areas.editor.unwrap()),
        View::Preview => render_preview(frame, state, areas.preview.unwrap()),
        View::List => {} // List is always shown
        View::Rename => render_rename(frame, state, frame.area()),
        View::LivePreview => {
            render_editor(frame, state, areas.editor.unwrap());
            render_preview(frame, state, areas.preview.unwrap())
        }
        View::DeleteConfirm => render_delete_confirm(frame, state, frame.area()),
    }

    // Add help/status bar if needed
}
