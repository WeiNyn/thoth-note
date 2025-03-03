use ratatui::{
    layout::{Alignment, Rect},
    symbols,
    text::Span,
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::AppState;

pub fn render_preview(frame: &mut Frame, state: &mut AppState, area: Rect) {
    let selected = state.list_state.selected.unwrap_or(0);
    if let Some(note) = state.notes.get(selected) {
        let content = note.content.clone();

        let preview = Paragraph::new(content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(state.theme.normal_style)
                    .border_set(symbols::border::ROUNDED)
                    .title(Span::styled("Preview", state.theme.title_style))
                    .title_alignment(Alignment::Center),
            )
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        frame.render_widget(preview, area);
    }
}
