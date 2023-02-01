use std::env::temp_dir;
use std::f32::consts::PI;

use commons::color::Rgb;

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
            corners: [ld, lc, lb, la],
            color: Rgb::new(1.0, 0.0, 0.0),
        },
        Quad {
            corners: [ua, ub, uc, ud],
            color: Rgb::new(1.0, 0.0, 0.0),
        },
        Quad {
            corners: [ua, la, lb, ub],
            color: Rgb::new(0.0, 1.0, 0.0),
        },
        Quad {
            corners: [uc, lc, ld, ud],
            color: Rgb::new(0.0, 1.0, 0.0),
        },
        Quad {
            corners: [ub, lb, lc, uc],
            color: Rgb::new(0.0, 0.0, 1.0),
        },
        Quad {
            corners: [ud, ld, la, ua],
            color: Rgb::new(0.0, 0.0, 1.0),
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
fn look_at() {
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
    graphics.look_at(&[-0.5, -0.5, -0.5], &(192, 64)).unwrap();
    graphics.render().unwrap();

    let temp_path = temp_dir().join("test.png");
    let temp_path = temp_path.to_str().unwrap();
    graphics.screenshot(temp_path).unwrap();

    // then
    let actual = image::open(temp_path).unwrap();
    let expected = image::open("test_resources/graphics/look_at.png").unwrap();
    assert_eq!(actual, expected);

    // when
    graphics.look_at(&[-0.5, -0.5, -0.5], &(192, 64)).unwrap();
    graphics.render().unwrap();
    graphics.screenshot(temp_path).unwrap();

    // then
    let actual = image::open(temp_path).unwrap();
    assert_eq!(actual, expected);
}
