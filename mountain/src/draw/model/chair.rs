use commons::color::Rgb;
use commons::geometry::xyz;
use engine::graphics::elements::Quad;

const GREY: Rgb<f32> = Rgb::new(0.125, 0.125, 0.125);

const POLE_FRONT: Quad = Quad {
    color: GREY,
    corners: [
        xyz(0.0, -0.05, -1.0),
        xyz(0.0, 0.05, -1.0),
        xyz(0.0, 0.05, -0.0),
        xyz(0.0, -0.05, -0.0),
    ],
};

const POLE_BACK: Quad = Quad {
    color: GREY,
    corners: [
        xyz(0.0, -0.05, -0.0),
        xyz(0.0, 0.05, -0.0),
        xyz(0.0, 0.05, -1.0),
        xyz(0.0, -0.05, -1.0),
    ],
};

const CHAIR_REST_FRONT: Quad = Quad {
    color: GREY,
    corners: [
        xyz(0.0, -0.75, -1.5),
        xyz(0.0, 0.75, -1.5),
        xyz(0.0, 0.75, -1.0),
        xyz(0.0, -0.75, -1.0),
    ],
};

const CHAIR_REST_BACK: Quad = Quad {
    color: GREY,
    corners: [
        xyz(0.0, -0.75, -1.0),
        xyz(0.0, 0.75, -1.0),
        xyz(0.0, 0.75, -1.5),
        xyz(0.0, -0.75, -1.5),
    ],
};

const CHAIR_SEAT: Quad = Quad {
    color: GREY,
    corners: [
        xyz(0.5, -0.75, -1.5),
        xyz(0.5, 0.75, -1.5),
        xyz(0.0, 0.75, -1.5),
        xyz(0.0, -0.75, -1.5),
    ],
};

pub fn model() -> Vec<Quad> {
    vec![
        POLE_FRONT,
        POLE_BACK,
        CHAIR_REST_FRONT,
        CHAIR_REST_BACK,
        CHAIR_SEAT,
    ]
}
