use commons::geometry::{xyz, XY};
use commons::grid::Grid;
use engine::graphics::utils::transformation_matrix;
use engine::graphics::Graphics;
use nalgebra::Matrix4;

use crate::draw::model::tree;
use crate::model::tree::Tree;

pub struct Drawing {
    pub index: usize,
}

impl Drawing {
    pub fn init(graphics: &mut dyn Graphics, trees: &Grid<Option<Tree>>) -> Drawing {
        let triangles = tree::model();
        let tree_count = trees
            .iter()
            .filter(|position| trees[position].is_some())
            .count();
        let index = graphics
            .create_instanced_triangles(&triangles, &tree_count)
            .unwrap();
        Drawing { index }
    }

    pub fn update(
        &self,
        graphics: &mut dyn Graphics,
        trees: &Grid<Option<Tree>>,
        terrain: &Grid<f32>,
        piste_map: &Grid<Option<usize>>,
    ) {
        let world_matrices = trees
            .iter()
            .flat_map(|position| trees[position].as_ref().map(|tree| (position, tree)))
            .map(|(position, tree)| {
                if piste_map[position].is_some() {
                    None
                } else {
                    Some(tree_world_matrix(terrain, &position, tree))
                }
            })
            .collect::<Vec<_>>();
        graphics
            .update_instanced_triangles(&self.index, &world_matrices)
            .unwrap();
    }
}

fn tree_world_matrix(
    terrain: &Grid<f32>,
    position: &XY<u32>,
    Tree { yaw, height }: &Tree,
) -> Matrix4<f32> {
    transformation_matrix(
        xyz(position.x as f32, position.y as f32, terrain[position]),
        *yaw,
        0.0,
        0.0,
        xyz(*height, *height, *height),
    )
}
