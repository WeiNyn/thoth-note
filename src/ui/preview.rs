use crate::theme::palette;
use ratatui::{
    layout::{Alignment, Margin, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols::{self},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarState, Wrap},
    Frame,
};
use std::{sync::LazyLock, vec};

use crate::app::AppState;
use ansi_to_tui::IntoText;
use itertools::{Itertools, Position};
use pulldown_cmark::{
    BlockQuoteKind, CodeBlockKind, CowStr, Event, HeadingLevel, LinkType, Options, Parser, Tag,
    TagEnd,
};
use syntect::{
    easy::HighlightLines,
    highlighting::ThemeSet,
    parsing::SyntaxSet,
    util::{as_24_bit_terminal_escaped, LinesWithEndings},
};
use tracing::{debug, instrument, warn};

pub fn render_preview(frame: &mut Frame, state: &mut AppState, area: Rect) {
    let content = state
        .editor_state
        .lines
        .flatten(&Some('\n'))
        .iter()
        .map(|row| row.to_string())
        .collect::<Vec<String>>()
        .join("");

    let selected = state.list_state.selected.unwrap_or(0);

    if let Some(note) = state.notes.get(selected) {
        let area_width = area.width;
        let text = from_str(&content, area_width);
        let paragraph = Paragraph::new(text)
            .alignment(Alignment::Left)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_set(symbols::border::ROUNDED)
                    .border_style(Style::default().fg(palette::TEAL))
                    .title(note.title.as_str())
                    .title_style(
                        Style::default()
                            .fg(palette::MAROON)
                            .add_modifier(Modifier::BOLD),
                    )
                    .title_alignment(Alignment::Center),
            )
            .scroll((state.preview_scroll_offset as u16, 0))
            .wrap(Wrap { trim: false });
        let line_count = paragraph.line_count(area.width);
        if state.preview_scroll_offset > line_count {
            state.preview_scroll_offset = line_count;
        }

        let scrollbar = Scrollbar::new(ratatui::widgets::ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"));

        let mut scrollbar_state = ScrollbarState::new(paragraph.line_count(area.width))
            .position(state.preview_scroll_offset);

        frame.render_widget(paragraph, area);
        frame.render_stateful_widget(
            scrollbar,
            area.inner(Margin {
                vertical: 1,
                horizontal: 0,
            }),
            &mut scrollbar_state,
        );
    }
}

pub fn from_str(input: &str, area_width: u16) -> Text {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(input, options);
    let mut writer = TextWriter::new(parser, area_width);
    writer.run();
    writer.text
}

struct TextWriter<'a, I> {
    /// Iterator supplying events.
    iter: I,

    /// Text to write to.
    text: Text<'a>,

    /// Current style.
    ///
    /// This is a stack of styles, with the top style being the current style.
    inline_styles: Vec<Style>,

    /// Prefix to add to the start of the each line.
    line_prefixes: Vec<Span<'a>>,

    /// Stack of line styles.
    line_styles: Vec<Style>,

    /// Used to highlight code blocks, set when  a codeblock is encountered
    code_highlighter: Option<HighlightLines<'a>>,

    /// Current list index as a stack of indices.
    list_indices: Vec<Option<u64>>,

    /// A link which will be appended to the current line when the link tag is closed.
    link: Option<CowStr<'a>>,

    needs_newline: bool,

    area_width: u16,
}

static SYNTAX_SET: LazyLock<SyntaxSet> = LazyLock::new(SyntaxSet::load_defaults_newlines);
static THEME_SET: LazyLock<ThemeSet> = LazyLock::new(ThemeSet::load_defaults);

