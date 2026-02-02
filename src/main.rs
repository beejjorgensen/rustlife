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

use windows::{HelpWindow, LifeWindow, Window, WindowDrawResult};

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
    HelpPopup, // TODO generalize to arbitrary popups
    Close,
    Quit,
}

/// Commands back to the app from event handlers
/// TODO break this into other source?
struct AppCommands(Vec<AppCommand>);

impl AppCommands {
    /// Create a no-op AppCommands.
    pub fn none() -> Self {
        Self(Vec::new())
    }

    /// Create an AppCommands with a single AppCommand.
    pub fn one(command: AppCommand) -> Self {
        Self(vec![command])
    }

    /// Append another AppCommand.
    pub fn push(&mut self, command: AppCommand) {
        self.0.push(command);
    }

    /// Append another set of AppCommands.
    pub fn append(&mut self, app_commands: &mut AppCommands) {
        self.0.append(&mut app_commands.0);
    }
}

impl IntoIterator for AppCommands {
    type Item = AppCommand;
    type IntoIter = std::vec::IntoIter<AppCommand>;

    /// Return a moving iterator.
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a AppCommands {
    type Item = &'a AppCommand;
    type IntoIter = std::slice::Iter<'a, AppCommand>;

    /// Return a borrowing iterator.
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

/// Main application structure.
struct App {
    /// Delay between animation frames.
    tick_rate: Duration,

    /// Time when the next tick should trigger.
    next_tick: Option<Instant>,

    /// Reference to the main Life window
    life_window: LifeWindow,

    /// Reference to the Help window
    help_window: HelpWindow,

    /// True if the help popup is active.
    help_popup: bool,
}

impl App {
    /// Create a new App object.
    fn new() -> Self {
        Self {
            tick_rate: Duration::from_millis(20),
            next_tick: None,
            life_window: LifeWindow::new(),
            help_window: HelpWindow {},
            help_popup: false,
        }
    }

    /// Run loop.
    fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        let terminal_size = terminal.size()?;

        self.life_window.init(&terminal_size);

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
            // TODO generalize away from help_popup, make a window stack
            if self.help_popup {
                let app_commands = self.help_window.handle_app_event(&mut app_event);
                for command in &app_commands {
                    if command == &AppCommand::Close {
                        self.help_popup = false;

                        // If I don't continue here (hackish af), a key event that's supposedly
                        // eaten by the help window still goes through to the life window, below.
                        // We need like a "StopPropagation" command or something.
                        continue 'outer;
                    }
                }
            }

            if app_event.propagate {
                // Main app
                let app_commands = self.life_window.handle_app_event(&mut app_event);

                for command in &app_commands {
                    match command {
                        AppCommand::Quit => break 'outer,

                        AppCommand::TimerStart(duration) => {
                            self.tick_rate = *duration;
                            self.next_tick = Some(Instant::now());
                        }

                        AppCommand::TimerStop => self.next_tick = None,

                        AppCommand::HelpPopup => self.help_popup = true,

                        _ => (),
                    }
                }
            }
        } // 'outer
        Ok(())
    }

    /// Main drawing method.
    fn draw(&mut self, frame: &mut ratatui::Frame) -> Option<WindowDrawResult> {
        let mut draw_result = self.life_window.draw(frame);

        if self.help_popup {
            draw_result = self.help_window.draw(frame);
        }

        draw_result
    }
}

/// Main.
fn main() -> Result<()> {
    let mut app = App::new();
    ratatui::run(|terminal| app.run(terminal))?;
    Ok(())
}
