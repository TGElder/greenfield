use commons::geometry::{xyz, XY, XYZ};
use commons::grid::Grid;
use engine::graphics::transform::Transform;
use engine::graphics::utils::{transformation_matrix, Transformation};
use nalgebra::Matrix4;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum StructureClass {
    ChairliftBaseStation,
}

impl StructureClass {
    pub fn wire_path_out(&self) -> Vec<[XYZ<f32>; 2]> {
        match self {
            StructureClass::ChairliftBaseStation => {
                vec![[xyz(-0.5, -0.5, 0.5), xyz(0.5, -0.5, 0.5)]]
            }
        }
    }

    pub fn wire_path_back(&self) -> Vec<[XYZ<f32>; 2]> {
        match self {
            StructureClass::ChairliftBaseStation => vec![[xyz(0.5, 0.5, 0.5), xyz(-0.5, 0.5, 0.5)]],
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Structure {
    pub class: StructureClass,
    pub position: XY<u32>,
    pub footprint: XYZ<u32>,
    pub rotation: f32,
    pub under_construction: bool,
}

pub fn get_wire_path(structures: &[&Structure], terrain: &Grid<f32>) -> Vec<[XYZ<f32>; 2]> {
    let unlinked = structures
        .iter()
        .flat_map(|structure| get_wire_path_out(structure, terrain))
        .chain(
            structures
                .iter()
                .rev()
                .flat_map(|structure| get_wire_path_back(structure, terrain)),
        )
        .collect::<Vec<_>>();

    let mut linked = unlinked
        .windows(2)
        .flat_map(|window| [window[0], [window[0][1], window[1][0]]])
        .collect::<Vec<_>>();

    if let (Some(first), Some(last)) = (unlinked.first(), unlinked.last()) {
        linked.push(*last);
        linked.push([last[1], first[0]]);
    }

    linked
}

pub fn transformation_matrix_for_structure(
    structure: &Structure,
    terrain: &Grid<f32>,
) -> Matrix4<f32> {
    transformation_matrix(Transformation {
        translation: Some(xyz(
            structure.position.x as f32,
            structure.position.y as f32,
            terrain[structure.position],
        )),
        scale: Some(xyz(
            structure.footprint.x as f32,
            structure.footprint.y as f32,
            structure.footprint.z as f32,
        )),
        yaw: Some(structure.rotation),
        ..Transformation::default()
    })
}

pub fn get_wire_path_out(structure: &Structure, terrain: &Grid<f32>) -> Vec<[XYZ<f32>; 2]> {
    let matrix = transformation_matrix_for_structure(structure, terrain);
    structure
        .class
        .wire_path_out()
        .iter()
        .map(|point| [point[0].transform(&matrix), point[1].transform(&matrix)])
        .collect()
}

pub fn get_wire_path_back(structure: &Structure, terrain: &Grid<f32>) -> Vec<[XYZ<f32>; 2]> {
    let matrix = transformation_matrix_for_structure(structure, terrain);
    structure
        .class
        .wire_path_back()
        .iter()
        .map(|point| [point[0].transform(&matrix), point[1].transform(&matrix)])
        .collect()
}
