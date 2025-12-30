use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{self, DefaultTerminal};

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
                _ => ()
            }
        }
        Ok(())
    }

    fn render(&self, frame: &mut ratatui::Frame) {
        frame.set_cursor_position((self.cursor_x, self.cursor_y));
    }

    fn handle_key_event(&mut self, key_event: &KeyEvent) -> bool {
        match key_event.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                return false;
            }
            KeyCode::Up => {
                self.cursor_y -= 1;
            }
            KeyCode::Down => {
                self.cursor_y += 1;
            }
            KeyCode::Left => {
                self.cursor_x -= 1;
            }
            KeyCode::Right => {
                self.cursor_x += 1;
            }
            _ => ()
        }

        true
    }
}

fn main() -> Result<()> {
    let mut app = App::new();
    app.start()
}
