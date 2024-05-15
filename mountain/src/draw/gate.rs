use commons::color::Rgb;
use commons::geometry::{xyz, XYRectangle, XYZ};

use commons::grid::Grid;
use commons::unsafe_ordering::unsafe_ordering;
use engine::graphics::elements::Quad;
use engine::graphics::models::cube;
use engine::graphics::transform::{Recolor, Transform};
use engine::graphics::utils::{transformation_matrix, triangles_from_quads, Transformation};
use engine::graphics::Graphics;

use crate::model::entrance::Entrance;
use crate::model::gate::Gate;

const GROUND_TO_BAR_METERS: f32 = 2.0;
const BAR_HEIGHT: f32 = 0.5;
const STRUCTURE_COLOR: Rgb<f32> = Rgb::new(0.0, 0.0, 0.0);
const BANNER_COLOR: Rgb<f32> = Rgb::new(1.0, 1.0, 0.0);

pub fn draw(
    graphics: &mut dyn Graphics,
    index: &usize,
    gate: &Gate,
    entrance: &Entrance,
    terrain: &Grid<f32>,
    piste_map: &Grid<Option<usize>>,
) {
    let Gate { footprint, .. } = gate;
    let XYRectangle { from, to } = footprint;
    let from = xyz(from.x as f32, from.y as f32, terrain[from]);
    let to = xyz(to.x as f32, to.y as f32, terrain[to]);

    let Some(max_ground_height) = footprint
        .iter()
        .map(|position| terrain[position])
        .max_by(unsafe_ordering)
    else {
        println!("WARN: Cannot draw gate with no footprint");
        return;
    };
    let bar_bottom_z = max_ground_height + GROUND_TO_BAR_METERS;
    let from_pole_height = bar_bottom_z - from.z;
    let to_pole_height = bar_bottom_z - to.z;

    let from_pole = translated_and_scaled_cube(
        xyz(from.x, from.y, from.z + from_pole_height / 2.0),
        xyz(0.1, 0.1, from_pole_height),
        &|_| STRUCTURE_COLOR,
    );
    let to_pole = translated_and_scaled_cube(
        xyz(to.x, to.y, to.z + to_pole_height / 2.0),
        xyz(0.1, 0.1, to_pole_height),
        &|_| STRUCTURE_COLOR,
    );

    let entrance_side = entrance_side(gate, entrance, piste_map);
    let coloring = |side: &cube::Side| {
        if *side == entrance_side {
            BANNER_COLOR
        } else {
            STRUCTURE_COLOR
        }
    };
    let banner = translated_and_scaled_cube(
        xyz(
            from.x + (to.x - from.x) / 2.0,
            from.y + (to.y - from.y) / 2.0,
            bar_bottom_z + BAR_HEIGHT / 2.0,
        ),
        xyz(
            (from.x - to.x).abs() + 0.1,
            (from.y - to.y).abs() + 0.1,
            BAR_HEIGHT,
        ),
        &coloring,
    );

    let quads = from_pole.chain(to_pole).chain(banner).collect::<Vec<_>>();
    let triangles = triangles_from_quads(&quads);
    graphics.draw_triangles(index, &triangles).unwrap();
}

fn entrance_side(
    Gate {
        footprint: XYRectangle { from, to },
        ..
    }: &Gate,
    Entrance {
        destination_piste_id,
        ..
    }: &Entrance,
    piste_map: &Grid<Option<usize>>,
) -> cube::Side {
    match (
        from.x == to.x,
        piste_map[from] == Some(*destination_piste_id),
    ) {
        (true, true) => cube::Side::Left,
        (true, false) => cube::Side::Right,
        (false, true) => cube::Side::Back,
        (false, false) => cube::Side::Front,
    }
}

fn translated_and_scaled_cube(
    translation: XYZ<f32>,
    scale: XYZ<f32>,
    coloring: &dyn Fn(&cube::Side) -> Rgb<f32>,
) -> impl Iterator<Item = Quad<Rgb<f32>>> {
    let transformation = transformation_matrix(Transformation {
        translation: Some(translation),
        scale: Some(scale),
        ..Transformation::default()
    });

    cube::model()
        .transform(&transformation)
        .recolor(coloring)
        .into_iter()
}
