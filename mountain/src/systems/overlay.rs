use std::collections::HashMap;

use commons::color::Rgba;
use commons::geometry::{xy, XYRectangle, XY};
use commons::origin_grid::OriginGrid;
use engine::graphics::Graphics;

use crate::draw::terrain;
use crate::model::Piste;

pub const CLEAR: Rgba<u8> = Rgba::new(0, 0, 0, 0);

pub struct System {
    updates: Vec<XYRectangle<u32>>,
    colors: Colors,
}

pub struct Colors {
    pub selection: Rgba<u8>,
    pub piste: Rgba<u8>,
}

impl System {
    pub fn new(colors: Colors) -> System {
        System {
            updates: vec![],
            colors,
        }
    }

    pub fn update(&mut self, update: XYRectangle<u32>) {
        self.updates.push(update);
    }

    pub fn run(
        &mut self,
        graphics: &mut dyn Graphics,
        drawing: Option<&terrain::Drawing>,
        selection: &Option<XYRectangle<u32>>,
        pistes: &HashMap<usize, Piste>,
    ) {
        let Some(drawing) = drawing else {return};

        for update in self.updates.drain(..) {
            let mut image = OriginGrid::from_rectangle(update, CLEAR);

            let XYRectangle { from, to } = update;
            for x in from.x..=to.x {
                for y in from.y..=to.y {
                    let position = xy(x, y);
                    image[position] = selection_color(self.colors.selection, &position, selection)
                        .or_else(|| piste_color(self.colors.piste, &position, pistes))
                        .unwrap_or(CLEAR);
                }
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
    selection: &Option<XYRectangle<u32>>,
) -> Option<Rgba<u8>> {
    let Some(selection) = selection else {return None};

    if selection.contains(xy) {
        Some(color)
    } else {
        None
    }
}

fn piste_color(color: Rgba<u8>, xy: &XY<u32>, pistes: &HashMap<usize, Piste>) -> Option<Rgba<u8>> {
    for piste in pistes.values() {
        if piste.grid.in_bounds(xy) && piste.grid[xy] {
            return Some(color);
        }
    }

    None
}