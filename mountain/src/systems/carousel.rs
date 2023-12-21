use std::collections::{HashMap, HashSet};

use commons::grid::Grid;

use crate::model::carousel::{Car, Carousel};
use crate::model::lift::Lift;
use crate::model::reservation::Reservation;
use crate::model::skiing::{Mode, Plan, State};
use crate::network::velocity_encoding::{encode_velocity, VELOCITY_LEVELS};
use crate::utils::carousel::{revolve, RevolveAction, RevolveEvent, RevolveResult};

pub struct System {
    last_micros: Option<u128>,
}

pub struct Parameters<'a> {
    pub micros: &'a u128,
    pub lifts: &'a HashMap<usize, Lift>,
    pub open: &'a HashSet<usize>,
    pub carousels: &'a HashMap<usize, Carousel>,
    pub reserved: &'a mut Grid<HashMap<usize, Reservation>>,
    pub plans: &'a mut HashMap<usize, Plan>,
    pub locations: &'a mut HashMap<usize, usize>,
    pub targets: &'a mut HashMap<usize, usize>,
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
            reserved,
            plans,
            locations,
            targets,
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

            if reserved[lift.drop_off.position]
                .values()
                .any(|reservation| reservation.is_reserved(micros))
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
                        if !open.contains(&carousel.lift_id) {
                            continue;
                        }
                        plans.retain(|plan_id, plan| {
                            if !matches!(targets.get(plan_id), Some(&target) if target == carousel.lift_id) {
                                return true;
                            }
                            if !matches!(plan, Plan::Stationary(State { position, ..}) if *position == lift.pick_up.position) {
                                return true;
                            }
                            targets.remove(plan_id);
                            locations.insert(*plan_id, *car_id);
                            reserved[lift.pick_up.position].remove(plan_id);
                            false
                        });
                    }
                    RevolveAction::DropOff => {
                        locations.retain(|location_id, location| {
                            if *location != *car_id {
                                return true;
                            }
                            plans.insert(
                                *location_id,
                                Plan::Stationary(State {
                                    position: lift.drop_off.position,
                                    mode: Mode::Skiing {
                                        velocity: encode_velocity(&carousel.velocity)
                                            .unwrap_or(VELOCITY_LEVELS - 1),
                                    },
                                    travel_direction: lift.drop_off.direction,
                                }),
                            );
                            reserved[lift.drop_off.position]
                                .insert(*location_id, Reservation::Eternal);
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
