use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::prelude::Rect;
use ratatui::{self, DefaultTerminal};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

mod life;
mod lifewidget;
mod util;

use life::*;
use lifewidget::*;

struct App {
    life_widget_state: LifeWidgetState,
    life_widget_rect: Rect,
}

impl App {
    fn new() -> Self {
        Self {
            life_widget_state: LifeWidgetState {
                cursor_x: 0,
                cursor_y: 0,
                life: Life::new(),
            },
            life_widget_rect: Rect::default(),
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

        const BORDER_SUB: usize = 2; // amount to subtract for life widget border

        self.life_widget_state.life.init(
            self.life_widget_rect.width as usize - BORDER_SUB,
            self.life_widget_rect.height as usize - BORDER_SUB,
        ); // -2 for the border

        self.life_widget_state.life.randomize();

        self.life_widget_state.cursor_x = self.life_widget_rect.width / 2 + self.life_widget_rect.x;
        self.life_widget_state.cursor_y =
            self.life_widget_rect.height / 2 + self.life_widget_rect.y;

        Ok(())
    }

    fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.init(&terminal)?;

        loop {
            terminal.draw(|frame| self.render(frame))?;

            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    if !self.handle_key_event(&key_event) {
                        break;
                    }
                }
                _ => (),
            }
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut ratatui::Frame) {
        let life_widget = LifeWidget {};

        //frame.render_stateful_widget(life_widget, frame.area(), &mut self.life_widget_state);
        frame.render_stateful_widget(
            life_widget,
            self.life_widget_rect,
            &mut self.life_widget_state,
        );

        frame.set_cursor_position((
            self.life_widget_state.cursor_x,
            self.life_widget_state.cursor_y,
        ));
    }

    fn handle_key_event(&mut self, key_event: &KeyEvent) -> bool {
        match key_event.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                return false;
            }

            KeyCode::Up | KeyCode::Char('k') => {
                self.life_widget_state.cursor_y -= 1;
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.life_widget_state.cursor_y += 1;
            }
            KeyCode::Left | KeyCode::Char('h') => {
                self.life_widget_state.cursor_x -= 1;
            }
            KeyCode::Right | KeyCode::Char('l') => {
                self.life_widget_state.cursor_x += 1;
            }
            KeyCode::Char('y') => {
                self.life_widget_state.cursor_x -= 1;
                self.life_widget_state.cursor_y -= 1;
            }
            KeyCode::Char('u') => {
                self.life_widget_state.cursor_x += 1;
                self.life_widget_state.cursor_y -= 1;
            }
            KeyCode::Char('b') => {
                self.life_widget_state.cursor_x -= 1;
                self.life_widget_state.cursor_y += 1;
            }
            KeyCode::Char('n') => {
                self.life_widget_state.cursor_x += 1;
                self.life_widget_state.cursor_y += 1;
            }

            KeyCode::Char('s') => {
                self.life_widget_state.life.step();
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
