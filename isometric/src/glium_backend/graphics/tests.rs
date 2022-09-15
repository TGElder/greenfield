use std::f32::consts::PI;
use std::time::Duration;

use commons::color::Color;

use crate::game::{self, Game};
use crate::glium_backend::{engine, graphics};
use crate::graphics::Quad;

use super::*;

#[test]
fn render_cube() {
    let engine = Engine::new(engine::Parameters {
        frame_duration: Duration::from_nanos(16_666_667),
    });
    let mut graphics = Graphics::with_engine(
        graphics::Parameters {
            name: "Test".to_string(),
            width: 256,
            height: 256,
            pitch: PI / 4.0,
            yaw: PI * (5.0 / 8.0),
            scale: 1.0,
        },
        &engine,
    );

    let la = [-0.5, -0.5, -0.5];
    let lb = [-0.5, 0.5, -0.5];
    let lc = [0.5, 0.5, -0.5];
    let ld = [0.5, -0.5, -0.5];
    let ua = [-0.5, -0.5, 0.5];
    let ub = [-0.5, 0.5, 0.5];
    let uc = [0.5, 0.5, 0.5];
    let ud = [0.5, -0.5, 0.5];

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
            corners: [ud, ld, lc, uc],
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

    graphics.add_quads(&quads);

    engine.run(Test { frame: 0 }, graphics);

    struct Test {
        frame: u64,
    }

    impl Game for Test {
        fn update(&mut self, graphics: &mut dyn GraphicsBackend) -> game::State {
            if self.frame == 1 {
                graphics.screenshot("test_resources/graphics/render_triangle.png");
                return game::State::Terminated;
            }
            self.frame += 1;
            game::State::Running
        }
    }
}
