use crate::{
    AppCommand, AppEvent, AppEventType,
    windows::{Window, WindowDrawResult},
};
use crossterm::event::{Event, KeyEventKind};
use ratatui::{
    text::{Line, Span},
    style::Style,
    widgets::{Paragraph, Block, Wrap},
    layout::{Flex, Alignment},
    prelude::{Constraint, Direction, Layout, Stylize},
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
        let outer = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(16)])
            .flex(Flex::Center)
            .split(frame.area());

        let inner = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Length(40)])
            .flex(Flex::Center)
            .split(outer[0]);

        let text = vec![
            Line::from(vec![
                Span::raw("First"),
                Span::styled("line", Style::new().green().italic()),
                ".".into(),
            ]),
            Line::from("Second line".red()),
            "Third line".into(),
        ];

        let paragraph = Paragraph::new(text)
            .block(Block::bordered().title("Paragraph"))
            .style(Style::new().white().on_black())
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        frame.render_widget(paragraph, inner[0]);

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
