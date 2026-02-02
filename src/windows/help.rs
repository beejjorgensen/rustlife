use crate::{
    AppCommand, AppCommands, AppEvent, AppEventType,
    widgets::HelpWidget,
    windows::{Window, WindowDrawResult},
};
use crossterm::event::{Event, KeyEventKind};
use ratatui::{
    layout::Flex,
    prelude::{Constraint, Direction, Layout},
};

pub struct HelpWindow;

impl Window for HelpWindow {
    /// Draw the Help Window.
    fn draw(&mut self, frame: &mut ratatui::Frame) -> Option<WindowDrawResult> {
        let outer = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(16)])
            .flex(Flex::Center)
            .split(frame.area());

        let inner = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Length(26)])
            .flex(Flex::Center)
            .split(outer[0]);

        let help = HelpWidget::new();
        frame.render_widget(help, inner[0]);

        Some(WindowDrawResult::cursor_hide())
    }

    /// Handle app events for the Help Window.
    fn handle_app_event(&mut self, app_event: &mut AppEvent) -> AppCommands {
        match &app_event.event_type {
            AppEventType::Event(e) => match e {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    app_event.propagate = false;
                    AppCommands::one(AppCommand::Quit)
                }

                _ => AppCommands::none(),
            },

            _ => AppCommands::none(),
        }
    }
}
