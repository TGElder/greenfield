use commons::geometry::xyz;
use engine::graphics::elements::{Quad, Triangle};
use engine::graphics::utils::{triangle_normal, triangles_from_quads};

#[derive(Clone, Copy)]
pub enum Color {
    Triangle,
    Quad,
}

const BASE: Quad<Color> = Quad {
    corners: [
        xyz(-0.5, 0.5, 0.0),
        xyz(0.5, 0.5, 0.0),
        xyz(0.5, -0.5, 0.0),
        xyz(-0.5, -0.5, 0.0),
    ],
    color: Color::Quad,
};

pub fn model() -> Vec<Triangle<Color>> {
    let base = BASE.corners;

    let peak_1 = (base[0] + base[1]) / 2.0 + xyz(0.0, 0.0, 1.0);
    let triangle_1 = [base[0], peak_1, base[1]];
    let triangle_1 = Triangle {
        corners: triangle_1,
        normal: triangle_normal(&triangle_1),
        color: Color::Triangle,
    };

    let peak_2 = (base[2] + base[3]) / 2.0 + xyz(0.0, 0.0, 1.0);
    let triangle_2 = [base[2], peak_2, base[3]];
    let triangle_2 = Triangle {
        corners: triangle_2,
        normal: triangle_normal(&triangle_2),
        color: Color::Triangle,
    };

    let side_1 = Quad {
        corners: [base[0], base[3], peak_2, peak_1],
        color: Color::Quad,
    };

    let side_2 = Quad {
        corners: [base[1], peak_1, peak_2, base[2]],
        color: Color::Quad,
    };

    triangles_from_quads(&[BASE, side_1, side_2])
        .drain(..)
        .chain([triangle_1, triangle_2])
        .collect()
}
