use commons::geometry::{XY, XYZ};
use nalgebra::Point3;

pub struct Lift {
    pub segments: Vec<Segment>,
    pub pick_up: Portal,
    pub drop_off: Portal,
}

pub struct Segment {
    pub from: XYZ<f32>,
    pub to: XYZ<f32>,
    length_meters: f32,
}

pub struct Portal {
    pub segment: usize,
    pub position: XY<u32>,
}

impl Segment {
    pub fn _new(from: XYZ<f32>, to: XYZ<f32>) -> Segment {
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
            .map(|pair| Segment::_new(pair[0], pair[1]))
            .collect()
    }

    pub fn length_meters(&self) -> &f32 {
        &self.length_meters
    }
}
