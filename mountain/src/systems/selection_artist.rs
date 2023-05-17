use commons::color::Rgba;
use commons::geometry::PositionedRectangle;
use commons::grid::Grid;
use engine::graphics::Graphics;

use crate::draw;

const CLEAR_COLOR: Rgba<u8> = Rgba::new(0, 0, 0, 0);

pub struct SelectionArtist {
    pub previous_selection: Option<PositionedRectangle<u32>>,
    pub selection_color: Rgba<u8>,
}

impl SelectionArtist {
    pub fn run(
        &mut self,
        graphics: &mut dyn Graphics,
        terrain_drawing: Option<&draw::terrain::Drawing>,
        selection: &Option<PositionedRectangle<u32>>,
    ) {
        if self.previous_selection != *selection {
            if let Some(terrain_drawing) = terrain_drawing {
                if let Some(selection) = self.previous_selection {
                    terrain_drawing
                        .modify_overlay(
                            graphics,
                            &selection.from,
                            &Grid::from_element(
                                selection.width() + 1,
                                selection.height() + 1,
                                CLEAR_COLOR,
                            ),
                        )
                        .unwrap_or_else(|_| {
                            println!("WARN: Could not clear previous selection from overlay")
                        });
                }

                if let Some(selection) = selection {
                    terrain_drawing
                        .modify_overlay(
                            graphics,
                            &selection.from,
                            &Grid::from_element(
                                selection.width() + 1,
                                selection.height() + 1,
                                self.selection_color,
                            ),
                        )
                        .unwrap_or_else(|_| {
                            println!("WARN: Could not draw previous selection on overlay")
                        });
                }
            }

            self.previous_selection = *selection;
        }
    }
}