impl<'a, I> TextWriter<'a, I>
where
    I: Iterator<Item = Event<'a>>,
{
    fn new(iter: I, area_width: u16) -> Self {
        Self {
            iter,
            text: Text::default(),
            inline_styles: vec![],
            line_styles: vec![],
            line_prefixes: vec![],
            list_indices: vec![],
            needs_newline: false,
            code_highlighter: None,
            link: None,
            area_width,
        }
    }

    fn run(&mut self) {
        debug!("Running text writer");
        while let Some(event) = self.iter.next() {
            self.handle_event(event);
        }
    }

    #[instrument(level = "debug", skip(self))]
    fn handle_event(&mut self, event: Event<'a>) {
        match event {
            Event::Start(tag) => self.start_tag(tag),
            Event::End(tag) => self.end_tag(tag),
            Event::Text(text) => self.text(text),
            Event::Code(code) => self.code(code),
            Event::Html(_html) => warn!("Html not yet supported"),
            Event::InlineHtml(_html) => warn!("Inline html not yet supported"),
            Event::FootnoteReference(_) => warn!("Footnote reference not yet supported"),
            Event::SoftBreak => self.soft_break(),
            Event::HardBreak => self.hard_break(),
            Event::Rule => self.rule(),
            Event::TaskListMarker(_) => warn!("Task list marker not yet supported"),
            Event::InlineMath(_) => warn!("Inline math not yet supported"),
            Event::DisplayMath(_) => warn!("Display math not yet supported"),
        }
    }

    fn start_tag(&mut self, tag: Tag<'a>) {
        match tag {
            Tag::Paragraph => self.start_paragraph(),
            Tag::Heading { level, .. } => self.start_heading(level),
            Tag::BlockQuote(kind) => self.start_blockquote(kind),
            Tag::CodeBlock(kind) => self.start_codeblock(kind),
            Tag::HtmlBlock => warn!("Html block not yet supported"),
            Tag::List(start_index) => self.start_list(start_index),
            Tag::Item => self.start_item(),
            Tag::FootnoteDefinition(_) => warn!("Footnote definition not yet supported"),
            Tag::Table(_) => warn!("Table not yet supported"),
            Tag::TableHead => warn!("Table head not yet supported"),
            Tag::TableRow => warn!("Table row not yet supported"),
            Tag::TableCell => warn!("Table cell not yet supported"),
            Tag::Emphasis => self.push_inline_style(Style::new().italic().fg(palette::SUBTEXT1)),
            Tag::Strong => self.push_inline_style(Style::new().fg(palette::LAVENDER)),
            Tag::Strikethrough => {
                self.push_inline_style(Style::new().crossed_out().fg(palette::MAROON))
            }
            Tag::Link {
                link_type,
                dest_url,
                title,
                ..
            } => self.push_link(link_type, dest_url, title),
            Tag::Image { .. } => warn!("Image not yet supported"),
            Tag::MetadataBlock(_) => warn!("Metadata block not yet supported"),
            Tag::DefinitionList => warn!("Definition list not yet supported"),
            Tag::DefinitionListTitle => warn!("Definition list title not yet supported"),
            Tag::DefinitionListDefinition => warn!("Definition list definition not yet supported"),
            _ => warn!("Tag not yet supported: {:?}", tag),
        }
    }

    fn end_tag(&mut self, tag: TagEnd) {
        match tag {
            TagEnd::Paragraph => self.end_paragraph(),
            TagEnd::Heading(_) => self.end_heading(),
            TagEnd::BlockQuote(_) => self.end_blockquote(),
            TagEnd::CodeBlock => self.end_codeblock(),
            TagEnd::HtmlBlock => {}
            TagEnd::List(_is_ordered) => self.end_list(),
            TagEnd::Item => {}
            TagEnd::FootnoteDefinition => {}
            TagEnd::Table => {}
            TagEnd::TableHead => {}
            TagEnd::TableRow => {}
            TagEnd::TableCell => {}
            TagEnd::Emphasis => self.pop_inline_style(),
            TagEnd::Strong => self.pop_inline_style(),
            TagEnd::Strikethrough => self.pop_inline_style(),
            TagEnd::Link => self.pop_link(),
            TagEnd::Image => {}
            TagEnd::MetadataBlock(_) => {}
            TagEnd::DefinitionList => {}
            TagEnd::DefinitionListTitle => {}
            TagEnd::DefinitionListDefinition => {}
            _ => warn!("Tag end not yet supported: {:?}", tag),
        }
    }

    fn start_paragraph(&mut self) {
        // Insert an empty line between paragraphs if there is at least one line of text already.
        if self.needs_newline {
            self.push_line(Line::default());
        }
        self.push_line(Line::default());
        self.needs_newline = false;
    }

    fn end_paragraph(&mut self) {
        self.needs_newline = true
    }

    fn start_heading(&mut self, level: HeadingLevel) {
        if self.needs_newline {
            self.push_line(Line::default());
        }
        let style = match level {
            HeadingLevel::H1 => styles::H1,
            HeadingLevel::H2 => styles::H2,
            HeadingLevel::H3 => styles::H3,
            HeadingLevel::H4 => styles::H4,
            HeadingLevel::H5 => styles::H5,
            HeadingLevel::H6 => styles::H6,
        };
        let content = format!("{} ", "▌".repeat(level as usize));
        self.push_line(Line::styled(content, style));
        self.needs_newline = false;
    }

    fn end_heading(&mut self) {
        self.needs_newline = true;
    }

    fn start_blockquote(&mut self, kind: Option<BlockQuoteKind>) {
        if self.needs_newline {
            self.push_line(Line::default());
            self.needs_newline = false;
        }
        match kind {
            None => {
                self.line_prefixes.push(Span::from("▌ "));
                self.line_styles
                    .push(Style::new().fg(Color::Rgb(166, 218, 149)));
            }
            Some(BlockQuoteKind::Note) | Some(BlockQuoteKind::Tip) => {
                self.line_prefixes.push(Span::from("▌✎ "));
                self.line_styles
                    .push(Style::new().fg(Color::Rgb(139, 213, 202)));
            }
            Some(BlockQuoteKind::Warning) => {
                self.line_prefixes.push(Span::from("▌⚠ "));
                self.line_styles
                    .push(Style::new().fg(Color::Rgb(245, 169, 127)));
            }
            Some(BlockQuoteKind::Caution) => {
                self.line_prefixes.push(Span::from("▌✖ "));
                self.line_styles
                    .push(Style::new().fg(Color::Rgb(238, 153, 160)));
            }
            Some(BlockQuoteKind::Important) => {
                self.line_prefixes.push(Span::from("▌🔥 "));
                self.line_styles
                    .push(Style::new().fg(Color::Rgb(245, 169, 127)));
            }
        }
    }

    fn end_blockquote(&mut self) {
        self.line_prefixes.pop();
        self.line_styles.pop();
        self.needs_newline = true;
    }

    fn text(&mut self, text: CowStr<'a>) {
        if let Some(highlighter) = &mut self.code_highlighter {
            let text: Text = LinesWithEndings::from(&text)
                .filter_map(|line| highlighter.highlight_line(line, &SYNTAX_SET).ok())
                .filter_map(|part| as_24_bit_terminal_escaped(&part, false).into_text().ok())
                .flatten()
                .collect();

            for line in text.lines {
                let mut prefixed_line = line;
                if let Some(prefix) = self.line_prefixes.last() {
                    prefixed_line.spans.insert(0, prefix.clone());
                }
                self.text.push_line(prefixed_line);
            }
            self.needs_newline = false;
            return;
        }

        for (position, line) in text.lines().with_position() {
            if self.needs_newline {
                self.push_line(Line::default());
                self.needs_newline = false;
            }

            if matches!(position, Position::Middle | Position::Last) {
                self.push_line(Line::default());
            }

            let style = self.inline_styles.last().copied().unwrap_or_default();

            let span = Span::styled(line.to_owned(), style);

            self.push_span(span);
        }
        self.needs_newline = false;
    }

    fn code(&mut self, code: CowStr<'a>) {
        let span = Span::styled(code, styles::CODE);
        self.push_span(span);
    }

    fn rule(&mut self) {
        self.push_line(Line::from("─".repeat(self.area_width as usize - 2)));
    }

    fn hard_break(&mut self) {
        self.push_line(Line::default());
    }

    fn start_list(&mut self, index: Option<u64>) {
        if self.list_indices.is_empty() && self.needs_newline {
            self.push_line(Line::default());
        }
        self.list_indices.push(index);
    }

    fn end_list(&mut self) {
        self.list_indices.pop();
        self.needs_newline = true;
    }

    fn start_item(&mut self) {
        let list_level = self.list_indices.len();
        let prefix = match list_level {
            1 => "■ ",
            2 => "‣  ",
            _ => "· ",
        };

        self.push_line(Line::default());
        let width = self.list_indices.len() * 4 - 3;
        if let Some(last_index) = self.list_indices.last_mut() {
            let span = match last_index {
                None => Span::from(" ".repeat(width - 1) + prefix),
                Some(index) => {
                    *index += 1;
                    format!("{:width$}. ", *index - 1).light_blue()
                }
            };
            self.push_span(span);
        }
        self.needs_newline = false;
    }

    fn soft_break(&mut self) {
        self.push_line(Line::default());
    }

    fn start_codeblock(&mut self, kind: CodeBlockKind<'_>) {
        if !self.text.lines.is_empty() {
            self.push_line(Line::default());
        }
        let lang = match kind {
            CodeBlockKind::Fenced(ref lang) => lang.as_ref(),
            CodeBlockKind::Indented => "",
        };

        self.line_styles.push(styles::CODE);

        self.set_code_highlighter(lang);

        let mut breaker = String::from("╒══");
        if !lang.is_empty() {
            breaker.push_str(format!(" {} ", lang).as_str());
        } else {
            breaker.push_str("══");
        }
        let filler = "═".repeat(self.area_width as usize - 2 - 5 - lang.len());
        breaker.push_str(&filler);
        self.push_line(Span::from(breaker).into());

        self.line_prefixes.push(Span::from("│"));
        self.needs_newline = true;
    }

    fn end_codeblock(&mut self) {
        self.line_prefixes.pop();
        let mut bottom_breaker = String::from("└");
        bottom_breaker.push_str(&"─".repeat((self.area_width - 3) as usize));
        self.push_line(Span::from(bottom_breaker).into());

        self.needs_newline = true;
        self.line_styles.pop();

        self.clear_code_highlighter();
    }

    #[instrument(level = "trace", skip(self))]
    fn set_code_highlighter(&mut self, lang: &str) {
        if let Some(syntax) = SYNTAX_SET.find_syntax_by_token(lang) {
            debug!("Starting code block with syntax: {:?}", lang);
            let theme = &THEME_SET.themes["base16-ocean.dark"];
            let highlighter = HighlightLines::new(syntax, theme);
            self.code_highlighter = Some(highlighter);
        } else {
            warn!("Could not find syntax for code block: {:?}", lang);
        }
    }

    #[instrument(level = "trace", skip(self))]
    fn clear_code_highlighter(&mut self) {
        self.code_highlighter = None;
    }

    #[instrument(level = "trace", skip(self))]
    fn push_inline_style(&mut self, style: Style) {
        let current_style = self.inline_styles.last().copied().unwrap_or_default();
        let style = current_style.patch(style);
        self.inline_styles.push(style);
        debug!("Pushed inline style: {:?}", style);
        debug!("Current inline styles: {:?}", self.inline_styles);
    }

    #[instrument(level = "trace", skip(self))]
    fn pop_inline_style(&mut self) {
        self.inline_styles.pop();
    }

    #[instrument(level = "trace", skip(self))]
    fn push_line(&mut self, line: Line<'a>) {
        let style = self.line_styles.last().copied().unwrap_or_default();
        let mut line = line.patch_style(style);

        // Add line prefixes to the start of the line.
        let line_prefixes = self.line_prefixes.iter().cloned().collect_vec();
        let has_prefixes = !line_prefixes.is_empty();
        if has_prefixes {
            line.spans.insert(0, " ".into());
        }
        for prefix in line_prefixes.iter().rev().cloned() {
            line.spans.insert(0, prefix);
        }
        self.text.lines.push(line);
    }

    #[instrument(level = "trace", skip(self))]
    fn push_span(&mut self, span: Span<'a>) {
        if let Some(line) = self.text.lines.last_mut() {
            line.push_span(span);
        } else {
            self.push_line(Line::from(vec![span]));
        }
    }

    /// Store the link to be appended to the link text
    #[instrument(level = "trace", skip(self))]
    fn push_link(&mut self, link_type: LinkType, dest_url: CowStr<'a>, title: CowStr<'a>) {
        match link_type {
            LinkType::Autolink => {
                self.link = None;
                self.push_inline_style(Style::default().underlined().fg(palette::BLUE));
            }
            _ => {
                self.link = Some(dest_url);
            }
        }
    }

    /// Append the link to the current line
    #[instrument(level = "trace", skip(self))]
    fn pop_link(&mut self) {
        if let Some(link) = self.link.take() {
            self.push_span(" (".into());
            self.push_span(Span::styled(link, styles::LINK));
            self.push_span(")".into());
        } else {
            self.pop_inline_style();
        }
    }
}

mod styles {
    use ratatui::style::{Modifier, Style};

    use crate::theme::palette;

    pub const H1: Style = Style::new()
        .fg(palette::PEACH)
        .add_modifier(Modifier::BOLD)
        .add_modifier(Modifier::UNDERLINED);
    pub const H2: Style = Style::new()
        .fg(palette::YELLOW)
        .add_modifier(Modifier::BOLD)
        .add_modifier(Modifier::UNDERLINED);
    pub const H3: Style = Style::new()
        .fg(palette::GREEN)
        .add_modifier(Modifier::BOLD)
        .add_modifier(Modifier::ITALIC);
    pub const H4: Style = Style::new()
        .fg(palette::TEAL)
        .add_modifier(Modifier::ITALIC);
    pub const H5: Style = Style::new()
        .fg(palette::TEAL)
        .add_modifier(Modifier::ITALIC);
    pub const H6: Style = Style::new()
        .fg(palette::TEAL)
        .add_modifier(Modifier::ITALIC);
    pub const CODE: Style = Style::new().fg(palette::FLAMINGO);
    pub const LINK: Style = Style::new()
        .fg(palette::BLUE)
        .add_modifier(Modifier::UNDERLINED);
}
