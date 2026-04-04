use crate::{
    AppCommand, AppEvent, AppEventType,
    life::Life,
    windows::{LifeWindow, TooSmallWindow, WindowDrawResult},
};
use crossterm::event::Event;
use ratatui::layout::Size;

/// Root Child Window Types
enum RootChildWindow {
    Life(LifeWindow),
    TooSmall(TooSmallWindow),
}

impl RootChildWindow {
    /// Initialize root child windows.
    fn init(&mut self) {
        match self {
            RootChildWindow::Life(win) => win.init(),
            RootChildWindow::TooSmall(_) => (),
        }
    }

    /// Draw root child windows.
    fn draw(&mut self, frame: &mut ratatui::Frame, life: &mut Life) -> Option<WindowDrawResult> {
        match self {
            RootChildWindow::Life(win) => win.draw(frame, life),
            RootChildWindow::TooSmall(win) => win.draw(frame),
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
            _ => None,
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

    /// True if we're too small.
    too_small: bool,
}

impl RootWindow {
    /// Create a new Root window.
    pub fn new() -> Self {
        Self {
            life: Life::new(),
            child_window: None,
            too_small: false,
        }
    }

    /// Choose the proper root window
    fn set_root_window(&mut self) {
        self.child_window = if self.too_small {
            Some(RootChildWindow::TooSmall(TooSmallWindow::new()))
        } else {
            Some(RootChildWindow::Life(LifeWindow::new()))
        };
    }

    /// Set too small flag and proper window.
    ///
    /// Returns true if it just changed.
    fn set_too_small(&mut self, width: u16, height: u16) -> bool {
        let too_small = width < 40 || height < 20;

        if too_small != self.too_small {
            self.too_small = too_small;
            self.set_root_window();
            true
        } else {
            false
        }
    }

    /// Initialize root window.
    pub fn init(&mut self, size: Size) {
        self.set_too_small(size.width, size.height);
        self.set_root_window(); // Unconditionally do this

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
        if let AppEventType::Event(Event::Resize(width, height)) = app_event.event_type
            && self.set_too_small(width, height)
            && self.too_small
        {
            return Some(AppCommand::TimerStop);
        }

        if let Some(win) = self.child_window.as_mut() {
            win.handle_app_event(app_event, &mut self.life)
        } else {
            None
        }
    }
}
