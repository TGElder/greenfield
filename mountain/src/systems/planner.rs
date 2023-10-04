use std::collections::{HashMap, HashSet};
use std::iter::once;

use commons::geometry::XY;
use commons::grid::Grid;
use network::model::Edge;
use rand::Rng;

use crate::model::piste::PisteCosts;
use crate::model::skiing::{Event, Mode, Plan, State};
use crate::network::skiing::SkiingNetwork;

use network::algorithms::find_best_within_steps::FindBestWithinSteps;

const MAX_STEPS: u64 = 8;
const MAX_DETOUR: u64 = 1;

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
    pub reserved: &'a mut Grid<bool>,
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
            reserved,
        }: Parameters<'_>,
    ) {
        self.add_new_finished(plans);

        self.finished.retain(|id| {
            let Some(current_plan) = plans.get_mut(id) else {
                return false;
            };

            free(current_plan, reserved);

            let plan = match current_plan {
                Plan::Moving(events) => {
                    let current_pair = events.windows(2).find(|pair| match pair {
                        [a, b] => *micros >= a.micros && *micros < b.micros,
                        _ => false,
                    });

                    match current_pair {
                        Some(pair) => Plan::Moving(pair.to_vec()),
                        None => Plan::Stationary(events.last().unwrap().state),
                    }
                }
                Plan::Stationary(state) => Plan::Stationary(*state),
            };


            let from = last_state(&plan);

            let onward_path = match (
                get_costs(id, locations, targets, distance_costs),
                get_costs(id, locations, targets, skiing_costs),
            ) {
                (Some(distance_costs), Some(skiing_costs)) => {
                    find_path(terrain, from, reserved, distance_costs, skiing_costs)
                }
                _ => None,
            };

            if let Some(onward_path) = onward_path {
                if !onward_path.is_empty() {
                    let last = onward_path.last().unwrap();
                    if last.to.mode != (Mode::Skiing { velocity: 0 }) {
                        println!("Current Plan = {:?}", current_plan);
                        println!("Plan = {:?}", plan);
                        println!("Onward = {:?}", onward_path);
                    }
                    match plan {
                        Plan::Stationary(_) => {
                            *current_plan = Plan::Moving(events(micros, onward_path));
                        }
                        Plan::Moving(mut e) => {
                            let start = e.pop().unwrap().micros;
                            e.append(&mut events(&start, onward_path));
                            *current_plan = Plan::Moving(e);
                        }
                    }
                } else {
                    if matches!(plan, Plan::Stationary(..)) {
                        *current_plan = plan;
                    }
                }
            } else {
                if matches!(plan, Plan::Stationary(..)) {
                    *current_plan = plan;
                }
            }

            reserve(current_plan, reserved);

            match current_plan {
                Plan::Stationary(_) => true,
                Plan::Moving(_) => false,
            }
        });
    }

    fn add_new_finished(&mut self, plans: &mut HashMap<usize, Plan>) {
        let new_finished = plans
            .iter_mut()
            .filter(|(id, _)| !self.finished.contains(id))
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

fn free(plan: &Plan, reserved: &mut Grid<bool>) {
    for position in iter_positions(plan) {
        reserved[position] = false
    }
}

fn reserve(plan: &Plan, reserved: &mut Grid<bool>) {
    for position in iter_positions(plan) {
        reserved[position] = true
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


fn find_path(
    terrain: &Grid<f32>,
    from: &State,
    reserved: &Grid<bool>,
    distance_costs: &HashMap<State, u64>,
    skiing_costs: &HashMap<State, u64>,
) -> Option<Vec<Edge<State>>> {
    let network = SkiingNetwork {
        terrain,
        reserved,
        distance_costs,
    };

    let mut rng = rand::thread_rng();

    // let steps = rng.gen_range(1..=MAX_STEPS);
    let steps = MAX_STEPS;

    network.find_best_within_steps(
        HashSet::from([*from]),
        &mut |_, state| {

            if let Mode::Skiing { velocity } = state.mode {
                if velocity != 0 {
                    return None;
                }
            }

            let Some(cost) = skiing_costs.get(state) else {
                return None;
            };

            // check for forbidden tiles
            if cost != &0 // goal tile is never forbidden
                && is_white_tile(&state.position)
            {
                return None;
            }

            Some(score(&mut rng, cost))
        },
        &mut |state| skiing_costs.contains_key(state),
        steps,
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
