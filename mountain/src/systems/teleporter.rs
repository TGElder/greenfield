use std::collections::HashMap;

use commons::grid::Grid;

use crate::model::lift::{self, Lift};
use crate::model::skiing::{Mode, Plan, State};

pub fn run(
    lifts: &HashMap<usize, Lift>,
    locations: &mut HashMap<usize, usize>,
    reserved: &mut Grid<bool>,
    plans: &mut HashMap<usize, Plan>,
) {
    locations.retain(|id, location| {
        let Some(lift) = lifts.get(location) else {
            return true;
        };
        let Some(Plan::Stationary(state)) = plans.get(id) else {
            return true;
        };

        for node in lift.nodes.iter() {
            if let Some(lift::Action::DropOff(to)) = node.from_action {
                if reserved[to] {
                    return true;
                }

                reserved[state.position] = false;
                plans.insert(
                    *id,
                    Plan::Stationary(State {
                        position: to,
                        mode: Mode::Skiing { velocity: 1 },
                        travel_direction: state.travel_direction,
                    }),
                );
                reserved[to] = true;
                return false;
            }
        }

        false
    });
}
