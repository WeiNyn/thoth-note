use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::app::AppState;

pub fn render_delete_confirm(frame: &mut Frame, state: &mut AppState, area: Rect) {
    // Calculate dialog dimensions
    let width = 50;
    let height = 3;
    let x = (area.width.saturating_sub(width)) / 2;
    let y = (area.height.saturating_sub(height)) / 2;
    let dialog_area = Rect::new(x, y, width, height);

    // Get current note title
    let title = state
        .list_state
        .selected
        .and_then(|i| state.notes.get(i))
        .map(|note| note.title.as_str())
        .unwrap_or("this note");

    // Create confirmation message
    let message = format!("Delete '{}'? (Enter to confirm, Esc to cancel)", title);

    // Render dialog box
    frame.render_widget(Clear, dialog_area); // Clear the background
    frame.render_widget(
        Paragraph::new(message)
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Red)),
        dialog_area,
    );
}
