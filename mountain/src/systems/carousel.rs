use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

use commons::grid::Grid;
use commons::map::ContainsKeyValue;

use crate::model::carousel::{Car, Carousel};
use crate::model::lift::Lift;
use crate::model::open;
use crate::model::reservation::{Reservation, ReservationPeriod};
use crate::model::skiing::Plan;
use crate::utils::carousel::{revolve, RevolveAction, RevolveEvent, RevolveResult};

pub struct System {
    last_micros: Option<u128>,
}

pub struct Parameters<'a> {
    pub micros: &'a u128,
    pub lifts: &'a HashMap<usize, Lift>,
    pub open: &'a HashMap<usize, open::Status>,
    pub carousels: &'a HashMap<usize, Carousel>,
    pub reservations: &'a mut Grid<HashMap<usize, Reservation>>,
    pub plans: &'a mut HashMap<usize, Plan>,
    pub locations: &'a mut HashMap<usize, usize>,
    pub targets: &'a mut HashMap<usize, usize>,
    pub global_targets: &'a mut HashMap<usize, usize>,
    pub cars: &'a mut HashMap<usize, Car>,
}

impl System {
    pub fn new() -> System {
        System { last_micros: None }
    }

    pub fn run(
        &mut self,
        Parameters {
            micros,
            lifts,
            open,
            carousels,
            reservations,
            plans,
            locations,
            targets,
            global_targets,
            cars,
        }: Parameters<'_>,
    ) {
        let Some(last_micros) = self.last_micros else {
            self.last_micros = Some(*micros);
            return;
        };

        let elasped_seconds = (micros - last_micros) as f32 / 1_000_000.0;
        self.last_micros = Some(*micros);

        for carousel in carousels.values() {
            let Some(lift) = lifts.get(&carousel.lift_id) else {
                return;
            };

            // get cars

            let mut car_ids = Vec::with_capacity(carousel.car_ids.len());
            let mut current_cars = Vec::with_capacity(carousel.car_ids.len());

            for car_id in carousel.car_ids.iter() {
                if let Some(car) = cars.get(car_id) {
                    car_ids.push(car_id);
                    current_cars.push(car);
                }
            }

            // revolve

            let meters = elasped_seconds * carousel.velocity;

            let mut revolve_result = revolve(lift, &current_cars, meters);

            // check for blocked drop off

            let occupied_locations = locations.values().collect::<HashSet<_>>();

            if reservations[lift.drop_off.state.position]
                .values()
                .any(|reservation| reservation.includes(micros))
            {
                if let Some(first_drop_off) = revolve_result.events.iter().find(|event| {
                    let car_id = car_ids[event.car_index];
                    event.action == RevolveAction::DropOff && occupied_locations.contains(car_id)
                }) {
                    revolve_result = revolve(lift, &current_cars, first_drop_off.revolve_meters);
                }
            }

            // process events

            let RevolveResult {
                cars: new_cars,
                events,
            } = revolve_result;

            for RevolveEvent {
                car_index, action, ..
            } in events
            {
                let car_id = car_ids[car_index];
                match action {
                    RevolveAction::PickUp => {
                        if !open.contains_key_value(lift.pick_up.id, open::Status::Open) {
                            continue;
                        }
                        plans.retain(|skier_id, plan| {
                            if !matches!(targets.get(skier_id), Some(&target) if target == lift.pick_up.id) {
                                return true;
                            }
                            if !matches!(plan, Plan::Stationary(state) if *state == lift.pick_up.state) {
                                return true;
                            }
                            targets.remove(skier_id);
                            if let Entry::Occupied(entry) = global_targets.entry(*skier_id) {
                                if *entry.get() == lift.pick_up.id {
                                    entry.remove();
                                }
                            }
                            locations.insert(*skier_id, *car_id);
                            reservations[lift.pick_up.state.position].remove(skier_id);
                            false
                        });
                    }
                    RevolveAction::DropOff => {
                        locations.retain(|skier_id, location| {
                            if *location != *car_id {
                                return true;
                            }
                            if let Entry::Occupied(entry) = global_targets.entry(*skier_id) {
                                if *entry.get() == lift.drop_off.id {
                                    entry.remove();
                                }
                            }
                            plans.insert(*skier_id, Plan::Stationary(lift.drop_off.state));
                            reservations[lift.drop_off.state.position].insert(
                                *skier_id,
                                Reservation::Mobile(ReservationPeriod::Permanent),
                            );
                            false
                        });
                    }
                }
            }

            // update cars

            car_ids.into_iter().zip(new_cars).for_each(|(id, car)| {
                cars.insert(*id, car);
            });
        }
    }
}
