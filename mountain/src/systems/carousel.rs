use std::collections::HashMap;

use commons::grid::Grid;

use crate::model::carousel::{Car, Carousel};
use crate::model::direction::Direction;
use crate::model::lift::Lift;
use crate::model::skiing::{Mode, Plan, State};
use crate::utils::carousel::{revolve, RevolveAction, RevolveEvent, RevolveResult};

pub struct System {
    last_micros: Option<u128>,
}

pub struct Parameters<'a> {
    pub micros: &'a u128,
    pub lifts: &'a HashMap<usize, Lift>,
    pub carousels: &'a HashMap<usize, Carousel>,
    pub reserved: &'a mut Grid<bool>,
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

            // check for blocked pickup

            if reserved[lift.drop_off.position] {
                if let Some(first_drop_off) = revolve_result
                    .events
                    .iter()
                    .find(|event| event.action == RevolveAction::DropOff)
                {
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
                        plans.retain(|plan_id, plan| {
                            if !matches!(targets.get(plan_id), Some(&target) if target == carousel.lift_id) {
                                return true;
                            }
                            if !matches!(plan, Plan::Stationary(State { position, ..}) if *position == lift.pick_up.position) {
                                return true;
                            }
                            println!("{} was picked up by {}", plan_id, car_id);
                            targets.remove(plan_id);
                            locations.insert(*plan_id, *car_id);
                            reserved[lift.pick_up.position] = false;
                            false
                        });
                    }
                    RevolveAction::DropOff => {
                        locations.retain(|location_id, location| {
                            if *location != *car_id {
                                return true;
                            }
                            println!("{} was dropped off from {}", location_id, car_id);
                            plans.insert(
                                *location_id,
                                Plan::Stationary(State {
                                    position: lift.drop_off.position,
                                    mode: Mode::Skiing { velocity: 0 },
                                    travel_direction: Direction::NorthEast,
                                }),
                            );
                            reserved[lift.drop_off.position] = true;
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
