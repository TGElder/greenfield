use std::collections::HashMap;

use commons::grid::Grid;

use crate::model::lift::Lift;
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

        if reserved[lift.drop_off.position] {
            return true;
        }

        reserved[state.position] = false;
        plans.insert(
            *id,
            Plan::Stationary(State {
                position: lift.drop_off.position,
                mode: Mode::Skiing { velocity: 1 },
                travel_direction: state.travel_direction,
            }),
        );
        reserved[lift.drop_off.position] = true;
        false
    });
}
