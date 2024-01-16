use std::collections::{HashMap, HashSet};
use std::iter::once;

use commons::geometry::XY;
use commons::grid::Grid;
use network::model::Edge;
use rand::Rng;

use crate::model::piste::{Costs, Piste};
use crate::model::reservation::Reservation;
use crate::model::skiing::{Ability, Event, Plan, State};
use crate::network::skiing::SkiingNetwork;

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
    pub pistes: &'a HashMap<usize, Piste>,
    pub abilities: &'a HashMap<usize, Ability>,
    pub distance_costs: &'a HashMap<usize, Costs>,
    pub skiing_costs: &'a HashMap<usize, HashMap<Ability, Costs>>,
    pub reservations: &'a mut Grid<HashMap<usize, Reservation>>,
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
            pistes,
            abilities,
            distance_costs,
            skiing_costs,
            reservations,
        }: Parameters<'_>,
    ) {
        self.add_new_finished(plans, micros);

        self.finished.retain(|id| {
            let Some(current_plan) = plans.get_mut(id) else {
                return false;
            };

            let Some(location) = locations.get(id) else {
                return false;
            };

            let Some(piste) = pistes.get(location) else {
                return false;
            };

            let Some(ability) = abilities.get(id) else {
                return false;
            };

            free(id, current_plan, reservations);

            let from = last_state(current_plan);
            *current_plan = match (
                get_costs(id, locations, targets, distance_costs),
                get_skiing_costs(id, locations, targets, skiing_costs, ability),
            ) {
                (Some(distance_costs), Some(skiing_costs)) => new_plan(
                    terrain,
                    micros,
                    from,
                    piste,
                    reservations,
                    distance_costs,
                    skiing_costs,
                    ability,
                ),
                _ => brake(*from),
            };
            reserve(id, current_plan, reservations);

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

fn get_skiing_costs<'a>(
    id: &usize,
    locations: &HashMap<usize, usize>,
    targets: &HashMap<usize, usize>,
    costs: &'a HashMap<usize, HashMap<Ability, Costs>>,
    ability: &Ability,
) -> Option<&'a HashMap<State, u64>> {
    let location = locations.get(id)?;
    let target = targets.get(id)?;
    let costs = costs.get(location)?.get(ability)?;
    costs.costs(target)
}

fn new_plan(
    terrain: &Grid<f32>,
    micros: &u128,
    from: &State,
    piste: &Piste,
    reservations: &Grid<HashMap<usize, Reservation>>,
    distance_costs: &HashMap<State, u64>,
    skiing_costs: &HashMap<State, u64>,
    ability: &Ability,
) -> Plan {
    match find_path(
        terrain,
        micros,
        from,
        piste,
        reservations,
        distance_costs,
        skiing_costs,
        ability,
    ) {
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
    distance_costs: &HashMap<State, u64>,
    skiing_costs: &HashMap<State, u64>,
    ability: &Ability,
) -> Option<Vec<Edge<State>>> {
    let network = SkiingNetwork {
        terrain,
        is_accessible_fn: &|position| {
            !reservations[position]
                .values()
                .any(|reservation| reservation.is_valid_at(micros))
        },
        is_skiable_edge_fn: &|a, b| match (distance_costs.get(a), distance_costs.get(b)) {
            (Some(to), Some(from)) => to < from,
            _ => false,
        },
        ability,
    };

    let mut rng = rand::thread_rng();

    let steps = rng.gen_range(1..=MAX_STEPS);

    network
        .find_best_within_steps(
            HashSet::from([*from]),
            &mut |_, state| {
                if state.position == from.position {
                    return None;
                }

                skiing_costs.get(state).map(|cost| score(&mut rng, cost))
            },
            &mut |state| piste.grid.in_bounds(state.position) && piste.grid[state.position],
            steps,
        )
        .map(|result| result.path)
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
    Plan::Stationary(State {
        velocity: 0,
        ..state
    })
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
