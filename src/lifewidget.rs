use ratatui::prelude::{Buffer, Rect, Stylize};
use ratatui::symbols::border;
use ratatui::widgets::{Block, StatefulWidget, Widget};
use ratatui::{self, text::Line};

use crate::util;

pub struct LifeWidgetState {
    pub cursor_x: u16,
    pub cursor_y: u16,
}

pub struct LifeWidget {}

impl StatefulWidget for LifeWidget {
    type State = LifeWidgetState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut LifeWidgetState) {
        let title = Line::from(" Life ".bold());

        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::THICK);

        let inner = block.inner(area);

        block.render(area, buf);

        (state.cursor_x, state.cursor_y) =
            util::clamp_to_rect(state.cursor_x, state.cursor_y, inner);

        buf[(state.cursor_x, state.cursor_y)].set_symbol("#");
    }
}
