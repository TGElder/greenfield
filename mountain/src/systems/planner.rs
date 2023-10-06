use std::collections::{HashMap, HashSet};
use std::iter::once;

use commons::geometry::XY;
use commons::grid::Grid;
use commons::origin_grid::OriginGrid;
use network::algorithms::find_path::FindPath;
use network::model::Edge;
use rand::Rng;

use crate::model::direction::DIRECTIONS;
use crate::model::lift::Lift;
use crate::model::piste::{Piste, PisteCosts};
use crate::model::reservation::Reservation;
use crate::model::skiing::{Event, Mode, Plan, State};
use crate::network::skiing::SkiingNetwork;
use crate::network::velocity_encoding::{VELOCITY_LEVELS, decode_velocity};

use network::algorithms::find_best_within_steps::FindBestWithinSteps;

const MAX_STEPS: u64 = 32;
const MAX_DETOUR: u64 = 2;

pub struct System {
    finished: HashVec,
}

pub struct Parameters<'a> {
    pub terrain: &'a Grid<f32>,
    pub micros: &'a u128,
    pub plans: &'a mut HashMap<usize, Plan>,
    pub locations: &'a HashMap<usize, usize>,
    pub targets: &'a HashMap<usize, usize>,
    pub distance_costs: &'a HashMap<usize, PisteCosts>,
    pub skiing_costs: &'a HashMap<usize, PisteCosts>,
    pub lifts: &'a HashMap<usize, Lift>,
    pub pistes: &'a HashMap<usize, Piste>,
    pub reserved: &'a mut Grid<Vec<Reservation>>,
}

impl System {
    pub fn new() -> System {
        System {
            finished: HashVec::new(),
        }
    }

    pub fn run(
        &mut self,
        Parameters {
            terrain,
            micros,
            plans,
            locations,
            targets,
            distance_costs,
            skiing_costs,
            lifts,
            pistes,
            reserved,
        }: Parameters<'_>,
    ) {
        self.add_new_finished(plans, micros);

        self.finished.retain(|id| {
            let Some(current_plan) = plans.get_mut(id) else {
                return false;
            };

            let Some(target) = targets.get(id) else {
                return false;
            };

            let Some(lift) = lifts.get(target) else {
                return false;
            };

            let Some(location) = locations.get(id) else {
                return false;
            };

            let Some(piste) = pistes.get(location) else {
                return false;
            };

            let to = &lift.from;

            free(id, current_plan, reserved);
            let from = last_state(current_plan);
            let from = &State {
                micros: *micros,
                ..*from
            };
            *current_plan = new_plan(terrain, micros, from, to, reserved, &piste.grid);
            reserve(id, current_plan, reserved, micros);

            match current_plan {
                Plan::Stationary(_) => true,
                Plan::Moving(_) => false,
            }
        });
    }

    fn add_new_finished(&mut self, plans: &mut HashMap<usize, Plan>, micros: &u128) {
        let new_finished = plans
            .iter_mut()
            .filter(|(id, _)| !self.finished.contains(id))
            .filter(|(_, plan)| finished(plan, micros))
            .map(|(id, _)| id)
            .collect::<Vec<_>>();

        for id in new_finished {
            self.finished.push(*id);
        }
    }
}

struct HashVec {
    waiting: HashSet<usize>,
    queue: Vec<usize>,
}

impl HashVec {
    fn new() -> HashVec {
        HashVec {
            waiting: HashSet::new(),
            queue: Vec::new(),
        }
    }

    fn contains(&self, value: &usize) -> bool {
        self.waiting.contains(value)
    }

    fn push(&mut self, value: usize) {
        self.waiting.insert(value);
        self.queue.push(value);
    }

    fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&usize) -> bool,
    {
        self.queue.retain(|value| {
            let out = f(value);
            if !out {
                self.waiting.remove(value);
            }
            out
        })
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

fn free(id: &usize, plan: &Plan, reserved: &mut Grid<Vec<Reservation>>) {
    for position in iter_positions(plan) {
        reserved[position].retain(|reservation| reservation.id() != id)
    }
}

fn reserve(id: &usize, plan: &Plan, reserved: &mut Grid<Vec<Reservation>>, micros: &u128) {
    match plan {
        Plan::Stationary(State { position, .. }) => {
            reserved[position].push(Reservation::Permanent {
                id: *id,
                from: *micros,
            })
        }
        Plan::Moving(events) => {
            events.windows(2).for_each(|pair| match pair {
                [a, b] => {
                    reserved[a.state.position].push(Reservation::Temporary {
                        id: *id,
                        from: a.micros,
                        to: b.micros,
                    });
                    reserved[b.state.position].push(Reservation::Temporary {
                        id: *id,
                        from: a.micros,
                        to: b.micros,
                    });
                }
                _ => (),
            });
            // events.last().iter().for_each(
            //     |Event {
            //          micros,
            //          state: State { position, .. },
            //      }| {
            //         reserved[position].push(Reservation::Permanent {
            //             id: *id,
            //             from: *micros,
            //         })
            //     },
            // )
        }
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
    costs: &'a HashMap<usize, PisteCosts>,
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
    to: &XY<u32>,
    reserved: &Grid<Vec<Reservation>>,
    piste: &OriginGrid<bool>,
) -> Plan {
    match find_path(micros, terrain, from, to, reserved, piste) {
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
    micros: &u128,
    terrain: &Grid<f32>,
    from: &State,
    to: &XY<u32>,
    reserved: &Grid<Vec<Reservation>>,
    piste: &OriginGrid<bool>,
) -> Option<Vec<Edge<State>>> {
    let network = SkiingNetwork {
        terrain,
        reserved,
        piste,
    };

    let max_velocity = decode_velocity(&(VELOCITY_LEVELS - 1)).unwrap();

    network.find_path(
        HashSet::from([*from]),
        &|state| state.position == *to,
        &|_, state| ((((state.position.x.abs_diff(to.x).pow(2) + state.position.y.abs_diff(to.y).pow(2)) as f32).sqrt() / max_velocity) * 1_000_000.0) as u64,
    )
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
    let mode = match state.mode {
        Mode::Walking => Mode::Walking,
        Mode::Skiing { .. } => Mode::Skiing { velocity: 0 },
    };
    Plan::Stationary(State { mode, ..state })
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

fn skiing_states_for_position(position: XY<u32>) -> impl Iterator<Item = State> {
    DIRECTIONS
        .iter()
        .copied()
        .flat_map(move |travel_direction| {
            (0..=VELOCITY_LEVELS).map(move |velocity| State {
                position,
                mode: Mode::Skiing { velocity },
                travel_direction,
                micros: 0,
            })
        })
}
