use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style},
    symbols,
    text::{Line, Span},
    widgets::{Block, Paragraph, Widget, Wrap},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub selected: bool,
}

impl Default for Note {
    fn default() -> Self {
        Self {
            title: String::new(),
            content: String::new(),
            created_at: Local::now(),
            updated_at: Local::now(),
            selected: false,
        }
    }
}

fn get_created_string(note: &Note) -> String {
    let now = Local::now();
    let duration = now.signed_duration_since(note.created_at);
    if duration.num_seconds() < 60 {
        "just now".to_string()
    } else if duration.num_minutes() < 60 {
        format!("{}m ago", duration.num_minutes())
    } else if duration.num_hours() < 24 {
        format!("{}h ago", duration.num_hours())
    } else if duration.num_days() < 7 {
        format!("{}d ago", duration.num_days())
    } else {
        note.updated_at.format("%Y-%m-%d").to_string()
    }
}

impl Widget for Note {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (fg_color, border_style) = if self.selected {
            (Color::Green, Style::default().fg(Color::Green))
        } else {
            (Color::Gray, Style::default().fg(Color::Gray))
        };

        let created_string = get_created_string(&self);

        Paragraph::new(vec![
            Line::from(created_string).style(Style::default().fg(fg_color))
        ])
        .block(
            Block::bordered()
                .border_style(border_style)
                .border_set(symbols::border::ROUNDED)
                .title(
                    Span::styled(self.title.as_str(), Style::default().fg(fg_color))
                        .into_centered_line(),
                )
                .padding(ratatui::widgets::Padding::left(1)),
        )
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true })
        .render(area, buf);
    }
}
