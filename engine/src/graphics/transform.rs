use commons::geometry::{xyz, XYZ};
use nalgebra::{Matrix4, Vector4};

use crate::graphics::elements::{Quad, Triangle};

pub trait Transform {
    fn transform(&self, transformation: &Matrix4<f32>) -> Self;
}

impl Transform for XYZ<f32> {
    fn transform(&self, transformation: &Matrix4<f32>) -> Self {
        let point = Vector4::new(self.x, self.y, self.z, 1.0);
        let transformed = transformation * point;
        xyz(transformed.x, transformed.y, transformed.z)
    }
}

impl Transform for Triangle {
    fn transform(&self, transformation: &Matrix4<f32>) -> Self {
        Triangle {
            corners: [
                self.corners[0].transform(transformation),
                self.corners[1].transform(transformation),
                self.corners[2].transform(transformation),
            ],
            color: self.color,
        }
    }
}

impl Transform for Quad {
    fn transform(&self, transformation: &Matrix4<f32>) -> Self {
        Quad {
            corners: [
                self.corners[0].transform(transformation),
                self.corners[1].transform(transformation),
                self.corners[2].transform(transformation),
                self.corners[3].transform(transformation),
            ],
            color: self.color,
        }
    }
}

impl Transform for Vec<Quad> {
    fn transform(&self, transformation: &Matrix4<f32>) -> Self {
        self.iter()
            .map(|quad| quad.transform(transformation))
            .collect()
    }
}
