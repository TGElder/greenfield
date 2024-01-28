use std::collections::{HashMap, HashSet};
use std::iter::once;

use commons::geometry::XY;
use commons::grid::Grid;
use network::model::Edge;
use rand::Rng;

use crate::model::hash_vec::HashVec;
use crate::model::piste::{Costs, Piste};
use crate::model::reservation::Reservation;
use crate::model::skiing::{Event, Plan, State};
use crate::network::skiing::SkiingNetwork;

use network::algorithms::find_best_within_steps::FindBestWithinSteps;

const MAX_STEPS: u64 = 16;
const MAX_DETOUR: u64 = 4;

pub struct Parameters<'a> {
    pub terrain: &'a Grid<f32>,
    pub micros: &'a u128,
    pub plans: &'a mut HashMap<usize, Plan>,
    pub locations: &'a HashMap<usize, usize>,
    pub targets: &'a HashMap<usize, usize>,
    pub pistes: &'a HashMap<usize, Piste>,
    pub costs: &'a HashMap<usize, Costs>,
    pub reservations: &'a mut Grid<HashMap<usize, Reservation>>,
    pub planning_queue: &'a mut HashVec<usize>,
}

pub fn run(
    Parameters {
        terrain,
        micros,
        plans,
        locations,
        targets,
        pistes,
        costs,
        reservations,
        planning_queue,
    }: Parameters<'_>,
) {
    add_new_finished(planning_queue, plans, micros);

    planning_queue.retain(|id| {
        let Some(current_plan) = plans.get_mut(id) else {
            return false;
        };

        let Some(location) = locations.get(id) else {
            return false;
        };

        let Some(piste) = pistes.get(location) else {
            return false;
        };

        free(id, current_plan, reservations);

        let from = last_state(current_plan);
        *current_plan = match get_costs(id, locations, targets, costs) {
            Some(costs) => new_plan(terrain, micros, from, piste, reservations, costs),
            _ => brake(*from),
        };
        reserve(id, current_plan, reservations);

        match current_plan {
            Plan::Stationary(_) => true,
            Plan::Moving(_) => false,
        }
    });
}

fn add_new_finished(
    planning_queue: &mut HashVec<usize>,
    plans: &mut HashMap<usize, Plan>,
    micros: &u128,
) {
    let new_finished = plans
        .iter_mut()
        .filter(|(id, _)| !planning_queue.contains(id))
        .filter(|(_, plan)| finished(plan, micros))
        .map(|(id, _)| id)
        .collect::<Vec<_>>();

    for id in new_finished {
        planning_queue.push(*id);
    }
}

fn finished(current_plan: &Plan, micros: &u128) -> bool {
    if let Plan::Moving(ref events) = current_plan {
        if let Some(last) = events.last() {
            if *micros < last.micros {
                return false;
            }
        }
    }
    true
}

fn free(id: &usize, plan: &Plan, reservations: &mut Grid<HashMap<usize, Reservation>>) {
    for position in iter_positions(plan) {
        reservations[position].remove(id);
    }
}

fn iter_positions<'a>(plan: &'a Plan) -> Box<dyn Iterator<Item = XY<u32>> + 'a> {
    match plan {
        Plan::Stationary(state) => Box::new(once(state.position)),
        Plan::Moving(events) => Box::new(
            events
                .iter()
                .map(|event| event.state)
                .map(|state| state.position),
        ),
    }
}

fn reserve(id: &usize, plan: &Plan, reservations: &mut Grid<HashMap<usize, Reservation>>) {
    match plan {
        Plan::Stationary(state) => {
            reservations[state.position].insert(*id, Reservation::Permanent);
        }
        Plan::Moving(events) => {
            for pair in events.windows(2) {
                reservations[pair[0].state.position]
                    .insert(*id, Reservation::Until(pair[1].micros));
            }
            if let Some(event) = events.last() {
                reservations[event.state.position].insert(*id, Reservation::Permanent);
            }
        }
    }
}

fn last_state(plan: &Plan) -> &State {
    match plan {
        Plan::Stationary(state) => state,
        Plan::Moving(events) => &events.last().unwrap().state,
    }
}

fn get_costs<'a>(
    id: &usize,
    locations: &HashMap<usize, usize>,
    targets: &HashMap<usize, usize>,
    costs: &'a HashMap<usize, Costs>,
) -> Option<&'a HashMap<State, u64>> {
    let location = locations.get(id)?;
    let target = targets.get(id)?;
    let costs = costs.get(location)?;
    costs.costs(target)
}

fn new_plan(
    terrain: &Grid<f32>,
    micros: &u128,
    from: &State,
    piste: &Piste,
    reservations: &Grid<HashMap<usize, Reservation>>,
    costs: &HashMap<State, u64>,
) -> Plan {
    match find_path(terrain, micros, from, piste, reservations, costs) {
        Some(edges) => {
            if edges.is_empty() {
                brake(*from)
            } else {
                Plan::Moving(events(micros, edges))
            }
        }
        None => brake(*from),
    }
}

fn find_path(
    terrain: &Grid<f32>,
    micros: &u128,
    from: &State,
    piste: &Piste,
    reservations: &Grid<HashMap<usize, Reservation>>,
    costs: &HashMap<State, u64>,
) -> Option<Vec<Edge<State>>> {
    let network = SkiingNetwork {
        terrain,
        is_accessible_fn: &|position| {
            !reservations[position]
                .values()
                .any(|reservation| reservation.is_valid_at(micros))
        },
        is_valid_edge_fn: &|a, b| match (costs.get(&a.stationary()), costs.get(&b.stationary())) {
            (Some(from), Some(to)) => to < from,
            _ => false,
        },
    };

    let mut rng = rand::thread_rng();

    let steps = MAX_STEPS;

    network
        .find_best_within_steps(
            HashSet::from([*from]),
            &mut |_, state| {
                if state.position == from.position {
                    return None;
                }

                let Some(cost) = costs.get(&state.stationary()) else {
                    return None;
                };

                if *cost != 0 && is_white_tile(&state.position) {
                    return None;
                }

                Some(score(&mut rng, cost))
            },
            &mut |state| piste.grid.in_bounds(state.position) && piste.grid[state.position],
            steps,
        )
        .map(|result| result.path)
}

fn is_white_tile(position: &XY<u32>) -> bool {
    position.x % 2 == position.y % 2
}

fn score<R>(rng: &mut R, cost: &u64) -> Score
where
    R: Rng,
{
    Score {
        cost: rng.gen_range(*cost..=cost * MAX_DETOUR),
    }
}

fn brake(state: State) -> Plan {
    Plan::Stationary(state.stationary())
}

fn events(start: &u128, edges: Vec<Edge<State>>) -> Vec<Event> {
    let mut out = Vec::with_capacity(edges.len());
    let mut micros = *start;
    let last_i = edges.len() - 1;
    for (i, edge) in edges.into_iter().enumerate() {
        out.push(Event {
            micros,
            state: edge.from,
        });
        micros += edge.cost as u128;
        if i == last_i {
            out.push(Event {
                micros,
                state: edge.to,
            });
        }
    }
    out
}

#[derive(Eq)]
struct Score {
    cost: u64,
}

impl PartialEq for Score {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Ord for Score {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
