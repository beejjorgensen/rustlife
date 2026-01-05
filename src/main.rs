use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::prelude::{Rect, Stylize};
use ratatui::symbols::border;
use ratatui::widgets::Block;
use ratatui::{self, DefaultTerminal, text::Line};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

mod life;
mod lifewidget;
mod util;

use life::*;
use lifewidget::*;

struct App {
    life_widget_rect: Rect,
    life: Life,
    cursor_x: u16,
    cursor_y: u16,
}

impl App {
    fn new() -> Self {
        Self {
            life_widget_rect: Rect::default(),
            life: Life::new(),
            cursor_x: 0,
            cursor_y: 0,
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

            match event::read()? {
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

        self.life_widget_rect = frame.area();
        let inner = util::inset_rect(1, 1, self.life_widget_rect);

        frame.render_widget(life_widget, self.life_widget_rect);

        (self.cursor_x, self.cursor_y) = util::clamp_to_rect(self.cursor_x, self.cursor_y, inner);

        frame.set_cursor_position((self.cursor_x, self.cursor_y));
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

            KeyCode::Char('r') => {
                self.life.randomize();
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
