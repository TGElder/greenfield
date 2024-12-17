use commons::color::Rgb;
use commons::geometry::{xyz, XYZ};
use commons::grid::Grid;
use engine::graphics::elements::Quad;
use engine::graphics::transform::Transform;
use engine::graphics::utils::{transformation_matrix, triangles_from_quads, Transformation};
use engine::graphics::Graphics;
use nalgebra::Matrix4;

use crate::draw::{line, model};
use crate::model::structure::{Structure, StructureClass};

pub fn draw_chain(
    graphics: &mut dyn Graphics,
    index: &usize,
    wire_index: &usize,
    structures: &[&Structure],
    terrain: &Grid<f32>,
) {
    let wire = get_wire_path(structures, terrain);
    line::draw2(graphics, wire_index, &wire, 0.5);

    let quads = structures
        .iter()
        .map(|structure| get_quads(structure, terrain))
        .collect::<Vec<_>>();

    let quads = quads.into_iter().flatten().collect::<Vec<_>>();

    let triangles = triangles_from_quads(&quads);
    graphics.draw_hologram(index, &triangles).unwrap();
}

pub fn get_quads(structure: &Structure, terrain: &Grid<f32>) -> Vec<Quad<Rgb<f32>>> {
    let model = match &structure.class {
        StructureClass::ChairliftBaseStation => model::chairlift::base_station(),
    };
    model
        .quads
        .transform(&transformation_matrix_for_structure(structure, terrain))
}

fn transformation_matrix_for_structure(structure: &Structure, terrain: &Grid<f32>) -> Matrix4<f32> {
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

fn get_wire_path(structures: &[&Structure], terrain: &Grid<f32>) -> Vec<[XYZ<f32>; 2]> {
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

fn get_wire_path_out(structure: &Structure, terrain: &Grid<f32>) -> Vec<[XYZ<f32>; 2]> {
    let matrix = transformation_matrix_for_structure(structure, terrain);
    structure
        .wire_path_out
        .iter()
        .map(|point| [point[0].transform(&matrix), point[1].transform(&matrix)])
        .collect()
}

fn get_wire_path_back(structure: &Structure, terrain: &Grid<f32>) -> Vec<[XYZ<f32>; 2]> {
    let matrix = transformation_matrix_for_structure(structure, terrain);
    structure
        .wire_path_back
        .iter()
        .map(|point| [point[0].transform(&matrix), point[1].transform(&matrix)])
        .collect()
}
