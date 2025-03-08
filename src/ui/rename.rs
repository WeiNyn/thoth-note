use ratatui::{
    layout::{Alignment, Margin, Rect},
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

use crate::app::AppState;

/// Renders the rename dialog
pub fn render_rename(frame: &mut Frame, state: &mut AppState, area: Rect) {
    let block = Block::default()
        .title("Enter Note Name")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::Yellow));

    let width = 50;
    let height = 3;
    let x = (area.width - width) / 2;
    let y = (area.height - height) / 2;

    let popup_area = Rect::new(
        area.x + x,
        area.y + y,
        width.min(area.width),
        height.min(area.height),
    );

    // Clear the background
    frame.render_widget(Clear, popup_area);
    frame.render_widget(block.clone(), popup_area);

    let text = Text::from(format!("> {}", state.rename_buffer));
    let input = Paragraph::new(text)
        .style(Style::default().add_modifier(Modifier::BOLD))
        .block(block)
        .alignment(Alignment::Left);

    frame.render_widget(input, popup_area.inner(Margin::default()));
}
