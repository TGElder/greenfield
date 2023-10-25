use commons::color::Rgb;
use commons::geometry::{xyz, XYZ};

use engine::graphics::elements::Quad;
use engine::graphics::Graphics;

const BLACK: Rgb<f32> = Rgb::new(0.0, 0.0, 0.0);

pub fn draw(graphics: &mut dyn Graphics, index: &usize, points: &[XYZ<f32>], height: f32) {
    let quads = points
        .windows(2)
        .flat_map(|pair| {
            let from = pair[0];
            let to = pair[1];
            [
                Quad {
                    color: BLACK,
                    corners: [
                        xyz(from.x, from.y, from.z),
                        xyz(from.x, from.y, from.z + height),
                        xyz(to.x, to.y, to.z + height),
                        xyz(to.x, to.y, to.z),
                    ],
                },
                Quad {
                    color: BLACK,
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

    graphics.draw_quads(index, &quads).unwrap();
}
