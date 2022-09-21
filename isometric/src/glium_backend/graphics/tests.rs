use std::env::temp_dir;
use std::f32::consts::PI;

use commons::color::Color;

use crate::glium_backend::graphics;
use crate::graphics::Quad;

use super::*;

#[test]
fn render_cube() {
    // given
    let mut graphics = Graphics::headless(graphics::Parameters {
        name: "Test".to_string(),
        width: 256,
        height: 256,
        pitch: PI / 4.0,
        yaw: PI * (5.0 / 8.0),
        scale: 1.0,
    });

    let la = [-0.5, -0.5, -0.5];
    let lb = [0.5, -0.5, -0.5];
    let lc = [0.5, 0.5, -0.5];
    let ld = [-0.5, 0.5, -0.5];
    let ua = [-0.5, -0.5, 0.5];
    let ub = [0.5, -0.5, 0.5];
    let uc = [0.5, 0.5, 0.5];
    let ud = [-0.5, 0.5, 0.5];

    let quads = vec![
        Quad {
            id: 0,
            corners: [ld, lc, lb, la],
            color: Color::rgb(1.0, 0.0, 0.0),
        },
        Quad {
            id: 0,
            corners: [ua, ub, uc, ud],
            color: Color::rgb(1.0, 0.0, 0.0),
        },
        Quad {
            id: 0,
            corners: [ua, la, lb, ub],
            color: Color::rgb(0.0, 1.0, 0.0),
        },
        Quad {
            id: 0,
            corners: [uc, lc, ld, ud],
            color: Color::rgb(0.0, 1.0, 0.0),
        },
        Quad {
            id: 0,
            corners: [ub, lb, lc, uc],
            color: Color::rgb(0.0, 0.0, 1.0),
        },
        Quad {
            id: 0,
            corners: [ud, ld, la, ua],
            color: Color::rgb(0.0, 0.0, 1.0),
        },
    ];

    // when
    graphics.add_quads(&quads);
    graphics.render();

    let temp_path = temp_dir().join("test.png");
    let temp_path = temp_path.to_str().unwrap();
    graphics.screenshot(temp_path);

    // then
    let actual = image::open(temp_path).unwrap();
    let expected = image::open("test_resources/graphics/render_cube.png").unwrap();
    assert_eq!(actual, expected);
}
