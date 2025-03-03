mod editor;
mod layout;
mod note_list;
mod preview;

use ratatui::Frame;

use crate::app::{AppState, View};

pub use editor::render_editor;
use layout::create_layout;
pub use note_list::render_note_list;
pub use preview::render_preview;

pub fn render(frame: &mut Frame, state: &mut AppState) {
    let areas = create_layout(frame.area());

    // Render the different components
    render_note_list(frame, state, areas.note_list);

    match state.current_view {
        View::Editor => render_editor(frame, state, areas.main),
        View::Preview => render_preview(frame, state, areas.main),
        View::List => {} // List is always shown
    }

    // Add help/status bar if needed
}
