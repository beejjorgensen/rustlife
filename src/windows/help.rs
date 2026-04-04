use crate::{
    AppCommand, AppEvent, AppEventType, util, widgets::HelpWidget, windows::WindowDrawResult,
};
use crossterm::event::{Event, KeyEventKind};

/// Window to show the HelpWidget.
pub struct HelpWindow;

impl HelpWindow {
    /// Make a new HelpWindow.
    pub fn new() -> Self {
        Self {}
    }

    /// Draw the Help Window.
    pub fn draw(&mut self, frame: &mut ratatui::Frame) -> Option<WindowDrawResult> {
        let area = util::centered_area(43, 17, frame);
        let help = HelpWidget::new();
        frame.render_widget(help, area);

        Some(WindowDrawResult::cursor_hide())
    }

    /// Handle app events for the Help Window.
    pub fn handle_app_event(&mut self, app_event: &mut AppEvent) -> Option<AppCommand> {
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
