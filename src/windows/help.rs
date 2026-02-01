use crate::widgets::HelpWidget;
use crate::windows::{Window, WindowResult};
use crossterm::event::KeyEvent;
use ratatui::{
    layout::Flex,
    prelude::{Constraint, Direction, Layout},
};

pub struct HelpWindow;

impl Window for HelpWindow {
    fn draw(&mut self, frame: &mut ratatui::Frame) {
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
    }

    fn handle_key_event(&mut self, _key_event: &KeyEvent) -> Option<WindowResult> {
        Some(WindowResult::Quit)
    }
}
