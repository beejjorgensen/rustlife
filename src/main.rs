//! Implementation of [Conway's Game of
//! Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) using [Ratatui](ratatui.rs).
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    self, DefaultTerminal,
    layout::Flex,
    prelude::{Constraint, Direction, Layout, Rect, Stylize},
    symbols::border,
    text::Line,
    widgets::Block,
};
use std::time::{Duration, Instant};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

mod helpwidget;
mod life;
mod lifewidget;
mod util;

use helpwidget::*;
use life::*;
use lifewidget::*;

/// Application-level events.
enum AppEvent {
    /// Normal crossterm event.
    Event(crossterm::event::Event),
    /// Tick event.
    Tick,
}

/// Main application structure.
struct App {
    /// The dimensions of the main life widget.
    life_widget_rect: Rect,

    /// The Life grid data structure.
    life: Life,

    /// Cursor X position.
    cursor_x: u16,

    /// Cursor Y position.
    cursor_y: u16,

    /// True if the life simuation is in continuous-run mode.
    running: bool,

    /// Delay between animation frames.
    tick_rate: Duration,

    /// Time when the next tick should trigger.
    next_tick: Instant,

    /// True if the help popup is active.
    help_popup: bool,
}

impl App {
    /// Create a new App object.
    fn new() -> Self {
        Self {
            life_widget_rect: Rect::default(),
            life: Life::new(),
            cursor_x: 0,
            cursor_y: 0,
            running: false,
            tick_rate: Duration::from_millis(20),
            next_tick: Instant::now(),
            help_popup: false,
        }
    }

    /// Initialize various data structures.
    fn init(&mut self, terminal: &DefaultTerminal) -> Result<()> {
        let term_size = terminal.size()?;

        self.life_widget_rect = Rect {
            x: 0,
            y: 0,
            width: term_size.width,
            height: term_size.height,
        };

        self.life.init(
            self.life_widget_rect.width as usize - 2,
            self.life_widget_rect.height as usize - 2,
        );

        self.life.randomize();

        self.cursor_x = self.life_widget_rect.width / 2 + self.life_widget_rect.x;
        self.cursor_y = self.life_widget_rect.height / 2 + self.life_widget_rect.y;

        Ok(())
    }

    /// Run loop.
    fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        self.init(terminal)?;

        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if self.help_popup {
                terminal.hide_cursor()?;
            } else {
                terminal.show_cursor()?;
            }

            terminal.set_cursor_position((self.cursor_x, self.cursor_y))?;

            let now = Instant::now();
            let timeout = self
                .next_tick
                .checked_duration_since(now)
                .unwrap_or(Duration::ZERO);

            let app_event = if self.running {
                if event::poll(timeout)? {
                    AppEvent::Event(event::read()?)
                } else {
                    AppEvent::Tick
                }
            } else {
                AppEvent::Event(event::read()?)
            };

            match app_event {
                AppEvent::Event(e) => match e {
                    Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                        if !self.handle_key_event(&key_event) {
                            break;
                        }
                    }

                    Event::Resize(width, height) => {
                        self.life_widget_rect.width = width;
                        self.life_widget_rect.height = height;
                        self.life.resize(width as usize - 2, height as usize - 2);
                    }

                    _ => (),
                },

                AppEvent::Tick => {
                    self.life.step();
                    self.next_tick += self.tick_rate;
                }
            }
        }
        Ok(())
    }

    /// Render the help popup
    fn show_help_popup(&self, frame: &mut ratatui::Frame) {
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

    /// Main drawing method.
    fn draw(&mut self, frame: &mut ratatui::Frame) {
        let block = Block::bordered()
            .title(Line::from(" Life ".bold()).centered())
            .title_bottom(Line::from(" q→Quit | ?→Help ").centered())
            .border_set(border::THICK);

        let life_widget = LifeWidget::new(&self.life).block(block);

        let inner = life_widget.inner(frame.area());

        frame.render_widget(life_widget, frame.area());

        (self.cursor_x, self.cursor_y) = util::clamp_to_rect(self.cursor_x, self.cursor_y, inner);

        if self.help_popup {
            self.show_help_popup(frame);
        }
    }

    /// Handle various key events.
    fn handle_key_event(&mut self, key_event: &KeyEvent) -> bool {
        if self.help_popup {
            self.help_popup = false;
            return true;
        }

        match key_event.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                return false;
            }

            KeyCode::Up | KeyCode::Char('k') => {
                self.cursor_y -= 1;
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.cursor_y += 1;
            }
            KeyCode::Left | KeyCode::Char('h') => {
                self.cursor_x -= 1;
            }
            KeyCode::Right | KeyCode::Char('l') => {
                self.cursor_x += 1;
            }
            KeyCode::Char('y') => {
                self.cursor_x -= 1;
                self.cursor_y -= 1;
            }
            KeyCode::Char('u') => {
                self.cursor_x += 1;
                self.cursor_y -= 1;
            }
            KeyCode::Char('b') => {
                self.cursor_x -= 1;
                self.cursor_y += 1;
            }
            KeyCode::Char('n') => {
                self.cursor_x += 1;
                self.cursor_y += 1;
            }

            KeyCode::Char('s') => {
                self.life.step();
            }

            KeyCode::Char(' ') | KeyCode::Char('t') => {
                self.life
                    .toggle(self.cursor_x as usize - 1, self.cursor_y as usize - 1);
            }

            KeyCode::Char('c') => {
                self.life.clear();
            }

            KeyCode::Char('R') => {
                self.life.randomize();
            }

            KeyCode::Char('r') => {
                self.running = !self.running;
                self.next_tick = Instant::now();
            }

            KeyCode::Char('?') => {
                self.help_popup = true;
            }

            _ => (),
        }

        true
    }
}

/// Main.
fn main() -> Result<()> {
    let mut app = App::new();
    ratatui::run(|terminal| app.run(terminal))?;
    Ok(())
}
