use ratatui::prelude::{Buffer, Rect, Stylize};
use ratatui::symbols::border;
use ratatui::widgets::{Block, Widget};
use ratatui::{self, text::Line};

use crate::life;

pub struct LifeWidget<'a> {
    block: Option<Block<'a>>,
    life: &'a life::Life,
}

impl<'a> LifeWidget<'a> {
    pub fn new(life: &'a life::Life) -> Self {
        Self {
            block: None,
            life,
        }}
    }
}

impl Widget for LifeWidget<'_> {
    fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Life ".bold());

        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::THICK);

        let inner = block.inner(area);
        block.render(area, buf);

        let cells = self.life.get_cells();

        for (y, row) in cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                match cell {
                    life::LifeCell::Alive => {
                        //buf[(x as u16, y as u16)].set_symbol("▒");
                        buf[(x as u16 + inner.x, y as u16 + inner.y)].set_symbol("▓");
                    }
                    life::LifeCell::Dead => (),
                }
            }
        }
    }
}
