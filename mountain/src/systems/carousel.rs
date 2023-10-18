use std::collections::HashMap;

use commons::geometry::xyz;
use commons::grid::Grid;

use crate::model::car::Car;
use crate::model::carousel::Carousel;
use crate::model::direction::Direction;
use crate::model::lift::Lift;
use crate::model::skiing::{Mode, Plan, State};

pub struct System {
    last_micros: Option<u128>,
}

impl System {
    pub fn new() -> System {
        System { last_micros: None }
    }

    pub fn run(
        &mut self,
        micros: &u128,
        terrain: &Grid<f32>,
        lifts: &HashMap<usize, Lift>,
        carousels: &HashMap<usize, Carousel>,
        reserved: &mut Grid<bool>,
        plans: &mut HashMap<usize, Plan>,
        locations: &mut HashMap<usize, usize>,
        targets: &mut HashMap<usize, usize>,
        cars: &mut HashMap<usize, Car>,
    ) {
        let Some(last_micros) = self.last_micros else {
            self.last_micros = Some(*micros);
            return;
        };

        let elasped_seconds = (micros - last_micros) as f32 / 1_000_000.0;

        self.last_micros = Some(*micros);

        for (car_id, car) in cars {
            self.update_car(
                car_id,
                &elasped_seconds,
                terrain,
                lifts,
                carousels,
                reserved,
                plans,
                locations,
                targets,
                car,
            )
        }
    }

    fn update_car(
        &self,
        car_id: &usize,
        elasped_seconds: &f32,
        terrain: &Grid<f32>,
        lifts: &HashMap<usize, Lift>,
        carousels: &HashMap<usize, Carousel>,
        reserved: &mut Grid<bool>,
        plans: &mut HashMap<usize, Plan>,
        locations: &mut HashMap<usize, usize>,
        targets: &mut HashMap<usize, usize>,
        car: &mut Car,
    ) {
        let Some(lift) = lifts.get(&car.lift) else {
            return;
        };
        let Some(carousel) = carousels.get(&car.lift) else {
            return;
        };

        if reserved[lift.to] {
            return;
        }

        let from = xyz(lift.from.x as f32, lift.from.y as f32, terrain[lift.from]);
        let to = xyz(lift.to.x as f32, lift.to.y as f32, terrain[lift.to]);

        let midpoint =
            ((from.x - to.x).powf(2.0) + (from.y - to.y).powf(2.0) + (from.z - to.z)).sqrt();
        let end = midpoint * 2.0;
        let new_position = car.position + carousel.velocity * elasped_seconds;

        // TODO what if both drop off and pick up happen?

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
                if matches!(targets.get(id), Some(&target) if target == car.lift) && matches!(plan, Plan::Stationary(State { position, ..}) if *position == lift.from) {
                    println!("{} is riding in {}", id, car_id);
                    locations.insert(*id, *car_id);
                    targets.remove(id);
                    reserved[lift.from] = false;
                    return false;
                }
                true
            });
        } else {
            car.position = new_position;
        }
    }
}
