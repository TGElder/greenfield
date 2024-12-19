use commons::color::Rgb;
use commons::geometry::{xyz, XYZ};

use engine::graphics::elements::Quad;
use engine::graphics::utils::triangles_from_quads;
use engine::graphics::Graphics;

const GREY: Rgb<f32> = Rgb::new(0.125, 0.125, 0.125);

pub fn draw(graphics: &mut dyn Graphics, index: &usize, segments: &[[&XYZ<f32>; 2]], height: f32) {
    let quads = segments
        .iter()
        .flat_map(|segment| {
            let from = segment[0];
            let to = segment[1];
            [
                Quad {
                    color: GREY,
                    corners: [
                        xyz(from.x, from.y, from.z),
                        xyz(from.x, from.y, from.z + height),
                        xyz(to.x, to.y, to.z + height),
                        xyz(to.x, to.y, to.z),
                    ],
                },
                Quad {
                    color: GREY,
                    corners: [
                        xyz(from.x, from.y, from.z),
                        xyz(to.x, to.y, to.z),
                        xyz(to.x, to.y, to.z + height),
                        xyz(from.x, from.y, from.z + height),
                    ],
                },
            ]
        })
        .collect::<Vec<_>>();

    let triangles = triangles_from_quads(&quads);
    graphics.draw_triangles(index, &triangles).unwrap();
}

pub fn draw2(graphics: &mut dyn Graphics, index: &usize, segments: &[[XYZ<f32>; 2]], height: f32) {
    let quads = segments
        .iter()
        .flat_map(|segment| {
            let from = segment[0];
            let to = segment[1];
            [
                Quad {
                    color: GREY,
                    corners: [
                        xyz(from.x, from.y, from.z),
                        xyz(from.x, from.y, from.z + height),
                        xyz(to.x, to.y, to.z + height),
                        xyz(to.x, to.y, to.z),
                    ],
                },
                Quad {
                    color: GREY,
                    corners: [
                        xyz(from.x, from.y, from.z),
                        xyz(to.x, to.y, to.z),
                        xyz(to.x, to.y, to.z + height),
                        xyz(from.x, from.y, from.z + height),
                    ],
                },
            ]
        })
        .collect::<Vec<_>>();

    let triangles = triangles_from_quads(&quads);
    graphics.draw_hologram(index, &triangles).unwrap();
}
