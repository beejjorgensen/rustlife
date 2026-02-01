use crossterm::event::KeyEvent;

pub trait Window {
    fn draw(&mut self, frame: &mut ratatui::Frame);
    fn handle_key_event(&mut self, key_event: &KeyEvent) -> Option<WindowResult>;
}

#[derive(PartialEq)]
pub enum WindowResult {
    Quit,
}
