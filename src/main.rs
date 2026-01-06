use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::prelude::{Constraint, Direction, Layout, Rect, Stylize};
use ratatui::symbols::border;
use ratatui::widgets::{Block, Clear, Paragraph};
use ratatui::{self, DefaultTerminal, text::Line};
use std::time::{Duration, Instant};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

mod life;
mod lifewidget;
mod util;

use life::*;
use lifewidget::*;

enum AppEvent {
    Event(crossterm::event::Event),
    Tick,
}

struct App {
    life_widget_rect: Rect,
    life: Life,
    cursor_x: u16,
    cursor_y: u16,
    running: bool,
    tick_rate: Duration,
    next_tick: Instant,
    help_popup: bool,
}

impl App {
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

    fn start(&mut self) -> Result<()> {
        let terminal = ratatui::init();
        let result = self.run(terminal);
        ratatui::restore();
        result
    }

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

    fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.init(&terminal)?;

        loop {
            terminal.draw(|frame| self.draw(frame))?;

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

    fn draw(&mut self, frame: &mut ratatui::Frame) {
        let title = Line::from(" Life ".bold());

        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::THICK);

        let life_widget = LifeWidget::new(&self.life).block(block);

        let inner = life_widget.inner(frame.area());

        frame.render_widget(life_widget, frame.area());

        (self.cursor_x, self.cursor_y) = util::clamp_to_rect(self.cursor_x, self.cursor_y, inner);

        frame.set_cursor_position((self.cursor_x, self.cursor_y));

        if self.help_popup {
            let outer = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Fill(1),
                    Constraint::Length(20),
                    Constraint::Fill(1),
                ])
                .split(frame.area());

            let inner = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Fill(1),
                    Constraint::Length(40),
                    Constraint::Fill(1),
                ])
                .split(outer[1]);

            let block = Block::bordered()
                .title(Line::from(" Help ".bold()))
                .border_set(border::THICK);

            let paragraph = Paragraph::new("Test line").block(block);

            frame.render_widget(Clear, inner[1]);
            frame.render_widget(paragraph, inner[1]);
        }
    }

    fn handle_key_event(&mut self, key_event: &KeyEvent) -> bool {
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
                self.help_popup = !self.help_popup;
            }

            _ => (),
        }

        true
    }
}

fn main() -> Result<()> {
    let mut app = App::new();
    app.start()
}
