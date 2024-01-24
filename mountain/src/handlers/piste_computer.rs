use std::collections::HashMap;

use commons::geometry::{xy, XY, XYZ};
use commons::grid::Grid;
use engine::binding::Binding;

use crate::model::entrance::Entrance;
use crate::model::exit::Exit;
use crate::model::lift::Lift;
use crate::model::piste::{Costs, Piste};
use crate::services::clock;
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
    pub distance_costs: &'a mut HashMap<usize, Costs>,
    pub clock: &'a mut clock::Service,
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
            distance_costs,
            clock,
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

        computer::exit::compute_piste(&piste_id, pistes, lifts, entrances, exits);
        computer::distance_network::compute_piste(
            &piste_id,
            pistes,
            terrain,
            exits,
            distance_costs,
        );

        clock.set_speed(current_speed);
    }
}
