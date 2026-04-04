//! Various utility functions
use crossterm::terminal;
use ratatui::{
    layout::{Constraint, Direction, Flex, Layout, Size},
    prelude::{Frame, Rect},
    style::Color,
};

/// Clamp x, y coordinates to a [`Rect`].
#[allow(dead_code)]
pub fn clamp_to_rect(x: u16, y: u16, rect: Rect) -> (u16, u16) {
    let max_x = rect.x + rect.width.saturating_sub(1);
    let max_y = rect.y + rect.height.saturating_sub(1);

    (x.clamp(rect.x, max_x), y.clamp(rect.y, max_y))
}

/// Compute a new [`Rect`] from a given one and inset values.
#[allow(dead_code)]
pub fn inset_rect(x_amt: u16, y_amt: u16, rect: Rect) -> Rect {
    Rect {
        x: rect.x + x_amt,
        y: rect.y + y_amt,
        width: rect.width.saturating_sub(x_amt * 2),
        height: rect.height.saturating_sub(y_amt * 2),
    }
}

/// Create a new centered area.
pub fn centered_area(width: u16, height: u16, frame: &Frame) -> Rect {
    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(height)])
        .flex(Flex::Center)
        .split(frame.area());

    let inner = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Length(width)])
        .flex(Flex::Center)
        .split(outer[0]);

    inner[0]
}

/// Get an Indexed color for RGB (0..5).
///
/// 5,5,0 = bright yellow, for instance.
///
/// Use `gray_to_indexed()` for non-black/non-white shades of gray.
///
/// https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit
#[allow(dead_code)]
pub fn rgb6_to_indexed(r: u8, g: u8, b: u8) -> Color {
    Color::Indexed(16 + 36 * r + 6 * g + b)
}

/// Get an Indexed color for gray.
///
/// There are 24 shades of gray, but this function special-cases black and white at either end for
/// a total of 26 grays, 0-25.
///
/// 0 = black, 25 = white, 13 = medium.
#[allow(dead_code)]
pub fn gray_to_indexed(g: u8) -> Color {
    if g == 0 {
        Color::Indexed(0) // Black
    } else if g == 25 {
        Color::Indexed(15) // White
    } else {
        Color::Indexed(232 + (g - 1))
    }
}

/// Get the terminal size as a Rect.
#[allow(dead_code)]
pub fn get_terminal_size() -> Size {
    let (cols, rows) = terminal::size().unwrap_or((80, 24));
    Size::new(cols, rows)
}
