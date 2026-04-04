use crate::windows::WindowDrawResult;

use ratatui::widgets::{Clear, Paragraph, Wrap};

/// Window to show the TooSmallWidget.
pub struct TooSmallWindow;

impl TooSmallWindow {
    /// Make a new TooSmallWindow.
    pub fn new() -> Self {
        Self {}
    }

    /// Draw the TooSmall Window.
    pub fn draw(&mut self, frame: &mut ratatui::Frame) -> Option<WindowDrawResult> {
        frame.render_widget(Clear, frame.area());

        let paragraph = Paragraph::new(vec![
            "Your terminal is too small!".into(),
            "".into(),
            "Make it larger!".into(),
        ])
        .wrap(Wrap { trim: true });

        frame.render_widget(paragraph, frame.area());

        None
    }
}
