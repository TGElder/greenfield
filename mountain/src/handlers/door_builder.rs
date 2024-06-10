use std::collections::{HashMap, HashSet};

use commons::geometry::{xy, XYRectangle, XY};
use commons::grid::Grid;
use engine::binding::Binding;

use crate::handlers::selection;
use crate::model::building::Building;

use crate::model::direction::{Direction, DIRECTIONS};
use crate::model::door::Door;

use crate::model::entrance::Entrance;
use crate::model::piste::Piste;
use crate::model::skiing::State;
use crate::services::id_allocator;

pub struct Handler {
    pub binding: Binding,
}

pub struct Parameters<'a> {
    pub event: &'a engine::events::Event,
    pub pistes: &'a HashMap<usize, Piste>,
    pub terrain: &'a Grid<f32>,
    pub selection: &'a mut selection::Handler,
    pub buildings: &'a HashMap<usize, Building>,
    pub id_allocator: &'a mut id_allocator::Service,
    pub doors: &'a mut HashMap<usize, Door>,
    pub entrances: &'a mut HashMap<usize, Entrance>,
}

impl Handler {
    pub fn handle(
        &mut self,
        Parameters {
            event,
            pistes,
            selection,
            buildings,
            terrain,
            id_allocator,
            doors,
            entrances,
        }: Parameters<'_>,
    ) {
        if !self.binding.binds_event(event) {
            return;
        }

        let Some(grid) = &selection.grid else {
            return;
        };
        if grid.width() != 1 && grid.height() != 1 {
            println!("WARN: Door must be 1 wide or 1 high");
            selection.clear_selection();
            return;
        }

        let longest_side_cell_count = grid.width().max(grid.height());
        if longest_side_cell_count < 2 {
            println!("WARN: Door must be at least 2 wide or 2 high");
            selection.clear_selection();
            return;
        }

        let rectangle = XYRectangle {
            from: *grid.origin(),
            to: *grid.origin() + xy(grid.width(), grid.height()),
        };

        let longest_side_position_count = longest_side_cell_count as usize + 1;
        let Some((building_id, building)) = buildings.iter().find(|(_building_id, building)| {
            rectangle
                .iter()
                .filter(|position| building.footprint.contains(position))
                .count()
                == longest_side_position_count
        }) else {
            println!(
                "WARN: Door must contain {} postions from the same building",
                longest_side_position_count
            );
            selection.clear_selection();
            return;
        };

        if building.under_construction {
            println!("WARN: Door cannot be added to building under construction");
            selection.clear_selection();
            return;
        }

        let (building_positions, piste_positions): (Vec<_>, Vec<_>) = rectangle
            .iter()
            .partition(|position| building.footprint.contains(position));
        let Some((piste_id, _)) = pistes.iter().find(|(_, piste)| {
            let grid = &piste.grid;
            piste_positions
                .iter()
                .all(|position| grid.in_bounds(position) && grid[position])
        }) else {
            println!(
                "WARN: Door must contain {} postions from the same piste",
                longest_side_position_count
            );
            selection.clear_selection();
            return;
        };

        let aperture = piste_positions
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != 0 && i != piste_positions.len() - 1) // removing first and last position
            .map(|(_, position)| *position)
            .collect();
        let door_id = id_allocator.next_id();
        doors.insert(
            door_id,
            Door {
                building_id: *building_id,
                piste_id: *piste_id,
                footprint: rectangle,
                direction: direction(&piste_positions, &building_positions),
                aperture,
            },
        );

        entrances.insert(
            door_id,
            Entrance {
                destination_piste_id: *piste_id,
                stationary_states: stationary_states(&piste_positions),
                altitude_meters: altitude_meters(terrain, &piste_positions),
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

fn stationary_states(piste_positions: &[XY<u32>]) -> HashSet<State> {
    piste_positions
        .iter()
        .flat_map(|&position| {
            DIRECTIONS.iter().map(move |&travel_direction| State {
                position,
                velocity: 0,
                travel_direction,
            })
        })
        .collect::<HashSet<_>>()
}

fn altitude_meters(terrain: &Grid<f32>, piste_positions: &[XY<u32>]) -> f32 {
    piste_positions
        .iter()
        .map(|position| terrain[position])
        .sum::<f32>()
        / piste_positions.len() as f32
}
