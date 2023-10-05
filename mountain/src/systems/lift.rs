use std::collections::HashMap;

use commons::grid::Grid;

use crate::model::lift::Lift;
use crate::model::reservation::Reservation;
use crate::model::skiing::{Mode, Plan, State};

pub fn run(
    micros: &u128,
    lifts: &HashMap<usize, Lift>,
    locations: &mut HashMap<usize, usize>,
    reserved: &mut Grid<Vec<Reservation>>,
    plans: &mut HashMap<usize, Plan>,
) {
    locations.retain(|id, location| {
        let Some(lift) = lifts.get(location) else {
            return true;
        };
        let Some(Plan::Stationary(state)) = plans.get(id) else {
            return true;
        };

        if reserved[lift.to]
            .iter()
            .any(|reservation| match reservation {
                Reservation::Permanent { from, .. } => micros >= from,
                Reservation::Temporary { from, to, .. } => micros >= from && micros < to,
            })
        {
            return true;
        }

        reserved[state.position].retain(|reservation| reservation.id() != id);
        plans.insert(
            *id,
            Plan::Stationary(State {
                position: lift.to,
                mode: Mode::Skiing { velocity: 1 },
                travel_direction: state.travel_direction,
                micros: *micros,
            }),
        );
        reserved[lift.to].push(Reservation::Permanent {
            id: *id,
            from: *micros,
        });
        false
    });
}
