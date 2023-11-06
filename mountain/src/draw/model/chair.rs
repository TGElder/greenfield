use commons::color::Rgb;
use commons::geometry::xyz;
use engine::graphics::elements::Quad;

const BLACK: Rgb<f32> = Rgb::new(0.0, 0.0, 0.0);

const POLE_FRONT: Quad = Quad {
    color: BLACK,
    corners: [
        xyz(0.0, -0.05, -1.0),
        xyz(0.0, 0.05, -1.0),
        xyz(0.0, 0.05, -0.0),
        xyz(0.0, -0.05, -0.0),
    ],
};

const POLE_BACK: Quad = Quad {
    color: BLACK,
    corners: [
        xyz(0.0, -0.05, -0.0),
        xyz(0.0, 0.05, -0.0),
        xyz(0.0, 0.05, -1.0),
        xyz(0.0, -0.05, -1.0),
    ],
};

const CHAIR_REST_FRONT: Quad = Quad {
    color: BLACK,
    corners: [
        xyz(0.0, -0.75, -1.5),
        xyz(0.0, 0.75, -1.5),
        xyz(0.0, 0.75, -1.0),
        xyz(0.0, -0.75, -1.0),
    ],
};

const CHAIR_REST_BACK: Quad = Quad {
    color: BLACK,
    corners: [
        xyz(0.0, -0.75, -1.0),
        xyz(0.0, 0.75, -1.0),
        xyz(0.0, 0.75, -1.5),
        xyz(0.0, -0.75, -1.5),
    ],
};

const CHAIR_SEAT: Quad = Quad {
    color: BLACK,
    corners: [
        xyz(0.5, -0.75, -1.5),
        xyz(0.5, 0.75, -1.5),
        xyz(0.0, 0.75, -1.5),
        xyz(0.0, -0.75, -1.5),
    ],
};

pub const MODEL: [Quad; 5] = [
    POLE_FRONT,
    POLE_BACK,
    CHAIR_REST_FRONT,
    CHAIR_REST_BACK,
    CHAIR_SEAT,
];
