use std::collections::HashSet;

use commons::color::Rgba;
use commons::geometry::{XYRectangle, XY};
use commons::grid::Grid;
use commons::origin_grid::OriginGrid;
use engine::graphics::Graphics;

use crate::draw::terrain;
use crate::handlers::selection;

pub const CLEAR: Rgba<u8> = Rgba::new(0, 0, 0, 0);

pub struct System {
    pub updates: Vec<XYRectangle<u32>>,
    pub colors: Colors,
}

pub struct Colors {
    pub selection: Rgba<u8>,
    pub piste: Rgba<u8>,
    pub piste_highlight: Rgba<u8>,
}

impl System {
    pub fn update(&mut self, update: XYRectangle<u32>) {
        self.updates.push(update);
    }

    pub fn run(
        &mut self,
        graphics: &mut dyn Graphics,
        drawing: Option<&terrain::Drawing>,
        piste_map: &Grid<Option<usize>>,
        highlights: &HashSet<usize>,
        selection: &selection::Handler,
    ) {
        let Some(drawing) = drawing else { return };

        for update in self.updates.drain(..) {
            let mut image = OriginGrid::from_rectangle(update, CLEAR);

            for position in update.iter() {
                image[position] = selection_color(self.colors.selection, &position, selection)
                    .or_else(|| {
                        piste_color(
                            self.colors.piste,
                            self.colors.piste_highlight,
                            &position,
                            piste_map,
                            highlights,
                        )
                    })
                    .unwrap_or(CLEAR);
            }

            drawing
                .modify_overlay(graphics, &image)
                .unwrap_or_else(|_| println!("WARN: Could not draw overlay"));
        }
    }
}

fn selection_color(
    color: Rgba<u8>,
    xy: &XY<u32>,
    selection: &selection::Handler,
) -> Option<Rgba<u8>> {
    let Some(rectangle) = selection.rectangle else {
        return None;
    };

    if rectangle.contains(xy) {
        Some(color)
    } else {
        None
    }
}

fn piste_color(
    color: Rgba<u8>,
    highlight: Rgba<u8>,
    position: &XY<u32>,
    piste_map: &Grid<Option<usize>>,
    highlights: &HashSet<usize>,
) -> Option<Rgba<u8>> {
    if let Some(piste_id) = piste_map[position] {
        if highlights.contains(&piste_id) {
            return Some(highlight);
        } else {
            return Some(color);
        }
    }

    None
}
