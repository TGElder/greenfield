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

impl<T> TransformInternal for Triangle<T>
where
    T: Copy,
{
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

impl<T> TransformInternal for Quad<T>
where
    T: Copy,
{
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

impl<T> Transform for Vec<Triangle<T>>
where
    T: Copy,
{
    fn transform(&self, transformation: &Matrix4<f32>) -> Self {
        let normal_transformation = normal_transformation(transformation);
        self.iter()
            .map(|quad| quad.transform(transformation, &normal_transformation))
            .collect()
    }
}

impl<T> Transform for Vec<Quad<T>>
where
    T: Copy,
{
    fn transform(&self, transformation: &Matrix4<f32>) -> Self {
        let normal_transformation = normal_transformation(transformation);
        self.iter()
            .map(|quad| quad.transform(transformation, &normal_transformation))
            .collect()
    }
}

pub trait Recolor<T, U, V> {
    fn recolor(&self, recoloring: &dyn Fn(&T) -> U) -> V;
}

impl<T, U> Recolor<T, U, Triangle<U>> for Triangle<T> {
    fn recolor(&self, recoloring: &dyn Fn(&T) -> U) -> Triangle<U> {
        Triangle {
            corners: self.corners,
            normal: self.normal,
            color: recoloring(&self.color),
        }
    }
}

impl<T, U> Recolor<T, U, Quad<U>> for Quad<T> {
    fn recolor(&self, recoloring: &dyn Fn(&T) -> U) -> Quad<U> {
        Quad {
            corners: self.corners,
            color: recoloring(&self.color),
        }
    }
}

impl<T, U> Recolor<T, U, Vec<Triangle<U>>> for Vec<Triangle<T>> {
    fn recolor(&self, recoloring: &dyn Fn(&T) -> U) -> Vec<Triangle<U>> {
        self.iter()
            .map(|triangle| triangle.recolor(recoloring))
            .collect()
    }
}

impl<T, U> Recolor<T, U, Vec<Quad<U>>> for Vec<Quad<T>> {
    fn recolor(&self, recoloring: &dyn Fn(&T) -> U) -> Vec<Quad<U>> {
        self.iter().map(|quad| quad.recolor(recoloring)).collect()
    }
}
