use std::collections::{HashMap, HashSet};

use commons::geometry::{xy, XYRectangle, XY, XYZ};
use commons::grid::Grid;

use crate::model::piste::Piste;
use crate::systems::terrain_artist;

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
    pub terrain_artist: &'a mut terrain_artist::System,
    pub graphics: &'a mut dyn engine::graphics::Graphics,
}

impl Handler {
    pub fn handle(
        &mut self,
        Parameters {
            event,
            mouse_xy,
            pistes,
            piste_map,
            highlights,
            terrain_artist,
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

            update_terrain_artist(terrain_artist, pistes, self.selected_piste);
            update_terrain_artist(terrain_artist, pistes, selected_piste);

            self.selected_piste = selected_piste;
        }
    }
}

fn update_terrain_artist(
    terrain_artist: &mut terrain_artist::System,
    pistes: &HashMap<usize, Piste>,
    selected_piste: Option<usize>,
) {
    if let Some(selected_piste) = selected_piste {
        if let Some(piste) = pistes.get(&selected_piste) {
            let grid = &piste.grid;
            terrain_artist.update_overlay(XYRectangle {
                from: *grid.origin(),
                to: *grid.origin() + xy(grid.width() - 2, grid.height() - 2),
            });
        }
    }
}
