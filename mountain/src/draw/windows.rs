use std::collections::HashMap;

use commons::grid::Grid;
use commons::unsafe_ordering::unsafe_ordering;
use engine::graphics::utils::{transformation_matrix, triangles_from_quads, Transformation};
use engine::graphics::Graphics;

use crate::draw::door;
use crate::draw::model::window::MODEL;
use crate::model::building::{Building, Window};
use crate::model::door::Door;

const TOTAL_DOOR_HEIGHT: f32 = door::HEIGHT * (1.0 + door::ROOF_HEIGHT) + 1.0;

pub struct Drawing {
    pub index: usize,
}

impl Drawing {
    pub fn init(graphics: &mut dyn Graphics) -> Drawing {
        let triangles = triangles_from_quads(&[MODEL]);
        let index = graphics.create_instanced_triangles(&triangles).unwrap();
        Drawing { index }
    }

    pub fn draw(
        &self,
        graphics: &mut dyn Graphics,
        terrain: &Grid<f32>,
        buildings: &HashMap<usize, Building>,
        building_to_doors: &HashMap<usize, Vec<&Door>>,
    ) {
        let empty = vec![];
        let world_matrices = buildings
            .iter()
            .filter(|(_, building)| !building.under_construction)
            .flat_map(|(building_id, building)| {
                let doors = building_to_doors.get(building_id).unwrap_or(&empty);

                building.windows.iter().filter(|window| {
                    !doors
                        .iter()
                        .any(|door| is_obscured_by_door(terrain, window, door))
                })
            })
            .map(|window| Transformation {
                translation: Some(window.position),
                yaw: Some(window.direction.angle()),
                ..Transformation::default()
            })
            .map(transformation_matrix)
            .map(Some)
            .collect::<Vec<_>>();

        graphics
            .update_instanced_triangles(&self.index, &world_matrices)
            .unwrap();
    }
}

fn is_obscured_by_door(terrain: &Grid<f32>, window: &Window, door: &Door) -> bool {
    let z_from = door
        .footprint
        .iter()
        .map(|position| terrain[position])
        .max_by(unsafe_ordering)
        .unwrap();
    let z_to = z_from + TOTAL_DOOR_HEIGHT;

    window.position.x >= door.footprint.from.x as f32
        && window.position.x <= door.footprint.to.x as f32
        && window.position.y >= door.footprint.from.y as f32
        && window.position.y <= door.footprint.to.y as f32
        && window.position.z >= z_from
        && window.position.z <= z_to
}
