use commons::color::Rgb;
use commons::geometry::xyz;
use engine::graphics::elements::Quad;

const COLOR: Rgb<f32> = Rgb::new(0.86, 0.01, 0.01);

const BODY_FRONT: Quad = Quad {
    color: COLOR,
    corners: [
        xyz(0.0, -0.25, 0.0),
        xyz(0.0, 0.25, 0.0),
        xyz(0.0, 0.25, 1.0),
        xyz(0.0, -0.25, 1.0),
    ],
};

const BODY_BACK: Quad = Quad {
    color: COLOR,
    corners: [
        xyz(0.0, -0.25, 1.0),
        xyz(0.0, 0.25, 1.0),
        xyz(0.0, 0.25, 0.0),
        xyz(0.0, -0.25, 0.0),
    ],
};

const SKIS: Quad = Quad {
    color: COLOR,
    corners: [
        xyz(-0.5, -0.25, 0.0),
        xyz(0.5, -0.25, 0.0),
        xyz(0.5, 0.25, 0.0),
        xyz(-0.5, 0.25, 0.0),
    ],
};

pub const WITHOUT_SKIS: [Quad; 2] = [BODY_FRONT, BODY_BACK];
pub const WITH_SKIS: [Quad; 3] = [BODY_FRONT, BODY_BACK, SKIS];
