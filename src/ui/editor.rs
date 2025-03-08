use edtui::{EditorStatusLine, EditorView, SyntaxHighlighter};
use ratatui::{
    layout::{Alignment, Rect},
    style::Style,
    symbols,
    text::Span,
    widgets::{Block, Borders},
    Frame,
};

use crate::{app::AppState, theme::palette};

pub fn render_editor(frame: &mut Frame, state: &mut AppState, area: Rect) {
    let syntax_highlighter = SyntaxHighlighter::new("ayu-dark", "markdown");
    let editor = EditorView::new(&mut state.editor_state)
        .syntax_highlighter(Some(syntax_highlighter))
        .wrap(true)
        .theme(
            edtui::EditorTheme::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(state.theme.selected_style)
                        .border_set(symbols::border::ROUNDED)
                        .title(Span::styled("Editor", state.theme.title_style))
                        .title_alignment(Alignment::Center),
                )
                .base(Style::default().bg(palette::BASE).fg(palette::OVERLAY0))
                .status_line(
                    EditorStatusLine::default()
                        .style_text(Style::default().fg(palette::ROSEWATER))
                        .style_line(Style::default().fg(palette::ROSEWATER)),
                ),
        );
    frame.render_widget(editor, area);
}
