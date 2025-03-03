use ratatui::style::{Color, Modifier, Style};

pub struct AppTheme {
    pub background: Color,
    pub foreground: Color,
    pub accent: Color,
    pub warning: Color,
    pub error: Color,
    pub info: Color,
    pub success: Color,

    // UI component styles
    pub selected_style: Style,
    pub normal_style: Style,
    pub title_style: Style,
    pub header_style: Style,
}

impl Default for AppTheme {
    fn default() -> Self {
        AppTheme {
            background: Color::Reset,
            foreground: Color::Gray,
            accent: Color::Yellow,
            warning: Color::Yellow,
            error: Color::Red,
            info: Color::Blue,
            success: Color::Green,

            selected_style: Style::default().fg(Color::Green),
            normal_style: Style::default().fg(Color::Gray),
            title_style: Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
            header_style: Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        }
    }
}

impl AppTheme {
    pub fn dark() -> Self {
        Self::default()
    }

    pub fn light() -> Self {
        Self {
            background: Color::White,
            foreground: Color::Black,
            accent: Color::Blue,
            warning: Color::Yellow,
            error: Color::Red,
            info: Color::Cyan,
            success: Color::Green,

            selected_style: Style::default().fg(Color::Blue),
            normal_style: Style::default().fg(Color::Black),
            title_style: Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),
            header_style: Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        }
    }
}
