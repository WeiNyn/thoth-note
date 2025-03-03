use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    text::Span,
    widgets::{Block, Borders},
    symbols,
};
use edtui::{EditorView, SyntaxHighlighter};

use crate::app::AppState;

pub fn render_editor(frame: &mut Frame, state: &mut AppState, area: Rect) {
    let syntax_highlighter = SyntaxHighlighter::new("ayu-dark", "markdown");
    let editor = EditorView::new(&mut state.editor_state)
        .syntax_highlighter(Some(syntax_highlighter))
        .wrap(true)
        .theme(
            edtui::EditorTheme::default().block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(state.theme.selected_style)
                    .border_set(symbols::border::ROUNDED)
                    .title(Span::styled("Editor", state.theme.title_style))
                    .title_alignment(Alignment::Center),
            ),
        );
    frame.render_widget(editor, area);
}
