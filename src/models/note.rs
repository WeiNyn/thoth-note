use chrono::{DateTime, Local};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style},
    symbols,
    text::{Line, Span},
    widgets::{Block, Paragraph, Widget, Wrap},
};

#[derive(Debug, Clone)]
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

impl Widget for Note {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (fg_color, border_style) = if self.selected {
            (Color::Green, Style::default().fg(Color::Green))
        } else {
            (Color::Gray, Style::default().fg(Color::Gray))
        };

        Paragraph::new(vec![
            Line::from(self.content.as_str()).style(Style::default().fg(fg_color))
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
