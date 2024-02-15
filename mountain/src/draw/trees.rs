use commons::geometry::{xyz, XY, XYZ};
use commons::grid::Grid;
use engine::graphics::Graphics;
use nalgebra::Matrix4;

use crate::draw::model::tree;
use crate::model::tree::Tree;

pub fn draw(graphics: &mut dyn Graphics, terrain: &Grid<f32>, trees: &Grid<Option<Tree>>) {
    let index = graphics.create_instanced_triangles().unwrap();
    let triangles = tree::model();
    let world_matrices = trees
        .iter()
        .flat_map(|position| trees[position].as_ref().map(|tree| (position, tree)))
        .map(|(position, tree)| tree_world_matrix(terrain, &position, tree))
        .collect::<Vec<_>>();
    graphics
        .draw_instanced_triangles(&index, &triangles, &world_matrices)
        .unwrap();
}

fn tree_world_matrix(
    terrain: &Grid<f32>,
    position: &XY<u32>,
    Tree { yaw, .. }: &Tree,
) -> Matrix4<f32> {
    let XYZ { x, y, z } = xyz(position.x as f32, position.y as f32, terrain[position]);
    let translation: Matrix4<f32> = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [x, y, z, 1.0],
    ]
    .into();

    let cos = yaw.cos();
    let sin = yaw.sin();
    let rotation: Matrix4<f32> = [
        [cos, sin, 0.0, 0.0],
        [-sin, cos, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
    .into();

    translation * rotation
}
