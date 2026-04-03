//! Various utility functions
use ratatui::{
    layout::{Constraint, Direction, Flex, Layout},
    prelude::{Frame, Rect},
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
