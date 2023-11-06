use commons::color::Rgb;
use commons::geometry::xyz;
use engine::graphics::elements::Quad;

const TORSO_FRONT: Quad = Quad {
    color: Rgb::new(1.0, 0.0, 0.0),
    corners: [
        xyz(0.0, -0.25, 0.0),
        xyz(0.0, 0.25, 0.0),
        xyz(0.0, 0.25, 0.5),
        xyz(0.0, -0.25, 0.5),
    ],
};

const TORSO_BACK: Quad = Quad {
    color: Rgb::new(1.0, 0.0, 0.0),
    corners: [
        xyz(0.0, -0.25, 0.5),
        xyz(0.0, 0.25, 0.5),
        xyz(0.0, 0.25, 0.0),
        xyz(0.0, -0.25, 0.0),
    ],
};

const LEGS_TOP: Quad = Quad {
    color: Rgb::new(1.0, 0.0, 0.0),
    corners: [
        xyz(0.25, -0.25, 0.0),
        xyz(0.25, 0.25, 0.0),
        xyz(0.0, 0.25, 0.0),
        xyz(0.0, -0.25, 0.0),
    ],
};

const LEGS_BOTTOM_FRONT: Quad = Quad {
    color: Rgb::new(1.0, 0.0, 0.0),
    corners: [
        xyz(0.25, -0.25, -0.25),
        xyz(0.25, 0.25, -0.25),
        xyz(0.25, 0.25, 0.0),
        xyz(0.25, -0.25, 0.0),
    ],
};

const LEGS_BOTTOM_BACK: Quad = Quad {
    color: Rgb::new(1.0, 0.0, 0.0),
    corners: [
        xyz(0.25, -0.25, 0.0),
        xyz(0.25, 0.25, 0.0),
        xyz(0.25, 0.25, -0.25),
        xyz(0.25, -0.25, -0.25),
    ],
};

const SKIS: Quad = Quad {
    color: Rgb::new(1.0, 0.0, 0.0),
    corners: [
        xyz(-0.25, -0.25, -0.25),
        xyz(0.75, -0.25, -0.25),
        xyz(0.75, 0.25, -0.25),
        xyz(-0.25, 0.25, -0.25),
    ],
};

pub const MODEL: [Quad; 6] = [
    TORSO_FRONT,
    TORSO_BACK,
    LEGS_TOP,
    LEGS_BOTTOM_FRONT,
    LEGS_BOTTOM_BACK,
    SKIS,
];
