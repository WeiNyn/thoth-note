use ratatui::{layout::Rect, widgets::{Block, BorderType, Borders}, Frame};
use tui_widget_list::{ListBuilder, ListView};

use crate::app::AppState;

pub fn render_note_list(frame: &mut Frame, state: &mut AppState, area: Rect) {
    let builder = ListBuilder::new(|context| {
        let mut note = state.notes[context.index].clone();
        note.selected = context.is_selected;
        (note, 3)
    });

    let note_count = state.notes.len();
    let block = Block::default()
    .title_top("<Ctrl-↑/↓>")
    .title_bottom("<Ctrl-N/R/D/S>")
    .borders(Borders::ALL)
    .border_type(BorderType::Rounded);

    let list = ListView::new(builder, note_count)
        .infinite_scrolling(true)
        .block(block)
        .scroll_axis(tui_widget_list::ScrollAxis::Vertical);

    frame.render_stateful_widget(list, area, &mut state.list_state);
}
