use commons::color::Rgb;
use commons::geometry::xyz;
use engine::graphics::elements::Quad;

const COLOR: Rgb<f32> = Rgb::new(0.0, 0.0, 0.0);

pub const MODEL: Quad<Rgb<f32>> = Quad {
    corners: [
        xyz(-0.5, -0.01, -0.5),
        xyz(0.5, -0.01, -0.5),
        xyz(0.5, -0.01, 0.5),
        xyz(-0.5, -0.01, 0.5),
    ],
    color: COLOR,
};
