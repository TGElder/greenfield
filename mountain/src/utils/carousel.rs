use commons::geometry::XY;
use commons::unsafe_ordering::unsafe_ordering;

use crate::model::car::Car;
use crate::model::lift::{Lift, Segment};

pub fn create_cars(
    lift_id: &usize,
    segments: &[Segment],
    min_lift_interval_meters: &f32,
) -> Vec<Car> {
    let mut meters_from_start_of_segment = 0.0;
    let mut result = vec![];

    let total_distance = segments
        .iter()
        .map(|segment| segment.length_meters())
        .sum::<f32>();
    let car_count = (total_distance / min_lift_interval_meters).floor();
    let lift_interval_meters = total_distance / car_count;

    for (segment_id, segment) in segments.iter().enumerate() {
        while meters_from_start_of_segment < *segment.length_meters() {
            result.push(Car {
                lift_id: *lift_id,
                segment: segment_id,
                meters_from_start: meters_from_start_of_segment,
            });
            meters_from_start_of_segment += lift_interval_meters;
        }

        meters_from_start_of_segment -= segment.length_meters();
    }
    result
}

#[derive(Debug)]
pub struct RevolveResult {
    pub cars: Vec<Car>,
    pub events: Vec<RevolveEvent>,
}

#[derive(Debug)]
pub struct RevolveEvent {
    pub meters: f32,
    pub car_index: usize,
    pub action: RevolveAction,
    pub position: XY<u32>,
}

#[derive(Debug, PartialEq)]
pub enum RevolveAction {
    PickUp,
    DropOff,
}

pub fn revolve(lift: &Lift, cars: &[&Car], meters: f32) -> RevolveResult {
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
        .sort_by(|a, b| unsafe_ordering(&a.meters, &b.meters));

    result
}

pub struct RevolveCarResult {
    pub car: Car,
    pub events: Vec<RevolveEvent>,
}

fn revolve_car(lift: &Lift, car_index: &usize, car: &Car, meters: f32) -> RevolveCarResult {
    let mut segment = car.segment;
    let mut meters_from_start = car.meters_from_start;
    let mut residual = meters;

    let mut events = vec![];

    loop {
        if meters_from_start == 0.0 {
            if lift.drop_off.segment == segment {
                events.push(RevolveEvent {
                    meters: meters - residual,
                    car_index: *car_index,
                    action: RevolveAction::DropOff,
                    position: lift.drop_off.position,
                });
            }
            if lift.pick_up.segment == segment {
                events.push(RevolveEvent {
                    meters: meters - residual,
                    car_index: *car_index,
                    action: RevolveAction::PickUp,
                    position: lift.pick_up.position,
                });
            }
        }
        let meters_to_end = lift.segments[segment].length_meters() - meters_from_start;
        if residual <= meters_to_end {
            break;
        }
        residual -= meters_to_end;
        segment = (segment + 1) % lift.segments.len();
        meters_from_start = 0.0;
    }

    RevolveCarResult {
        car: Car {
            lift_id: car.lift_id,
            segment,
            meters_from_start: meters_from_start + residual,
        },
        events,
    }
}

#[cfg(test)]
mod tests {
    use commons::almost_eq::assert_almost_eq;
    use commons::geometry::{xy, xyz};

    use crate::model::lift;

    use super::*;

    #[test]
    fn test_create_cars() {
        // given
        let segments = vec![
            Segment::_new(xyz(0.0, 0.0, 0.0), xyz(0.5, 0.0, 0.0)),
            Segment::_new(xyz(0.0, 0.0, 0.0), xyz(0.5, 0.0, 0.0)),
        ];

        // when
        let result = create_cars(&1986, &segments, &0.19);
        dbg!(&result);

        // then
        let expected = vec![
            Car {
                lift_id: 1986,
                segment: 0,
                meters_from_start: 0.0,
            },
            Car {
                lift_id: 1986,
                segment: 0,
                meters_from_start: 0.2,
            },
            Car {
                lift_id: 1986,
                segment: 0,
                meters_from_start: 0.4,
            },
            Car {
                lift_id: 1986,
                segment: 1,
                meters_from_start: 0.1,
            },
            Car {
                lift_id: 1986,
                segment: 1,
                meters_from_start: 0.3,
            },
        ];

        for (i, actual) in result.iter().enumerate() {
            let expected = &expected[i];
            assert_eq!(actual.lift_id, expected.lift_id);
            assert_eq!(actual.segment, expected.segment);
            assert_almost_eq(actual.meters_from_start, expected.meters_from_start);
        }
    }

