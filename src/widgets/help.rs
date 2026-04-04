//! This widget draws the help window.
//!
//! Example:
//!
//! ```
//! let help = HelpWidget::new();
//! frame.render_widget(help, inner[0]);
//! ```
use ratatui::{
    prelude::{Buffer, Rect, Stylize},
    style::Style,
    symbols::border,
    text::{Line, Span},
    widgets::{Block, Clear, Padding, Paragraph, Widget},
};

use crate::util;

/// A HelpWidget.
pub struct HelpWidget {}

impl HelpWidget {
    /// Construct a new HelpWidget.
    pub fn new() -> Self {
        HelpWidget {}
    }
}

impl Widget for HelpWidget {
    /// Render this HelpWidget.
    fn render(self, area: Rect, buf: &mut Buffer) {
        Widget::render(&self, area, buf);
    }
}

impl Widget for &HelpWidget {
    /// Render this HelpWidget.
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title(Line::from(" Help ".bold()))
            .title_bottom(Line::from(" Press any key ").centered())
            .padding(Padding::new(2, 2, 1, 1))
            .border_set(border::THICK);

        let s = Style::new().fg(util::rgb6_to_indexed(3, 3, 5));

        let text = vec![
            Line::styled("y k u", s),
            " \\|/".into(),
            Line::from(vec![
                Span::styled("h", s),
                Span::raw("-+-"),
                Span::styled("l", s),
                Span::raw("  Cursor movement"),
            ]),
            " /|\\".into(),
            Line::from(vec![
                Span::styled("b j n", s),
                Span::raw("  (or arrow keys)"),
            ]),
            "".into(),
            Line::from(vec![
                Span::styled("t", s),
                Span::raw(": Toggle cell       "),
                Span::styled("s", s),
                Span::raw(": Step"),
            ]),
            Line::from(vec![
                Span::styled("r", s),
                Span::raw(": Run start/stop    "),
                Span::styled("c", s),
                Span::raw(": Clear screen"),
            ]),
            Line::from(vec![
                Span::styled("R", s),
                Span::raw(": Randomize         "),
                Span::styled("?", s),
                Span::raw(": Help"),
            ]),
            Line::from(vec![
                Span::styled("a", s),
                Span::raw(": About             "),
                Span::styled("q", s),
                Span::raw(": Help"),
            ]),
            "".into(),
            Line::from(vec![
                Span::raw("Numeric count followed by \""),
                Span::styled("h", s),
                Span::raw("\" draws a"),
            ]),
            "horizontal line.".into(),
        ];

        let paragraph = Paragraph::new(text).block(block);

        Widget::render(Clear, area, buf);

        paragraph.render(area, buf);
    }
}
