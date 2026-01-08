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
    symbols::border,
    text::Line,
    widgets::{Block, Clear, Padding, Paragraph, Widget},
};

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
            .padding(Padding::uniform(1))
            .border_set(border::THICK);

        let text = vec![
            "y k u".into(),
            " \\|/".into(),
            "h-+-l  Cursor movement".into(),
            " /|\\".into(),
            "b j n  (or arrow keys)".into(),
            "".into(),
            "t: Toggle cell".into(),
            "s: Step".into(),
            "r: Run start/stop".into(),
            "c: Clear screen".into(),
            "R: Randomize".into(),
            "q: Quit".into(),
        ];

        let paragraph = Paragraph::new(text).block(block);

        Widget::render(Clear, area, buf);

        paragraph.render(area, buf);
    }
}
