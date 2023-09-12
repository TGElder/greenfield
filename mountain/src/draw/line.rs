use commons::color::Rgb;
use commons::geometry::{xyz, XYZ};

use engine::graphics::elements::Quad;
use engine::graphics::Graphics;

const BLACK: Rgb<f32> = Rgb::new(0.0, 0.0, 0.0);

pub fn draw(
    graphics: &mut dyn Graphics,
    index: &usize,
    from: &XYZ<f32>,
    to: &XYZ<f32>,
    height: f32,
) {
    graphics
        .draw_quads(
            index,
            &[
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
            ],
        )
        .unwrap();
}
