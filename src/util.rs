use ratatui::prelude::Rect;

pub fn clamp_to_rect(x: u16, y: u16, rect: Rect) -> (u16, u16) {
    let max_x = rect.x + rect.width.saturating_sub(1);
    let max_y = rect.y + rect.height.saturating_sub(1);

    (x.clamp(rect.x, max_x), y.clamp(rect.y, max_y))
}

