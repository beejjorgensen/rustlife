use crate::{
    AppCommand, AppCommands, AppEvent, AppEventType,
    life::Life,
    util,
    widgets::LifeWidget,
    windows::{Window, WindowDrawResult},
};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{layout::Size, prelude::Stylize, symbols::border, text::Line, widgets::Block};
use std::time::Duration;

/// Window to show the Life grid.
pub struct LifeWindow {
    /// The Life grid data structure.
    life: Life,

    /// Cursor X position.
    cursor_x: u16,

    /// Cursor Y position.
    cursor_y: u16,

    /// True if the life simuation is in continuous-run mode.
    pub running: bool,

    /// Tracker for prefix count on some commands
    count: u32,
}

impl LifeWindow {
    /// Create a new LifeWindow.
    pub fn new() -> LifeWindow {
        LifeWindow {
            life: Life::new(),
            cursor_x: 0,
            cursor_y: 0,
            running: false,
            count: 0,
        }
    }

    /// Handle Life window key events.
    fn handle_key_event(&mut self, key_event: &KeyEvent) -> AppCommands {
        let mut app_commands = AppCommands::none();
        let old_running = self.running;

        match key_event.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                app_commands.push(AppCommand::Quit);
            }

            KeyCode::Up | KeyCode::Char('k') => {
                self.cursor_y -= 1;
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.cursor_y += 1;
            }
            KeyCode::Left | KeyCode::Char('h') => {
                if self.count > 0 {
                    self.running = false;
                    self.life.horizontal_line(
                        self.cursor_x as usize - 1,
                        self.cursor_y as usize - 1,
                        self.count,
                    );
                } else {
                    self.cursor_x -= 1;
                }
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
                self.running = false;
                self.life.step();
            }

            KeyCode::Char(' ') | KeyCode::Char('t') => {
                self.running = false;
                self.life
                    .toggle(self.cursor_x as usize - 1, self.cursor_y as usize - 1);
            }

            KeyCode::Char('c') => {
                self.running = false;
                self.life.clear();
            }

            KeyCode::Char('R') => {
                self.life.randomize();
            }

            KeyCode::Char('r') => {
                self.running = !self.running;
            }

            KeyCode::Char('?') => {
                app_commands.push(AppCommand::HelpPopup);
            }

            _ => (),
        }

        match key_event.code {
            KeyCode::Char(c) if c.is_ascii_digit() => {
                let value = c.to_digit(10).unwrap();
                self.count = self.count * 10 + value;
            }

            _ => {
                self.count = 0;
            }
        }

        let running_changed = old_running != self.running;

        if running_changed {
            if self.running {
                app_commands.push(AppCommand::TimerStart(Duration::from_millis(20)));
            } else {
                app_commands.push(AppCommand::TimerStop);
            }
        }

        app_commands
    }
}

impl Window for LifeWindow {
    /// Initialize the LifeWindow.
    fn init(&mut self, terminal_size: &Size) {
        self.life.init(
            terminal_size.width as usize - 2,
            terminal_size.height as usize - 2,
        );

        self.life.randomize();

        self.cursor_x = terminal_size.width / 2;
        self.cursor_y = terminal_size.height / 2;
    }

    /// Draw the LifeWindow
    fn draw(&mut self, frame: &mut ratatui::Frame) -> Option<WindowDrawResult> {
        let block = Block::bordered()
            .title(Line::from(" Life ".bold()).centered())
            .title_bottom(Line::from(" q→Quit | ?→Help ").centered())
            .border_set(border::THICK);

        let life_widget = LifeWidget::new(&self.life).block(block);

        let inner = life_widget.inner(frame.area());

        frame.render_widget(life_widget, frame.area());

        (self.cursor_x, self.cursor_y) = util::clamp_to_rect(self.cursor_x, self.cursor_y, inner);

        Some(WindowDrawResult::cursor_position(
            self.cursor_x,
            self.cursor_y,
        ))
    }

    /// Handle application events.
    fn handle_app_event(&mut self, app_event: &mut AppEvent) -> AppCommands {
        let mut app_commands = AppCommands::none();

        match &app_event.event_type {
            AppEventType::Event(e) => match e {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    app_commands.append(&mut self.handle_key_event(key_event));
                }

                Event::Resize(width, height) => {
                    self.life.resize(*width as usize - 1, *height as usize - 2);
                }

                _ => (),
            },

            AppEventType::Tick => {
                self.life.step();
            }
        }

        app_commands
    }
}
