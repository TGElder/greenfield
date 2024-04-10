use std::collections::HashMap;

use commons::geometry::{xy, XYRectangle, XY, XYZ};
use commons::grid::Grid;
use engine::binding::Binding;

use crate::model::ability::Ability;
use crate::model::entrance::Entrance;
use crate::model::exit::Exit;
use crate::model::lift::Lift;
use crate::model::piste::{Costs, Piste};
use crate::model::reservation::Reservation;
use crate::services::clock;
use crate::systems::terrain_artist;
use crate::utils::computer;

pub struct Handler {
    pub binding: Binding,
}

pub struct Parameters<'a> {
    pub event: &'a engine::events::Event,
    pub mouse_xy: &'a Option<XY<u32>>,
    pub terrain: &'a Grid<f32>,
    pub pistes: &'a HashMap<usize, Piste>,
    pub piste_map: &'a Grid<Option<usize>>,
    pub lifts: &'a HashMap<usize, Lift>,
    pub entrances: &'a HashMap<usize, Entrance>,
    pub exits: &'a mut HashMap<usize, Vec<Exit>>,
    pub reservations: &'a Grid<HashMap<usize, Reservation>>,
    pub costs: &'a mut HashMap<usize, Costs>,
    pub abilities: &'a mut HashMap<usize, Ability>,
    pub clock: &'a mut clock::Service,
    pub terrain_artist: &'a mut terrain_artist::System,
    pub graphics: &'a mut dyn engine::graphics::Graphics,
}

impl Handler {
    pub fn handle(
        &mut self,
        Parameters {
            event,
            mouse_xy,
            terrain,
            pistes,
            piste_map,
            lifts,
            entrances,
            exits,
            reservations,
            costs,
            abilities,
            clock,
            terrain_artist,
            graphics,
        }: Parameters<'_>,
    ) {
        if !self.binding.binds_event(event) {
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
        let Some(piste_id) = piste_map[position] else {
            return;
        };

        let current_speed = clock.speed();
        clock.set_speed(0.0);

        computer::exit::compute_piste(&piste_id, pistes, piste_map, lifts, entrances, exits);
        computer::costs::compute_piste(&piste_id, pistes, terrain, exits, reservations, costs);
        computer::piste_ability::compute_piste(
            &piste_id, pistes, costs, exits, lifts, entrances, abilities,
        );
        computer::piste_network::compute_piste_network(
            piste_map,
            lifts,
            entrances,
            costs,
            Ability::Advanced,
        );

        if let Some(piste) = pistes.get(&piste_id) {
            let grid = &piste.grid;
            terrain_artist.update(XYRectangle {
                from: *grid.origin(),
                to: *grid.origin() + xy(grid.width() - 2, grid.height() - 2),
            });
        }

        clock.set_speed(current_speed);
    }
}
