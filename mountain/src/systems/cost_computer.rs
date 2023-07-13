use std::collections::{HashMap, HashSet};
use std::iter::once;

use commons::geometry::{xy, XY};
use commons::grid::Grid;
use network::model::{Edge, InNetwork};

use crate::model::skiing::{Mode, State};
use crate::model::{Lift, Piste, PisteCosts, DIRECTIONS};
use crate::network::skiing::{SkiingInNetwork, SkiingNetwork};
use crate::network::velocity_encoding::{encode_velocity, VELOCITY_LEVELS};
use network::algorithms::costs_to_target::CostsToTarget;

pub fn run(
    terrain: &Grid<f32>,
    pistes: &HashMap<usize, Piste>,
    piste_costs: &mut HashMap<usize, PisteCosts>,
    lifts: &HashMap<usize, Lift>,
) {
    for (piste_index, piste) in pistes.iter() {
        let costs = compute_costs(terrain, piste, lifts);
        piste_costs.insert(*piste_index, costs);
    }
}

fn compute_costs(terrain: &Grid<f32>, piste: &Piste, lifts: &HashMap<usize, Lift>) -> PisteCosts {
    let mut out = PisteCosts::new();

    let network = SkiingNetwork {
        terrain,
        reserved: &terrain.map(|_, _| false),
    };
    let piste_positions = piste_positions(piste);
    let network = Network { terrain, piste };

    for (
        lift,
        Lift {
            from,
            max_entry_velocity,
            ..
        },
    ) in lifts
    {
        let grid = &piste.grid;
        if grid.in_bounds(from) && grid[from] {
            let costs = compute_costs_for_position(&network, from, max_entry_velocity);
            let coverage =
                costs.len() as f32 / (piste_positions.len() * DIRECTIONS.len() * 9) as f32;
            println!("INFO: Coverage for lift {} = {}", lift, coverage);
            out.set_costs(*lift, costs)
        }
    }

    out
}

fn piste_positions(piste: &Piste) -> HashSet<XY<u32>> {
    piste
        .grid
        .iter()
        .filter(|position| piste.grid[position])
        .collect::<HashSet<_>>()
}

fn compute_costs_for_position(
    network: &Network,
    position: &XY<u32>,
    max_velocity: &f32,
) -> HashMap<State, u64> {
    let costs = network.costs_to_target(&HashSet::from_iter([*position]));
    state_costs(&costs, network.piste)
}

fn state_costs(costs: &HashMap<XY<u32>, u64>, piste: &Piste) -> HashMap<State, u64> {
    piste
        .grid
        .iter()
        .flat_map(|xy| {
            DIRECTIONS.iter().flat_map(move |direction| {
                (0..VELOCITY_LEVELS)
                    .map(move |velocity| State {
                        position: xy,
                        mode: Mode::Skiing { velocity },
                        travel_direction: *direction,
                    })
                    .chain(once(State {
                        position: xy,
                        mode: Mode::Walking,
                        travel_direction: *direction,
                    }))
            })
        })
        .flat_map(|state| costs.get(&state.position).map(|cost| (state, *cost)))
        .collect()
}

struct Network<'a> {
    terrain: &'a Grid<f32>,
    piste: &'a Piste,
}

impl<'a> InNetwork<XY<u32>> for Network<'a> {
    fn edges_in<'b>(
        &'b self,
        to: &'b XY<u32>,
    ) -> Box<dyn Iterator<Item = network::model::Edge<XY<u32>>> + 'b> {
        Box::new(
            [
                xy(-1, -1),
                xy(0, -1),
                xy(1, -1),
                xy(-1, 0),
                xy(1, 0),
                xy(-1, 1),
                xy(0, 1),
                xy(1, 1),
            ]
            .into_iter()
            .flat_map(move |offset| {
                let n = self.terrain.offset(to, offset)?;

                if self.piste.grid.in_bounds(n) && self.piste.grid[n] {
                    Some(Edge {
                        from: n,
                        to: *to,
                        cost: (((offset.x.abs() + offset.y.abs()) as f32).sqrt() * 100.0) as u32,
                    })
                } else {
                    None
                }
            }),
        )
    }
}
