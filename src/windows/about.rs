use crate::{
    AppCommand, AppEvent, AppEventType, util,
    windows::{Window, WindowDrawResult},
};
use crossterm::event::{Event, KeyEventKind};
use ratatui::{
    layout::Alignment,
    prelude::Stylize,
    style::Style,
    symbols::border,
    text::{Line, Span},
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
        let area = util::centered_area(40, 16, frame);

        let text = vec![
            Line::from(vec![
                Span::raw("First"),
                Span::styled("line", Style::new().green().italic()),
                ".".into(),
            ]),
            Line::from("Second line".red()),
            "Third line".into(),
        ];

        let about_block = Block::bordered()
            .title(Line::from(" About ".bold()))
            .title_bottom(Line::from(" Press any key ").centered())
            .padding(Padding::uniform(1))
            .border_set(border::THICK);

        let paragraph = Paragraph::new(text)
            .block(about_block)
            //.style(Style::new().white().on_black())
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
