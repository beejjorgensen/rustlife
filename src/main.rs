use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{self, DefaultTerminal};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(render)?;
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                if !handle_key_event(&key_event) {
                    break;
                }
            }

            e => {
                println!("{:#?}", e);
            }
        }
    }
    Ok(())
}

fn render(frame: &mut ratatui::Frame) {
    let x = frame.area().width / 2;
    let y = frame.area().height / 2;
    frame.set_cursor_position((x, y));
}

fn handle_key_event(key_event: &KeyEvent) -> bool {
    if key_event.code == KeyCode::Char('q') || key_event.code == KeyCode::Esc {
        false
    } else {
        true
    }
}
