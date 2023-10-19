use std::collections::HashMap;

use commons::grid::Grid;
use nalgebra::Point3;

use crate::model::car::Car;
use crate::model::carousel::Carousel;
use crate::model::direction::Direction;
use crate::model::lift::Lift;
use crate::model::skiing::{Mode, Plan, State};

pub struct System {
    last_micros: Option<u128>,
}

pub struct Parameters<'a> {
    pub micros: &'a u128,
    pub terrain: &'a Grid<f32>,
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
            terrain,
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

        for (car_id, car) in cars {
            let Some(lift) = lifts.get(&car.lift) else {
                return;
            };
            let Some(carousel) = carousels.get(&car.lift) else {
                return;
            };

            let max_metres_travelled = 

            if reserved[lift.to] {
                return;
            }

            let midpoint = nalgebra::distance(
                &Point3::new(lift.from.x as f32, lift.from.y as f32, terrain[lift.from]),
                &Point3::new(lift.to.x as f32, lift.to.y as f32, terrain[lift.to]),
            );
            let end = midpoint * 2.0;

            let new_position = car.position + carousel.velocity * elasped_seconds;

            // drop off
            if car.position <= midpoint && new_position > midpoint {
                car.position = new_position;
                locations.retain(|id, location| {
                    if location != car_id {
                        return true;
                    }
                    println!("{} was dropped off from {}", id, car_id);
                    plans.insert(
                        *id,
                        Plan::Stationary(State {
                            position: lift.to,
                            mode: Mode::Skiing { velocity: 0 },
                            travel_direction: Direction::NorthEast,
                        }),
                    );
                    reserved[lift.to] = true;
                    false
                });
            }
            // pick up
            else if new_position >= end {
                car.position = new_position - end;
                plans.retain(|id, plan| {
                    if !matches!(targets.get(id), Some(&target) if target == car.lift) {
                        return true;
                    }
                    if !matches!(plan, Plan::Stationary(State { position, ..}) if *position == lift.from) {
                        return true;
                    }
                    println!("{} was picked up by {}", id, car_id);
                    targets.remove(id);
                    locations.insert(*id, *car_id);
                    reserved[lift.from] = false;
                    false
                });
            } else {
                car.position = new_position;
            }
        }
    }
}
