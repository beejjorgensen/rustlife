/// This is passed back from the `draw()` method to let the main app do things with the Terminal
/// instance.
pub struct WindowDrawResult {
    pub cursor_x: u16,
    pub cursor_y: u16,
    pub cursor_visible: bool,
}

impl Default for WindowDrawResult {
    /// Make a new default WindowDrawResult.
    // TODO make AppCommand?
    fn default() -> Self {
        Self {
            cursor_x: 0,
            cursor_y: 0,
            cursor_visible: false,
        }
    }
}

impl WindowDrawResult {
    /// Make a new WindowDrawResult with the cursor positioned.
    pub fn cursor_position(cursor_x: u16, cursor_y: u16) -> Self {
        Self {
            cursor_x,
            cursor_y,
            cursor_visible: true,
        }
    }

    /// Make a new WindowDrawResult with the cursor hidden.
    pub fn cursor_hide() -> Self {
        Self {
            cursor_x: 0,
            cursor_y: 0,
            cursor_visible: false,
        }
    }
}
