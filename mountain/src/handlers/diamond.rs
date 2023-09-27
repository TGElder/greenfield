use std::collections::{HashMap, HashSet};
use std::iter::once;

use commons::geometry::{xy, XY, XYZ};
use commons::grid::Grid;
use engine::binding::Binding;
use network::algorithms::costs_to_target::CostsToTarget;
use network::model::{Edge, InNetwork, OutNetwork};

use crate::model::direction::DIRECTIONS;
use crate::model::piste::PisteCosts;
use crate::model::skiing::{Mode, State};
use crate::network::skiing::SkiingNetwork;
use crate::network::velocity_encoding::VELOCITY_LEVELS;

pub struct Handler {
    pub binding: Binding,
}

impl Handler {
    pub fn handle(
        &self,
        event: &engine::events::Event,
        mouse_xy: &Option<XY<u32>>,
        graphics: &mut dyn engine::graphics::Graphics,
        terrain: &Grid<f32>,
        distance_costs: &HashMap<usize, PisteCosts>,
        piste_map: &Grid<Option<usize>>,
        diamond: &mut HashSet<XY<u32>>,
    ) {
        if !self.binding.binds_event(event) {
            return;
        }

        let Some(mouse_xy) = mouse_xy else { return };
        let Ok(XYZ { x, y, .. }) = graphics.world_xyz_at(mouse_xy) else {
            return;
        };
        let position = xy(x.round() as u32, y.round() as u32);
        let Some(piste) = piste_map[position] else {
            return;
        };

        let Some(distance_costs) = distance_costs.get(&piste) else {
            return;
        };

        let Some(target) = distance_costs.lifts().next() else {
            return;
        };
        let Some(distance_costs) = distance_costs.costs(target) else {
            return;
        };

        let network = SkiingNetwork {
            terrain,
            reserved: &terrain.map(|_, _| false),
            distance_costs,
        };

        let costs = network.costs_to_target(&states_for_position(position).collect());

        *diamond = costs.keys().map(|state| state.position).collect();
    }
}

impl InNetwork<State> for SkiingNetwork<'_> {
    fn edges_in<'a>(
        &'a self,
        to: &'a State,
    ) -> Box<dyn Iterator<Item = network::model::Edge<State>> + 'a> {
        Box::new(self.edges_out(to).map(|edge| Edge {
            from: edge.to,
            to: edge.from,
            ..edge
        }))
    }
}

fn states_for_position(position: XY<u32>) -> impl Iterator<Item = State> {
    DIRECTIONS.into_iter().flat_map(move |travel_direction| {
        modes().map(move |mode| State {
            position,
            mode,
            travel_direction,
        })
    })
}

fn modes() -> impl Iterator<Item = Mode> {
    (0..VELOCITY_LEVELS)
        .map(|velocity| Mode::Skiing { velocity })
        .chain(once(Mode::Walking))
}
