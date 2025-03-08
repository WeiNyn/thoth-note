use ratatui::layout::{Constraint, Direction, Layout, Rect};

use crate::app::View;

pub struct Areas {
    pub note_list: Rect,
    pub preview: Option<Rect>,
    pub editor: Option<Rect>,
}

pub fn create_layout(area: Rect, view: View) -> Areas {
    if let View::LivePreview = view {
        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(40),
                Constraint::Percentage(40),
            ].as_ref())
            .split(area);

        return Areas {
            note_list: columns[0],
            preview: Some(columns[2]),
            editor: Some(columns[1]),
        };
    }
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(80),
        ].as_ref())
        .split(area);
    
    match view {
        View::Editor => Areas {
            note_list: columns[0],
            preview: None,
            editor: Some(columns[1]),
        },
        View::Preview => Areas {
            note_list: columns[0],
            preview: Some(columns[1]),
            editor: None,
        },
        _ => Areas {
            note_list: columns[0],
            preview: Some(columns[1]),
            editor: None,
        },
    }
}
