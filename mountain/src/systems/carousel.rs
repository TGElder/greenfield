use std::collections::HashMap;

use commons::grid::Grid;
use commons::unsafe_ordering::unsafe_ordering;

use crate::model::car::Car;
use crate::model::carousel::Carousel;
use crate::model::direction::Direction;
use crate::model::lift::{self, Lift};
use crate::model::skiing::{Mode, Plan, State};

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

        for (lift_id, carousel) in carousels {
            let Some(lift) = lifts.get(lift_id) else {
                return;
            };

            let mut max_travel_metres = elasped_seconds * carousel.velocity;

            struct Event {
                car_id: usize,
                metres: f32,
                action: lift::Action,
            }

            let mut events = vec![];

            for car_id in carousel.cars.iter() {
                let Some(car) = cars.get(car_id) else {
                    return;
                };
                let mut segment = car.segment;
                let mut residual = max_travel_metres;
                let mut position_metres = car.position_metres;

                while residual >= lift.nodes[segment].distance_metres - position_metres {
                    residual -= lift.nodes[segment].distance_metres - position_metres;
                    position_metres = 0.0;
                    segment = (segment + 1) % lift.nodes.len();
                    if let Some(action) = &lift.nodes[segment].from_action {
                        events.push(Event {
                            car_id: *car_id,
                            metres: residual,
                            action: *action,
                        });
                    }
                }
            }

            events.sort_by(|b, a| unsafe_ordering(&b.metres, &a.metres));

            for event in events {
                if event.metres > max_travel_metres {
                    break;
                }
                match event.action {
                    lift::Action::PickUp(pick_up_position) => {
                        plans.retain(|plan_id, plan| {
                            if !matches!(targets.get(plan_id), Some(&target) if target == *lift_id) {
                                return true;
                            }
                            if !matches!(plan, Plan::Stationary(State { position, ..}) if *position == pick_up_position) {
                                return true;
                            }
                            println!("{} was picked up by {}", plan_id, event.car_id);
                            targets.remove(plan_id);
                            locations.insert(*plan_id, event.car_id);
                            reserved[pick_up_position] = false;
                            false
                        });
                    }
                    lift::Action::DropOff(drop_off_position) => {
                        if reserved[drop_off_position] {
                            max_travel_metres = event.metres;
                            continue;
                        }
                        locations.retain(|location_id, location| {
                            if *location != event.car_id {
                                return true;
                            }
                            println!("{} was dropped off from {}", location_id, event.car_id);
                            plans.insert(
                                *location_id,
                                Plan::Stationary(State {
                                    position: drop_off_position,
                                    mode: Mode::Skiing { velocity: 0 },
                                    travel_direction: Direction::NorthEast,
                                }),
                            );
                            reserved[drop_off_position] = true;
                            false
                        });
                    }
                }
            }

            for car_id in carousel.cars.iter() {
                let Some(car) = cars.get_mut(car_id) else {
                    return;
                };
                let mut residual = max_travel_metres;

                while residual >= lift.nodes[car.segment].distance_metres - car.position_metres {
                    residual -= lift.nodes[car.segment].distance_metres - car.position_metres;
                    car.position_metres = 0.0;
                    car.segment = (car.segment + 1) % lift.nodes.len();
                }

                car.position_metres = residual;
            }
        }
    }
}
