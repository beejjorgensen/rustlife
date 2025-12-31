use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::prelude::Rect;
use ratatui::{self, DefaultTerminal};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

mod life;
mod lifewidget;
mod util;

use lifewidget::*;

struct App {
    life_widget_state: LifeWidgetState,
}

impl App {
    fn new() -> Self {
        Self {
            life_widget_state: LifeWidgetState {
                cursor_x: 0,
                cursor_y: 0,
            },
        }
    }

    fn start(&mut self) -> Result<()> {
        let terminal = ratatui::init();
        let result = self.run(terminal);
        ratatui::restore();
        result
    }

    fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let term_size = terminal.size()?;

        self.life_widget_state.cursor_x = term_size.width / 2;
        self.life_widget_state.cursor_y = term_size.height / 2;

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
            Rect {
                x: 10,
                y: 5,
                width: 30,
                height: 20,
            },
            &mut self.life_widget_state,
        );

        frame.set_cursor_position((
            self.life_widget_state.cursor_x,
            self.life_widget_state.cursor_y,
        ));
    }

    fn handle_key_event(&mut self, key_event: &KeyEvent) -> bool {
        match key_event.code {
            KeyCode::Char('Q') | KeyCode::Esc => {
                return false;
            }

            KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('w') => {
                self.life_widget_state.cursor_y -= 1;
            }
            KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('s') | KeyCode::Char('x') => {
                self.life_widget_state.cursor_y += 1;
            }
            KeyCode::Left | KeyCode::Char('h') | KeyCode::Char('a') => {
                self.life_widget_state.cursor_x -= 1;
            }
            KeyCode::Right | KeyCode::Char('l') | KeyCode::Char('d') => {
                self.life_widget_state.cursor_x += 1;
            }
            KeyCode::Char('u') | KeyCode::Char('q') => {
                self.life_widget_state.cursor_x -= 1;
                self.life_widget_state.cursor_y -= 1;
            }
            KeyCode::Char('o') | KeyCode::Char('e') => {
                self.life_widget_state.cursor_x += 1;
                self.life_widget_state.cursor_y -= 1;
            }
            KeyCode::Char('n') | KeyCode::Char('z') => {
                self.life_widget_state.cursor_x -= 1;
                self.life_widget_state.cursor_y += 1;
            }
            KeyCode::Char(',') | KeyCode::Char('c') => {
                self.life_widget_state.cursor_x += 1;
                self.life_widget_state.cursor_y += 1;
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
