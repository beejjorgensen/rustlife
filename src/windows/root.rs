use crate::{
    AppCommand, AppEvent,
    life::Life,
    windows::{LifeWindow, WindowDrawResult},
};
use ratatui::layout::Size;

/// Root Child Window Types
enum RootChildWindow {
    Life(LifeWindow),
}

impl RootChildWindow {
    /// Initialize root child windows.
    fn init(&mut self) {
        match self {
            RootChildWindow::Life(win) => win.init(),
        }
    }

    /// Draw root child windows.
    fn draw(&mut self, frame: &mut ratatui::Frame, life: &mut Life) -> Option<WindowDrawResult> {
        match self {
            RootChildWindow::Life(win) => win.draw(frame, life),
        }
    }

    /// Handle app events on root child windows.
    fn handle_app_event(
        &mut self,
        app_event: &mut AppEvent,
        life: &mut Life,
    ) -> Option<AppCommand> {
        match self {
            RootChildWindow::Life(win) => win.handle_app_event(app_event, life),
        }
    }
}

/// Root Window.
///
/// This is a dummy window that we layer the real root window(s) on top of. This allows us to swap
/// root windows out easily and to use the same event handling everywhere.
pub struct RootWindow {
    /// The Life grid data structure.
    life: Life,

    /// Root window trampoline
    child_window: Option<RootChildWindow>,
}

impl RootWindow {
    /// Create a new Root window.
    pub fn new() -> Self {
        Self {
            life: Life::new(),
            child_window: Some(RootChildWindow::Life(LifeWindow::new())),
        }
    }

    /// Initialize root window.
    pub fn init(&mut self, size: Size) {
        self.life
            .init(size.width as usize - 2, size.height as usize - 2);

        self.life.randomize();

        if let Some(win) = self.child_window.as_mut() {
            win.init();
        }
    }

    /// Draw the Root Window.
    pub fn draw(&mut self, frame: &mut ratatui::Frame) -> Option<WindowDrawResult> {
        if let Some(win) = self.child_window.as_mut() {
            win.draw(frame, &mut self.life)
        } else {
            None
        }
    }

    /// Handle app events for the Root Window.
    pub fn handle_app_event(&mut self, app_event: &mut AppEvent) -> Option<AppCommand> {
        if let Some(win) = self.child_window.as_mut() {
            win.handle_app_event(app_event, &mut self.life)
        } else {
            None
        }
    }
}
