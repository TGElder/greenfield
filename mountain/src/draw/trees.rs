use std::f32::consts::PI;

use commons::geometry::{xyz, XYZ};
use commons::grid::Grid;
use engine::graphics::Graphics;
use nalgebra::Matrix4;
use rand::random;

use crate::draw::model;

pub fn draw(graphics: &mut dyn Graphics, trees: &Grid<bool>, terrain: &Grid<f32>) {
    let index = graphics.create_instanced_triangles().unwrap();
    let triangles = model::tree::tree();
    let world_matrices = trees
        .iter()
        .filter(|position| trees[position])
        .map(|position| xyz(position.x as f32, position.y as f32, terrain[position]))
        .map(|XYZ { x, y, z }| {
            let angle = random::<f32>() * PI * 2.0;

            let cos = angle.cos();
            let sin = angle.sin();
            let rotation: Matrix4<f32> = [
                [cos, sin, 0.0, 0.0],
                [-sin, cos, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
            .into();

            let translation: Matrix4<f32> = [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [x, y, z, 1.0],
            ]
            .into();

            translation * rotation
        })
        .collect::<Vec<_>>();
    graphics
        .draw_instanced_triangles(&index, &triangles, &world_matrices)
        .unwrap();
}
