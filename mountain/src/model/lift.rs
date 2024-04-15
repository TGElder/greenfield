use commons::geometry::{XY, XYZ};
use nalgebra::Point3;
use serde::{Deserialize, Serialize};

use crate::model::direction::Direction;

#[derive(Serialize, Deserialize)]
pub struct Lift {
    pub segments: Vec<Segment>,
    pub pick_up: Portal,
    pub drop_off: Portal,
    pub carousel_id: usize,
}

impl Lift {
    fn _ride_length_meters(&self) -> f32 {
        let mut segment = self.pick_up.segment;
        let mut out = 0.0;
        while segment != self.drop_off.segment {
            out += self.segments[segment].length_meters;
            segment = (segment + 1) % self.segments.len();
        }
        out
    }
}

#[derive(Serialize, Deserialize)]
pub struct Segment {
    pub from: XYZ<f32>,
    pub to: XYZ<f32>,
    length_meters: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Portal {
    pub segment: usize,
    pub position: XY<u32>,
    pub direction: Direction,
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

    pub fn segments(points: &[XYZ<f32>]) -> Vec<Segment> {
        points
            .windows(2)
            .map(|pair| Segment::new(pair[0], pair[1]))
            .collect()
    }

    pub fn length_meters(&self) -> &f32 {
        &self.length_meters
    }
}

#[cfg(test)]
mod tests {
    use commons::geometry::{xy, xyz};

    use super::*;

    #[test]
    fn test_ride_length_meters_pick_up_before_drop_off() {
        // given
        let lift = Lift {
            segments: Segment::segments(&[
                xyz(0.0, 0.0, 0.0),
                xyz(2.0, 0.0, 0.0),
                xyz(2.0, 1.0, 0.0),
                xyz(0.0, 1.0, 0.0),
            ]),
            pick_up: Portal {
                segment: 0,
                position: xy(0, 0),
                direction: Direction::North,
            },
            drop_off: Portal {
                segment: 2,
                position: xy(0, 0),
                direction: Direction::North,
            },
            carousel_id: 0,
        };

        // when
        let result = lift._ride_length_meters();

        // then
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_ride_length_meters_pick_up_after_drop_off() {
        // given
        let lift = Lift {
            segments: Segment::segments(&[
                xyz(0.0, 1.0, 0.0),
                xyz(0.0, 0.0, 0.0),
                xyz(2.0, 0.0, 0.0),
                xyz(2.0, 1.0, 0.0),
            ]),
            pick_up: Portal {
                segment: 1,
                position: xy(0, 0),
                direction: Direction::North,
            },
            drop_off: Portal {
                segment: 0,
                position: xy(0, 0),
                direction: Direction::North,
            },
            carousel_id: 0,
        };

        // when
        let result = lift._ride_length_meters();

        // then
        assert_eq!(result, 3.0);
    }
}
