use ratatui::prelude::{BlockExt, Buffer, Rect};
use ratatui::widgets::{Block, Widget};

use crate::life;

pub struct LifeWidget<'a> {
    block: Option<Block<'a>>,
    life: &'a life::Life,
}

impl<'a> LifeWidget<'a> {
    pub fn new(life: &'a life::Life) -> Self {
        Self { block: None, life }
    }

    #[allow(dead_code)]
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    #[allow(dead_code)]
    pub fn inner(&self, area: Rect) -> Rect {
        if let Some(block) = &self.block {
            block.inner(area)
        } else {
            area
        }
    }
}

impl Widget for LifeWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Widget::render(&self, area, buf);
    }
}

impl Widget for &LifeWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = area.intersection(buf.area);
        if let Some(block) = self.block.as_ref() {
            block.render(area, buf);
        }

        let inner = self.block.inner_if_some(area);

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
