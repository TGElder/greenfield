use commons::geometry::XYZ;
use nalgebra::Point3;
use serde::{Deserialize, Serialize};

use crate::model::skiing::State;

#[derive(Serialize, Deserialize)]
pub struct Lift {
    pub segments: Vec<Segment>,
    pub pick_up: Portal,
    pub drop_off: Portal,
    pub carousel_id: usize,
    pub buildings_id: usize,
}

#[derive(Serialize, Deserialize)]
pub struct Segment {
    pub from: XYZ<f32>,
    pub to: XYZ<f32>,
    length_meters: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Portal {
    pub id: usize,
    pub segment: usize,
    pub state: State,
}

impl Lift {
    pub fn ride_length_meters(&self) -> f32 {
        let mut segment = self.pick_up.segment;
        let mut out = 0.0;
        while segment != self.drop_off.segment {
            out += self.segments[segment].length_meters;
            segment = (segment + 1) % self.segments.len();
        }
        out
    }
}

impl Segment {
    pub fn new(from: XYZ<f32>, to: XYZ<f32>) -> Segment {
        let length_meters = nalgebra::distance(
            &Point3::new(from.x, from.y, from.z),
            &Point3::new(to.x, to.y, to.z),
        );
        Segment {
            from,
            to,
            length_meters,
        }
    }

    pub fn length_meters(&self) -> &f32 {
        &self.length_meters
    }
}

#[cfg(test)]
mod tests {
    use commons::geometry::{xy, xyz};

    use crate::model::direction::Direction;

    use super::*;

    fn segments(points: &[XYZ<f32>]) -> Vec<Segment> {
        points
            .windows(2)
            .map(|pair| Segment::new(pair[0], pair[1]))
            .collect()
    }

    #[test]
    fn test_ride_length_meters_pick_up_before_drop_off() {
        // given
        let lift = Lift {
            segments: segments(&[
                xyz(0.0, 0.0, 0.0),
                xyz(2.0, 0.0, 0.0),
                xyz(2.0, 1.0, 0.0),
                xyz(0.0, 1.0, 0.0),
                xyz(0.0, 0.0, 0.0),
            ]),
            pick_up: Portal {
                id: 0,
                segment: 0,
                state: State {
                    position: xy(0, 0),
                    travel_direction: Direction::North,
                    velocity: 0,
                },
            },
            drop_off: Portal {
                id: 0,
                segment: 2,
                state: State {
                    position: xy(2, 1),
                    travel_direction: Direction::North,
                    velocity: 0,
                },
            },
            carousel_id: 0,
            buildings_id: 0,
        };

        // when
        let result = lift.ride_length_meters();

        // then
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_ride_length_meters_pick_up_after_drop_off() {
        // given
        let lift = Lift {
            segments: segments(&[
                xyz(2.0, 1.0, 0.0),
                xyz(0.0, 1.0, 0.0),
                xyz(0.0, 0.0, 0.0),
                xyz(2.0, 0.0, 0.0),
                xyz(2.0, 1.0, 0.0),
            ]),
            pick_up: Portal {
                id: 0,
                segment: 2,
                state: State {
                    position: xy(0, 0),
                    travel_direction: Direction::North,
                    velocity: 0,
                },
            },
            drop_off: Portal {
                id: 0,
                segment: 0,
                state: State {
                    position: xy(0, 0),
                    travel_direction: Direction::North,
                    velocity: 0,
                },
            },
            carousel_id: 0,
            buildings_id: 0,
        };

        // when
        let result = lift.ride_length_meters();

        // then
        assert_eq!(result, 3.0);
    }
}
