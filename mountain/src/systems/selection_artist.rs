use commons::color::Rgba;
use commons::geometry::PositionedRectangle;
use commons::grid::Grid;
use engine::graphics::Graphics;

use crate::draw::terrain;

const CLEAR_COLOR: Rgba<u8> = Rgba::new(0, 0, 0, 0);

pub struct SelectionArtist {
    pub drawn_selection: Option<PositionedRectangle<u32>>,
    pub selection_color: Rgba<u8>,
}

impl SelectionArtist {
    pub fn run(
        &mut self,
        graphics: &mut dyn Graphics,
        drawing: Option<&terrain::Drawing>,
        selection: &Option<PositionedRectangle<u32>>,
    ) {
        if self.drawn_selection == *selection {
            return;
        }
        let Some(drawing) = drawing else {return};

        self.draw_selection(drawing, graphics, CLEAR_COLOR);
        self.drawn_selection = *selection;
        self.draw_selection(drawing, graphics, self.selection_color);
    }

    fn draw_selection(
        &mut self,
        drawing: &terrain::Drawing,
        graphics: &mut dyn Graphics,
        color: Rgba<u8>,
    ) {
        if let Some(selection) = self.drawn_selection {
            drawing
                .modify_overlay(
                    graphics,
                    &selection.from,
                    &Grid::from_element(selection.width() + 1, selection.height() + 1, color),
                )
                .unwrap_or_else(|_| println!("WARN: Could not draw selection"));
        }
    }
}
