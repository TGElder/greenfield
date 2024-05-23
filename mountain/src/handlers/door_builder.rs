use std::collections::{HashMap, HashSet};

use commons::geometry::{xy, XYRectangle};
use engine::binding::Binding;

use crate::handlers::selection;
use crate::model::building::Building;

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
            println!("WARN: Door must contain two points from the same building");
            selection.clear_selection();
            return;
        };

        let piste_points = rectangle
            .iter()
            .filter(|position| !building.footprint.contains(position))
            .collect::<HashSet<_>>();
        let Some((piste_id, _)) = pistes.iter().find(|(_, piste)| {
            let grid = &piste.grid;
            piste_points
                .iter()
                .all(|position| grid.in_bounds(position) && grid[position])
        }) else {
            println!("WARN: Door must contain two points from the same piste");
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
                aperture: piste_points,
            },
        );
        selection.clear_selection();
    }
}
