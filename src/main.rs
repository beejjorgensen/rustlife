use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{self, DefaultTerminal};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

struct App {
}

impl App {
    fn new() -> Self {
        Self {}
    }

    fn start(&self) -> Result<()> {
        let terminal = ratatui::init();
        let result = self.run(terminal);
        ratatui::restore();
        result
    }

    fn run(&self, mut terminal: DefaultTerminal) -> Result<()> {
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
        let x = frame.area().width / 2;
        let y = frame.area().height / 2;
        frame.set_cursor_position((x, y));
    }

    fn handle_key_event(&self, key_event: &KeyEvent) -> bool {
        if key_event.code == KeyCode::Char('q') || key_event.code == KeyCode::Esc {
            false
        } else {
            true
        }
    }
}

fn main() -> Result<()> {
    let app = App::new();
    app.start()
}
