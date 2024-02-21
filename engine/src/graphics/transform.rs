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

trait TransformInternal {
    fn transform(
        &self,
        transformation: &Matrix4<f32>,
        normal_transformation: &Matrix4<f32>,
    ) -> Self;
}

impl TransformInternal for Triangle {
    fn transform(
        &self,
        transformation: &Matrix4<f32>,
        normal_transformation: &Matrix4<f32>,
    ) -> Self {
        Triangle {
            corners: [
                self.corners[0].transform(transformation),
                self.corners[1].transform(transformation),
                self.corners[2].transform(transformation),
            ],
            normal: self.normal.transform(normal_transformation),
            color: self.color,
        }
    }
}

impl TransformInternal for Quad {
    fn transform(&self, transformation: &Matrix4<f32>, _: &Matrix4<f32>) -> Self {
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

fn normal_transformation(transformation: &Matrix4<f32>) -> Matrix4<f32> {
    transformation.try_inverse().unwrap().transpose()
}

impl Transform for Vec<Triangle> {
    fn transform(&self, transformation: &Matrix4<f32>) -> Self {
        let normal_transformation = normal_transformation(transformation);
        self.iter()
            .map(|quad| quad.transform(transformation, &normal_transformation))
            .collect()
    }
}

impl Transform for Vec<Quad> {
    fn transform(&self, transformation: &Matrix4<f32>) -> Self {
        let normal_transformation = normal_transformation(transformation);
        self.iter()
            .map(|quad| quad.transform(transformation, &normal_transformation))
            .collect()
    }
}
