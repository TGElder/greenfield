use std::collections::HashMap;

use commons::grid::Grid;

use crate::model::direction::Direction;
use crate::model::door::Door;
use crate::model::reservation::{Reservation, ReservationPeriod};
use crate::model::skiing::{Plan, State};

pub struct Parameters<'a> {
    pub doors: &'a HashMap<usize, Door>,
    pub locations: &'a mut HashMap<usize, usize>,
    pub plans: &'a mut HashMap<usize, Plan>,
    pub reservations: &'a mut Grid<HashMap<usize, Reservation>>,
}

pub fn run(
    Parameters {
        doors,
        locations,
        plans,
        reservations,
    }: Parameters<'_>,
) {
    for door in doors.values() {
        let mut skiers_to_spawn = locations
            .iter()
            .filter(|&(_, location_id)| *location_id == door.building_id)
            .map(|(skier_id, _)| *skier_id)
            .collect::<Vec<_>>();

        if skiers_to_spawn.is_empty() {
            continue;
        }

        for position in &door.aperture {
            if !skiers_to_spawn.is_empty() && reservations[position].is_empty() {
                let skier_id = skiers_to_spawn.pop().unwrap();

                locations.remove(&skier_id);
                plans.insert(
                    skier_id,
                    Plan::Stationary(State {
                        position: *position,
                        velocity: 0,
                        travel_direction: Direction::North,
                    }),
                );
                reservations[position]
                    .insert(skier_id, Reservation::Mobile(ReservationPeriod::Permanent));
            }
        }
    }
}
