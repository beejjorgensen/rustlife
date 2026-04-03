use crate::{
    AppCommand, AppEvent, AppEventType, util,
    windows::{Window, WindowDrawResult},
};
use crossterm::event::{Event, KeyEventKind};
use ratatui::{
    layout::Alignment,
    prelude::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Clear, Padding, Paragraph, Wrap},
};

/// Window to show the AboutWidget.
pub struct AboutWindow;

impl AboutWindow {
    pub fn new() -> Self {
        Self {}
    }
}

impl Window for AboutWindow {
    /// Draw the About Window.
    fn draw(&mut self, frame: &mut ratatui::Frame) -> Option<WindowDrawResult> {
        let area = util::centered_area(40, 12, frame);

        let text = vec![
            "A Rust/Ratatui implementation of Conway's Game of Life"
                .bold()
                .into(),
            "".into(),
            "Written by Beej Jorgensen <beej@beej.us>".into(),
            "".into(),
            "Released to the public domain under the terms of the Unlicense".into(),
        ];

        let about_block = Block::bordered()
            .title(Line::from(" About ".bold()))
            .title_bottom(Line::from(" Press any key ").centered())
            .padding(Padding::new(2, 2, 1, 1))
            .border_set(border::THICK);

        let paragraph = Paragraph::new(text)
            .block(about_block)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        frame.render_widget(Clear, area);
        frame.render_widget(paragraph, area);

        Some(WindowDrawResult::cursor_hide())
    }

    /// Handle app events for the About Window.
    fn handle_app_event(&mut self, app_event: &mut AppEvent) -> Option<AppCommand> {
        match &app_event.event_type {
            AppEventType::Event(e) => match e {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    app_event.propagate = false;
                    Some(AppCommand::CloseChildWindow)
                }

                _ => None,
            },

            _ => None,
        }
    }
}
