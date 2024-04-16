use commons::unsafe_ordering::unsafe_ordering;

use crate::model::carousel::Car;
use crate::model::lift::{Lift, Segment};

pub fn create_cars(
    carousel_id: usize,
    segments: &[Segment],
    min_lift_interval_meters: &f32,
) -> Vec<Car> {
    let lift_interval_meters = lift_interval_meters(min_lift_interval_meters, segments);

    let mut result = vec![];
    let mut distance_from_segment_start_meters = 0.0;

    for (segment_id, segment) in segments.iter().enumerate() {
        while distance_from_segment_start_meters < *segment.length_meters() {
            result.push(Car {
                carousel_id,
                segment: segment_id,
                distance_from_start_meters: distance_from_segment_start_meters,
            });
            distance_from_segment_start_meters += lift_interval_meters;
        }

        distance_from_segment_start_meters -= segment.length_meters();
    }

    result
}

fn lift_interval_meters(min_lift_interval_meters: &f32, segments: &[Segment]) -> f32 {
    let total_distance = segments
        .iter()
        .map(|segment| segment.length_meters())
        .sum::<f32>();
    let car_count = (total_distance / min_lift_interval_meters).floor();
    total_distance / car_count
}

#[derive(Debug)]
pub struct RevolveResult {
    pub cars: Vec<Car>,
    pub events: Vec<RevolveEvent>,
}

#[derive(Debug)]
pub struct RevolveEvent {
    pub revolve_meters: f32,
    pub car_index: usize,
    pub action: RevolveAction,
}

#[derive(Debug, PartialEq)]
pub enum RevolveAction {
    PickUp,
    DropOff,
}

pub fn revolve(lift: &Lift, cars: &[&Car], meters: f32) -> RevolveResult {
    if meters < 0.0 {
        panic!("Negative revolutions are not supported");
    }
    let mut result = cars
        .iter()
        .enumerate()
        .map(|(car_index, car)| revolve_car(lift, &car_index, car, meters))
        .fold(
            RevolveResult {
                cars: vec![],
                events: vec![],
            },
            |mut result, mut car_result| {
                result.cars.push(car_result.car);
                result.events.append(&mut car_result.events);
                result
            },
        );

    result
        .events
        .sort_by(|a, b| unsafe_ordering(&a.revolve_meters, &b.revolve_meters));

    result
}

pub struct RevolveCarResult {
    pub car: Car,
    pub events: Vec<RevolveEvent>,
}

fn revolve_car(lift: &Lift, car_index: &usize, car: &Car, revolve_meters: f32) -> RevolveCarResult {
    let mut segment = car.segment;
    let mut residual_meters = revolve_meters;
    let mut distance_from_segment_start_meters = car.distance_from_start_meters;

    let mut events = vec![];

    loop {
        if distance_from_segment_start_meters == 0.0 {
            if segment == lift.drop_off.segment {
                events.push(RevolveEvent {
                    revolve_meters: revolve_meters - residual_meters,
                    car_index: *car_index,
                    action: RevolveAction::DropOff,
                });
            }
            if segment == lift.pick_up.segment {
                events.push(RevolveEvent {
                    revolve_meters: revolve_meters - residual_meters,
                    car_index: *car_index,
                    action: RevolveAction::PickUp,
                });
            }
        }
        let distance_to_segment_end_meters =
            lift.segments[segment].length_meters() - distance_from_segment_start_meters;
        if residual_meters <= distance_to_segment_end_meters {
            break;
        }
        segment = (segment + 1) % lift.segments.len();
        residual_meters -= distance_to_segment_end_meters;
        distance_from_segment_start_meters = 0.0;
    }

    RevolveCarResult {
        car: Car {
            carousel_id: car.carousel_id,
            segment,
            distance_from_start_meters: distance_from_segment_start_meters + residual_meters,
        },
        events,
    }
}

#[cfg(test)]
mod tests {
    use commons::almost_eq::assert_almost_eq;
    use commons::geometry::{xy, xyz};

    use crate::model::direction::Direction;
    use crate::model::lift;
    use crate::model::skiing::State;

    use super::*;

    fn compare_results(actual: &RevolveResult, expected: &RevolveResult) {
        compare_cars(&actual.cars, &expected.cars);
        compare_events(&actual.events, &expected.events);
    }

    fn compare_cars(actual: &[Car], expected: &[Car]) {
        dbg!(actual);
        for (i, actual) in actual.iter().enumerate() {
            let expected = &expected[i];
            assert_eq!(actual.segment, expected.segment);
            assert_almost_eq(
                actual.distance_from_start_meters,
                expected.distance_from_start_meters,
            );
        }
    }

    fn compare_events(actual: &[RevolveEvent], expected: &[RevolveEvent]) {
        dbg!(actual);
        for (i, actual) in actual.iter().enumerate() {
            let expected = &expected[i];
            assert_almost_eq(actual.revolve_meters, expected.revolve_meters);
            assert_eq!(actual.car_index, expected.car_index);
            assert_eq!(actual.action, expected.action);
        }
    }

