use commons::color::Rgb;
use commons::grid::Grid;
use engine::graphics::elements::Triangle;
use engine::graphics::transform::Transform;
use engine::graphics::{DrawMode, Graphics};

use crate::draw::model::{lift_building, line};
use crate::model::lift_building::{LiftBuilding, LiftBuildingClass, LiftBuildings};

pub fn draw(graphics: &mut dyn Graphics, index: &usize, lift: &LiftBuildings, terrain: &Grid<f32>) {
    let wire = lift.wire_path_over_terrain(terrain);

    let mut wire_triangles = line::model(&wire, 0.5);
    let building_triangles = lift
        .buildings
        .iter()
        .flat_map(|building| building_triangles(building, terrain));
    let triangles = wire_triangles
        .drain(..)
        .chain(building_triangles)
        .collect::<Vec<_>>();

    graphics
        .draw_triangles(index, DrawMode::Hologram, &triangles)
        .unwrap();
}

fn building_triangles(building: &LiftBuilding, terrain: &Grid<f32>) -> Vec<Triangle<Rgb<f32>>> {
    let triangles = match &building.class {
        LiftBuildingClass::ChairliftPylon => lift_building::chairlift_pylon(),
        LiftBuildingClass::ChairliftStation => lift_building::chairlift_station(),
    };
    triangles.transform(&building.transformation_matrix(terrain))
}
