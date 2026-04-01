use crate::{AppEvent, AppCommand};
use ratatui::layout::Size;

/// Interface for a Window. The Window draws its own widgets and handles events.
pub trait Window {
    /// Initialize given the terminal.
    fn init(&mut self, _terminal_size: &Size) {}

    /// Ratatui main drawing function.
    fn draw(&mut self, frame: &mut ratatui::Frame) -> Option<WindowDrawResult>;

    /// Handle application events.
    fn handle_app_event(&mut self, _app_event: &mut AppEvent) -> Option<AppCommand> {
        None
    }

    /// Get the child window
    fn get_child(&self) -> Option<&dyn Window> { None }

    /// Get the child window mutably
    fn get_child_mut(&self) -> Option<&mut dyn Window> { None }
}

/// This is passed back from the `draw()` method to let the main app do things with the Terminal
/// instance.
pub struct WindowDrawResult {
    pub cursor_x: u16,
    pub cursor_y: u16,
    pub cursor_visible: bool,
}

impl Default for WindowDrawResult {
    /// Make a new default WindowDrawResult.
    // TODO make AppCommand?
    fn default() -> Self {
        Self {
            cursor_x: 0,
            cursor_y: 0,
            cursor_visible: false,
        }
    }
}

impl WindowDrawResult {
    /// Make a new WindowDrawResult with the cursor positioned.
    pub fn cursor_position(cursor_x: u16, cursor_y: u16) -> Self {
        Self {
            cursor_x,
            cursor_y,
            cursor_visible: true,
        }
    }

    /// Make a new WindowDrawResult with the cursor hidden.
    pub fn cursor_hide() -> Self {
        Self {
            cursor_x: 0,
            cursor_y: 0,
            cursor_visible: false,
        }
    }
}
