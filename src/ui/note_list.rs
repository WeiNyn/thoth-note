use ratatui::{layout::Rect, Frame};
use tui_widget_list::{ListBuilder, ListView};

use crate::app::AppState;

pub fn render_note_list(frame: &mut Frame, state: &mut AppState, area: Rect) {
    let builder = ListBuilder::new(|context| {
        let mut note = state.notes[context.index].clone();
        note.selected = context.is_selected;
        (note, 3)
    });

    let note_count = state.notes.len();
    let list = ListView::new(builder, note_count)
        .infinite_scrolling(true)
        .scroll_axis(tui_widget_list::ScrollAxis::Vertical);

    frame.render_stateful_widget(list, area, &mut state.list_state);
}
