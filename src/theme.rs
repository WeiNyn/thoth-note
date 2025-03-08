use ratatui::style::{Color, Modifier, Style};

pub mod palette {
    use ratatui::style::Color;

    pub const ROSEWATER: Color = Color::Rgb(244, 219, 214);
    pub const FLAMINGO: Color = Color::Rgb(240, 198, 198);
    pub const PINK: Color = Color::Rgb(245, 189, 230);
    pub const MAUVE: Color = Color::Rgb(198, 160, 246);
    pub const RED: Color = Color::Rgb(237, 135, 150);
    pub const MAROON: Color = Color::Rgb(238, 153, 160);
    pub const PEACH: Color = Color::Rgb(245, 169, 127);
    pub const YELLOW: Color = Color::Rgb(238, 212, 159);
    pub const GREEN: Color = Color::Rgb(166, 218, 149);
    pub const TEAL: Color = Color::Rgb(139, 213, 202);
    pub const SKY: Color = Color::Rgb(145, 215, 227);
    pub const SAPPHIRE: Color = Color::Rgb(125, 196, 228);
    pub const BLUE: Color = Color::Rgb(138, 173, 244);
    pub const LAVENDER: Color = Color::Rgb(183, 189, 248);
    pub const TEXT: Color = Color::Rgb(202, 211, 245);
    pub const SUBTEXT1: Color = Color::Rgb(184, 192, 224);
    pub const SUBTEXT0: Color = Color::Rgb(165, 173, 203);
    pub const OVERLAY2: Color = Color::Rgb(147, 154, 183);
    pub const OVERLAY1: Color = Color::Rgb(128, 135, 162);
    pub const OVERLAY0: Color = Color::Rgb(110, 115, 141);
    pub const SURFACE2: Color = Color::Rgb(91, 96, 120);
    pub const SURFACE1: Color = Color::Rgb(73, 77, 100);
    pub const SURFACE0: Color = Color::Rgb(54, 58, 79);
    pub const BASE: Color = Color::Rgb(36, 39, 58);
    pub const MANTLE: Color = Color::Rgb(30, 32, 48);
    pub const CRUST: Color = Color::Rgb(24, 25, 38);
}

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

            selected_style: Style::default().fg(palette::GREEN),
            normal_style: Style::default().fg(palette::TEXT),
            title_style: Style::default()
                .fg(palette::YELLOW)
                .add_modifier(Modifier::BOLD),
            header_style: Style::default()
                .fg(palette::SKY)
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