    #[test]
    fn test_create_cars() {
        // given
        let segments =
            Segment::segments(&[xyz(0.0, 0.0, 0.0), xyz(0.5, 0.0, 0.0), xyz(0.0, 0.0, 0.0)]);

        // when
        let result = create_cars(7, &segments, &0.19);

        // then
        let expected = vec![
            Car {
                carousel_id: 7,
                segment: 0,
                distance_from_start_meters: 0.0,
            },
            Car {
                carousel_id: 7,
                segment: 0,
                distance_from_start_meters: 0.2,
            },
            Car {
                carousel_id: 7,
                segment: 0,
                distance_from_start_meters: 0.4,
            },
            Car {
                carousel_id: 7,
                segment: 1,
                distance_from_start_meters: 0.1,
            },
            Car {
                carousel_id: 7,
                segment: 1,
                distance_from_start_meters: 0.3,
            },
        ];

        compare_cars(&result, &expected);
    }

    #[test]
    fn test_revolve() {
        // given
        let lift = Lift {
            pick_up: lift::Portal {
                segment: 0,
                state: State {
                    position: xy(0, 0),
                    travel_direction: Direction::North,
                    velocity: 0,
                },
            },
            drop_off: lift::Portal {
                segment: 1,
                state: State {
                    position: xy(1, 0),
                    travel_direction: Direction::North,
                    velocity: 0,
                },
            },
            segments: Segment::segments(&[
                xyz(0.0, 0.0, 0.0),
                xyz(1.0, 0.0, 0.0),
                xyz(0.0, 0.0, 0.0),
            ]),
            carousel_id: 0,
        };
        let cars = vec![
            &Car {
                carousel_id: 7,
                segment: 0,
                distance_from_start_meters: 0.0,
            },
            &Car {
                carousel_id: 7,
                segment: 0,
                distance_from_start_meters: 0.67,
            },
            &Car {
                carousel_id: 7,
                segment: 1,
                distance_from_start_meters: 0.33,
            },
        ];

        // when
        let result = revolve(&lift, &cars, 1.0);

        // then
        let expected = RevolveResult {
            cars: vec![
                Car {
                    carousel_id: 7,
                    segment: 0,
                    distance_from_start_meters: 1.0,
                },
                Car {
                    carousel_id: 7,
                    segment: 1,
                    distance_from_start_meters: 0.67,
                },
                Car {
                    carousel_id: 7,
                    segment: 0,
                    distance_from_start_meters: 0.33,
                },
            ],
            events: vec![
                RevolveEvent {
                    revolve_meters: 0.0,
                    car_index: 0,
                    action: RevolveAction::PickUp,
                },
                RevolveEvent {
                    revolve_meters: 0.33,
                    car_index: 1,
                    action: RevolveAction::DropOff,
                },
                RevolveEvent {
                    revolve_meters: 0.67,
                    car_index: 2,
                    action: RevolveAction::PickUp,
                },
            ],
        };

        compare_results(&result, &expected);
    }

    #[test]
    fn test_revolve_action_only_generated_once() {
        // given
        let lift = Lift {
            pick_up: lift::Portal {
                segment: 0,
                state: State {
                    position: xy(0, 0),
                    travel_direction: Direction::North,
                    velocity: 0,
                },
            },
            drop_off: lift::Portal {
                segment: 1,
                state: State {
                    position: xy(1, 0),
                    travel_direction: Direction::North,
                    velocity: 0,
                },
            },
            segments: Segment::segments(&[
                xyz(0.0, 0.0, 0.0),
                xyz(1.0, 0.0, 0.0),
                xyz(0.0, 0.0, 0.0),
            ]),
            carousel_id: 0,
        };
        let cars = vec![&Car {
            carousel_id: 7,
            segment: 0,
            distance_from_start_meters: 0.5,
        }];

        // when
        let result = revolve(&lift, &cars, 0.5);

        // then
        let expected = RevolveResult {
            cars: vec![Car {
                carousel_id: 7,
                segment: 0,
                distance_from_start_meters: 1.0,
            }],
            events: vec![],
        };
        compare_results(&result, &expected);

        // given
        // cars initialized with output of last revolution
        let cars = result.cars.iter().collect::<Vec<_>>();

        // when
        let result = revolve(&lift, &cars, 0.5);

        // then
        let expected = RevolveResult {
            cars: vec![Car {
                carousel_id: 7,
                segment: 1,
                distance_from_start_meters: 0.5,
            }],
            events: vec![RevolveEvent {
                revolve_meters: 0.0,
                car_index: 0,
                action: RevolveAction::DropOff,
            }],
        };
        compare_results(&result, &expected);
    }
}
