use std::collections::{HashMap, HashSet};

use commons::geometry::{xy, XYRectangle, XY, XYZ};
use commons::grid::Grid;

use crate::model::piste::Piste;
use crate::systems::overlay;

#[derive(Default)]
pub struct Handler {
    selected_piste: Option<usize>,
}

pub struct Parameters<'a> {
    pub event: &'a engine::events::Event,
    pub mouse_xy: &'a Option<XY<u32>>,
    pub pistes: &'a HashMap<usize, Piste>,
    pub piste_map: &'a Grid<Option<usize>>,
    pub highlights: &'a mut HashSet<usize>,
    pub overlay: &'a mut overlay::System,
    pub graphics: &'a mut dyn engine::graphics::Graphics,
}

impl Handler {
    pub fn run(
        &mut self,
        Parameters {
            event,
            mouse_xy,
            pistes,
            piste_map,
            highlights,
            overlay,
            graphics,
        }: Parameters<'_>,
    ) {
        if !matches!(event, engine::events::Event::MouseMoved(..)) {
            return;
        }

        let Some(mouse_xy) = mouse_xy else { return };
        let Ok(XYZ { x, y, .. }) = graphics.world_xyz_at(mouse_xy) else {
            return;
        };
        let position = xy(
            (x.floor() as u32).min(piste_map.width() - 2),
            (y.floor() as u32).min(piste_map.height() - 2),
        );
        let selected_piste = piste_map[position];
        if self.selected_piste != selected_piste {
            if let Some(selected_piste) = self.selected_piste {
                highlights.remove(&selected_piste);
            }
            if let Some(selected_piste) = selected_piste {
                highlights.insert(selected_piste);
            }

            update_overlay(pistes, overlay, self.selected_piste);
            update_overlay(pistes, overlay, selected_piste);

            self.selected_piste = selected_piste;
        }
    }
}

fn update_overlay(
    pistes: &HashMap<usize, Piste>,
    overlay: &mut overlay::System,
    selected_piste: Option<usize>,
) {
    if let Some(selected_piste) = selected_piste {
        if let Some(piste) = pistes.get(&selected_piste) {
            let grid = &piste.grid;
            overlay.update(XYRectangle {
                from: *grid.origin(),
                to: *grid.origin() + xy(grid.width() - 2, grid.height() - 2),
            });
        }
    }
}
