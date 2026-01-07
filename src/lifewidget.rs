//!This widget renders the current frame of a [`Life`] object.
//!
//!Example:
//!
//!```
//!let title = Line::from(" Life ".bold());
//!
//!let block = Block::bordered()
//!    .title(title.centered())
//!    .border_set(border::THICK);
//!
//!// `life` is a `Life` object
//!let life_widget = LifeWidget::new(&life).block(block);
//!
//!let inner = life_widget.inner(frame.area());
//!
//!frame.render_widget(life_widget, frame.area());
//!```
//![`Life`]: crate::Life
use ratatui::{
    prelude::{BlockExt, Buffer, Rect},
    widgets::{Block, Widget},
};

use crate::life;

/// A LifeWidget structure.
pub struct LifeWidget<'a> {
    /// An optional surrounding Block widget.
    block: Option<Block<'a>>,

    /// Reference to the life data structure.
    life: &'a life::Life,
}

impl<'a> LifeWidget<'a> {
    /// Create a new LifeWidget.
    pub fn new(life: &'a life::Life) -> Self {
        Self { block: None, life }
    }

    /// Add a Block widget to this LifeWidget.
    #[allow(dead_code)]
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    /// Get the inner dimensions of the LifeWidget.
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
    /// Render this LifeWidget.
    fn render(self, area: Rect, buf: &mut Buffer) {
        Widget::render(&self, area, buf);
    }
}

impl Widget for &LifeWidget<'_> {
    /// Render this borrowed LifeWidget.
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