    #[test]
    fn test_revolve() {
        // given
        let lift = Lift {
            pick_up: lift::Portal {
                segment: 0,
                position: xy(0, 0),
            },
            drop_off: lift::Portal {
                segment: 1,
                position: xy(1, 0),
            },
            segments: vec![
                Segment::_new(xyz(0.0, 0.0, 0.0), xyz(1.0, 0.0, 0.0)),
                Segment::_new(xyz(0.0, 0.0, 0.0), xyz(1.0, 0.0, 0.0)),
            ],
        };
        let cars = vec![
            &Car {
                lift_id: 0,
                segment: 0,
                meters_from_start: 0.0,
            },
            &Car {
                lift_id: 0,
                segment: 0,
                meters_from_start: 0.67,
            },
            &Car {
                lift_id: 0,
                segment: 1,
                meters_from_start: 0.33,
            },
        ];

        // when
        let result = revolve(&lift, &cars, 1.0);
        dbg!(&result);

        // then
        let expected = RevolveResult {
            cars: vec![
                Car {
                    lift_id: 0,
                    segment: 0,
                    meters_from_start: 1.0,
                },
                Car {
                    lift_id: 0,
                    segment: 1,
                    meters_from_start: 0.67,
                },
                Car {
                    lift_id: 0,
                    segment: 0,
                    meters_from_start: 0.33,
                },
            ],
            events: vec![
                RevolveEvent {
                    meters: 0.0,
                    car_index: 0,
                    action: RevolveAction::PickUp,
                    position: xy(0, 0),
                },
                RevolveEvent {
                    meters: 0.33,
                    car_index: 1,
                    action: RevolveAction::DropOff,
                    position: xy(1, 0),
                },
                RevolveEvent {
                    meters: 0.67,
                    car_index: 2,
                    action: RevolveAction::PickUp,
                    position: xy(0, 0),
                },
            ],
        };

        for (i, actual) in result.cars.iter().enumerate() {
            let expected = &expected.cars[i];
            assert_eq!(actual.lift_id, expected.lift_id);
            assert_eq!(actual.segment, expected.segment);
            assert_almost_eq(actual.meters_from_start, expected.meters_from_start);
        }

        for (i, actual) in result.events.iter().enumerate() {
            let expected = &expected.events[i];
            assert_almost_eq(actual.meters, expected.meters);
            assert_eq!(actual.car_index, expected.car_index);
            assert_eq!(actual.action, expected.action);
            assert_eq!(actual.position, expected.position);
        }
    }

    #[test]
    fn test_revolve_action_only_generated_once() {
        // given
        let lift = Lift {
            pick_up: lift::Portal {
                segment: 0,
                position: xy(0, 0),
            },
            drop_off: lift::Portal {
                segment: 1,
                position: xy(1, 0),
            },
            segments: vec![
                Segment::_new(xyz(0.0, 0.0, 0.0), xyz(1.0, 0.0, 0.0)),
                Segment::_new(xyz(0.0, 0.0, 0.0), xyz(1.0, 0.0, 0.0)),
            ],
        };
        let cars = vec![&Car {
            lift_id: 0,
            segment: 0,
            meters_from_start: 0.5,
        }];

        // when
        let result = revolve(&lift, &cars, 0.5);
        dbg!(&result);

        // then
        assert_eq!(result.cars[0].segment, 0);
        assert_almost_eq(result.cars[0].meters_from_start, 1.0);
        assert!(result.events.is_empty());

        // given
        // cars initialized with output of last revolution
        let cars = result.cars.iter().collect::<Vec<_>>();

        // when
        let result = revolve(&lift, &cars, 0.5);
        dbg!(&result);

        // then
        assert_eq!(result.cars[0].segment, 1);
        assert_almost_eq(result.cars[0].meters_from_start, 0.5);
        assert_almost_eq(result.events[0].meters, 0.0);
        assert_eq!(result.events[0].action, RevolveAction::DropOff);
    }
}
