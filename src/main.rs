use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::prelude::Stylize;
use ratatui::symbols::border;
use ratatui::widgets::Block;
use ratatui::{self, DefaultTerminal, text::Line};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

struct App {
    cursor_x: u16,
    cursor_y: u16,
}

impl App {
    fn new() -> Self {
        Self {
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

    fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let term_size = terminal.size()?;

        self.cursor_x = term_size.width / 2;
        self.cursor_y = term_size.height / 2;

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
        if self.cursor_y < 1 {
            self.cursor_y = 1;
        }

        if self.cursor_y > frame.area().height - 2 {
            self.cursor_y = frame.area().height - 2;
        }

        if self.cursor_x < 1 {
            self.cursor_x = 1;
        }

        if self.cursor_x > frame.area().width - 2 {
            self.cursor_x = frame.area().width - 2;
        }

        let title = Line::from(" Life ".bold());

        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::THICK);

        frame.render_widget(block, frame.area());

        let fb = frame.buffer_mut();
        fb[(self.cursor_x, self.cursor_y)].set_symbol("#");

        frame.set_cursor_position((self.cursor_x, self.cursor_y));
    }

    fn handle_key_event(&mut self, key_event: &KeyEvent) -> bool {
        match key_event.code {
            KeyCode::Char('Q') | KeyCode::Esc => {
                return false;
            }

            KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('w') => {
                self.cursor_y -= 1;
            }
            KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('s') | KeyCode::Char('x') => {
                self.cursor_y += 1;
            }
            KeyCode::Left | KeyCode::Char('h') | KeyCode::Char('a') => {
                self.cursor_x -= 1;
            }
            KeyCode::Right | KeyCode::Char('l') | KeyCode::Char('d') => {
                self.cursor_x += 1;
            }
            KeyCode::Char('u') | KeyCode::Char('q') => {
                self.cursor_x -= 1;
                self.cursor_y -= 1;
            }
            KeyCode::Char('o') | KeyCode::Char('e') => {
                self.cursor_x += 1;
                self.cursor_y -= 1;
            }
            KeyCode::Char('n') | KeyCode::Char('z') => {
                self.cursor_x -= 1;
                self.cursor_y += 1;
            }
            KeyCode::Char(',') | KeyCode::Char('c') => {
                self.cursor_x += 1;
                self.cursor_y += 1;
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
