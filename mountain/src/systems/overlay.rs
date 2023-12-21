use std::collections::{HashMap, HashSet};

use commons::color::Rgba;
use commons::geometry::{XYRectangle, XY};
use commons::grid::Grid;
use commons::origin_grid::OriginGrid;
use engine::graphics::Graphics;

use crate::draw::terrain;
use crate::handlers::selection;
use crate::model::reservation::Reservation;

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

impl Colors {
    fn selection_color(&self, xy: &XY<u32>, selection: &selection::Handler) -> Option<Rgba<u8>> {
        let Some(rectangle) = selection.rectangle else {
            return None;
        };

        if rectangle.contains(xy) {
            Some(self.selection)
        } else {
            None
        }
    }

    fn piste_color(
        &self,
        position: &XY<u32>,
        piste_map: &Grid<Option<usize>>,
        highlights: &HashSet<usize>,
    ) -> Option<Rgba<u8>> {
        if let Some(piste_id) = piste_map[position] {
            if highlights.contains(&piste_id) {
                return Some(self.piste_highlight);
            } else {
                return Some(self.piste);
            }
        }

        None
    }

    fn reserved_color(
        &self,
        position: &XY<u32>,
        micros: &u128,
        reserved: &Grid<HashMap<usize, Reservation>>,
    ) -> Option<Rgba<u8>> {
        if reserved[position]
            .values()
            .any(|reservation| reservation.is_reserved(micros))
        {
            Some(Rgba::new(255, 0, 0, 128))
        } else {
            None
        }
    }
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
        micros: &u128,
        reserved: &Grid<HashMap<usize, Reservation>>,
    ) {
        let Some(drawing) = drawing else { return };

        for update in self.updates.drain(..) {
            let mut image = OriginGrid::from_rectangle(update, CLEAR);

            for position in update.iter() {
                image[position] = self
                    .colors
                    .selection_color(&position, selection)
                    // .or_else(|| self.colors.reserved_color(&position, micros, reserved))
                    .or_else(|| self.colors.piste_color(&position, piste_map, highlights))
                    .unwrap_or(CLEAR);
            }

            drawing
                .modify_overlay(graphics, &image)
                .unwrap_or_else(|_| println!("WARN: Could not draw overlay"));
        }
    }
}
