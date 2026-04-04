//! Implementation of [Conway's Game of
//! Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) using [Ratatui](ratatui.rs).
use crossterm::event;
use ratatui::{self, DefaultTerminal};
use std::time::{Duration, Instant};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

mod life;
mod util;
mod widgets;
mod windows;

use windows::{RootWindow, WindowDrawResult};

/// Application-level event types.
#[derive(PartialEq)]
enum AppEventType {
    /// Normal crossterm event.
    Event(crossterm::event::Event),
    /// Tick event.
    Tick,
}

/// Application-level events.
struct AppEvent {
    pub event_type: AppEventType,
    pub propagate: bool,
}

impl AppEvent {
    /// Construct a new AppEvent
    fn new(event_type: AppEventType) -> Self {
        AppEvent {
            event_type,
            propagate: true,
        }
    }
}

#[derive(PartialEq)]
/// Returned from handling events to control the main app.
pub enum AppCommand {
    TimerStart(Duration), // Tick immediately
    TimerStop,
    TimerContinue,
    CursorHide,
    CursorShow,
    CursorPosition(u16, u16), // Implies "CursorShow"
    CloseChildWindow,
    Quit,
}

/// Main application structure.
struct App {
    /// Delay between animation frames.
    tick_rate: Duration,

    /// Time when the next tick should trigger.
    next_tick: Option<Instant>,

    /// Reference to the main Life window
    root_window: RootWindow,
}

impl App {
    /// Create a new App object.
    fn new() -> Self {
        Self {
            tick_rate: Duration::from_millis(20),
            next_tick: None,
            root_window: RootWindow::new(),
        }
    }

    fn init(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        let size = terminal.size()?;

        if size.width < 40 || size.height < 20 {
            return Err("terminal too small".into());
        }

        self.root_window.init(size);

        Ok(())
    }

    /// Run loop.
    fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        self.init(terminal)?;

        'outer: loop {
            let mut draw_result = None;

            terminal.draw(|frame| draw_result = self.draw(frame))?;

            if let Some(dr) = draw_result {
                if dr.cursor_visible {
                    terminal.show_cursor()?;
                } else {
                    terminal.hide_cursor()?;
                }

                terminal.set_cursor_position((dr.cursor_x, dr.cursor_y))?;
            }

            let mut app_event = if let Some(next_tick) = self.next_tick {
                let now = Instant::now();
                let timeout = next_tick
                    .checked_duration_since(now)
                    .unwrap_or(Duration::ZERO);

                if event::poll(timeout)? {
                    AppEvent::new(AppEventType::Event(event::read()?))
                } else {
                    self.next_tick = Some(next_tick + self.tick_rate);
                    AppEvent::new(AppEventType::Tick)
                }
            } else {
                AppEvent::new(AppEventType::Event(event::read()?))
            };

            // Route events to windows
            let result = self.root_window.handle_app_event(&mut app_event);

            match result {
                Some(AppCommand::Quit) => break 'outer,

                Some(AppCommand::TimerStart(duration)) => {
                    self.tick_rate = duration;
                    self.next_tick = Some(Instant::now());
                }

                Some(AppCommand::TimerStop) => self.next_tick = None,

                _ => (),
            }
        } // 'outer
        Ok(())
    }

    /// Main drawing method.
    fn draw(&mut self, frame: &mut ratatui::Frame) -> Option<WindowDrawResult> {
        self.root_window.draw(frame)
    }
}

/// Main.
fn main() -> Result<()> {
    let mut app = App::new();
    ratatui::run(|terminal| app.run(terminal))?;
    Ok(())
}
