use std::env::temp_dir;
use std::f32::consts::PI;

use commons::color::Color;

use crate::glium_backend::graphics;
use crate::graphics::elements::Quad;
use crate::graphics::projections::isometric;

use super::*;

fn cube_quads() -> Vec<Quad> {
    let la = [-0.5, -0.5, -0.5];
    let lb = [0.5, -0.5, -0.5];
    let lc = [0.5, 0.5, -0.5];
    let ld = [-0.5, 0.5, -0.5];
    let ua = [-0.5, -0.5, 0.5];
    let ub = [0.5, -0.5, 0.5];
    let uc = [0.5, 0.5, 0.5];
    let ud = [-0.5, 0.5, 0.5];

    vec![
        Quad {
            id: 1,
            corners: [ld, lc, lb, la],
            color: Color::rgb(1.0, 0.0, 0.0),
        },
        Quad {
            id: 2,
            corners: [ua, ub, uc, ud],
            color: Color::rgb(1.0, 0.0, 0.0),
        },
        Quad {
            id: 3,
            corners: [ua, la, lb, ub],
            color: Color::rgb(0.0, 1.0, 0.0),
        },
        Quad {
            id: 4,
            corners: [uc, lc, ld, ud],
            color: Color::rgb(0.0, 1.0, 0.0),
        },
        Quad {
            id: 5,
            corners: [ub, lb, lc, uc],
            color: Color::rgb(0.0, 0.0, 1.0),
        },
        Quad {
            id: 6,
            corners: [ud, ld, la, ua],
            color: Color::rgb(0.0, 0.0, 1.0),
        },
    ]
}

#[test]
fn render_cube() {
    // given
    let mut graphics = GliumGraphics::headless(graphics::Parameters {
        name: "Test".to_string(),
        width: 256,
        height: 256,
        projection: Box::new(isometric::Projection::new(isometric::Parameters {
            pitch: PI / 4.0,
            yaw: PI * (5.0 / 8.0),
            scale: 1.0,
        })),
    })
    .unwrap();

    // when
    graphics.add_quads(&cube_quads()).unwrap();
    graphics.render().unwrap();

    let temp_path = temp_dir().join("test.png");
    let temp_path = temp_path.to_str().unwrap();
    graphics.screenshot(temp_path).unwrap();

    // then
    let actual = image::open(temp_path).unwrap();
    let expected = image::open("test_resources/graphics/render_cube.png").unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn id_at() {
    // given
    let mut graphics = GliumGraphics::headless(graphics::Parameters {
        name: "Test".to_string(),
        width: 256,
        height: 256,
        projection: Box::new(isometric::Projection::new(isometric::Parameters {
            pitch: PI / 4.0,
            yaw: PI * (5.0 / 8.0),
            scale: 1.0,
        })),
    })
    .unwrap();

    // when
    graphics.add_quads(&cube_quads()).unwrap();
    graphics.render().unwrap();

    // then
    assert_eq!(graphics.id_at((162, 141)).unwrap(), 2);
    assert_eq!(graphics.id_at((162, 142)).unwrap(), 5);
    assert_eq!(graphics.id_at((163, 141)).unwrap(), 4);
    assert_eq!(graphics.id_at((250, 250)).unwrap(), 0);
}
