use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub struct Areas {
    pub note_list: Rect,
    pub main: Rect,
}

pub fn create_layout(area: Rect) -> Areas {
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(80),
        ].as_ref())
        .split(area);
        
    Areas {
        note_list: columns[0],
        main: columns[1],
    }
}
