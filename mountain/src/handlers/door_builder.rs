use std::collections::{HashMap, HashSet};

use commons::geometry::{xy, XYRectangle, XY};
use engine::binding::Binding;

use crate::handlers::selection;
use crate::model::building::Building;

use crate::model::direction::Direction;
use crate::model::door::Door;

use crate::model::piste::Piste;
use crate::services::id_allocator;

pub struct Handler {
    pub binding: Binding,
}

pub struct Parameters<'a> {
    pub event: &'a engine::events::Event,
    pub pistes: &'a HashMap<usize, Piste>,
    pub selection: &'a mut selection::Handler,
    pub buildings: &'a HashMap<usize, Building>,
    pub id_allocator: &'a mut id_allocator::Service,
    pub doors: &'a mut HashMap<usize, Door>,
}

impl Handler {
    pub fn handle(
        &mut self,
        Parameters {
            event,
            pistes,
            selection,
            buildings,
            doors,
            id_allocator,
        }: Parameters<'_>,
    ) {
        if !self.binding.binds_event(event) {
            return;
        }

        let Some(grid) = &selection.grid else {
            return;
        };
        if grid.width() != 1 || grid.height() != 1 {
            println!("WARN: Door must be 1x1");
            selection.clear_selection();
            return;
        }
        let rectangle = XYRectangle {
            from: *grid.origin(),
            to: *grid.origin() + xy(grid.width(), grid.height()),
        };

        let Some((building_id, building)) = buildings.iter().find(|(_building_id, building)| {
            rectangle
                .iter()
                .filter(|position| building.footprint.contains(position))
                .count()
                == 2
        }) else {
            println!("WARN: Door must contain two postions from the same building");
            selection.clear_selection();
            return;
        };

        let (building_positions, piste_positions): (Vec<_>, Vec<_>) = rectangle
            .iter()
            .partition(|position| building.footprint.contains(position));
        let Some((piste_id, _)) = pistes.iter().find(|(_, piste)| {
            let grid = &piste.grid;
            piste_positions
                .iter()
                .all(|position| grid.in_bounds(position) && grid[position])
        }) else {
            println!("WARN: Door must contain two postions from the same piste");
            selection.clear_selection();
            return;
        };

        let door_id = id_allocator.next_id();
        doors.insert(
            door_id,
            Door {
                building_id: *building_id,
                piste_id: *piste_id,
                footprint: rectangle,
                direction: direction(&piste_positions, &building_positions),
                aperture: HashSet::from_iter(piste_positions),
            },
        );
        selection.clear_selection();
    }
}

fn direction(piste_positions: &[XY<u32>], building_positions: &[XY<u32>]) -> Direction {
    let vector = xy(
        (piste_positions[0].x as f32 + piste_positions[1].x as f32)
            - (building_positions[0].x as f32 + building_positions[1].x as f32),
        (piste_positions[0].y as f32 + piste_positions[1].y as f32)
            - (building_positions[0].y as f32 + building_positions[1].y as f32),
    );
    let angle = vector.angle();
    Direction::snap_to_direction(angle)
}
