use std::f32::consts::PI;
use std::time::Duration;

use commons::color::Rgb;
use commons::grid::Grid;
use commons::noise::simplex_noise;
use engine::engine::Engine;
use engine::events::{Event, EventHandler, KeyboardKey};
use engine::glium_backend;
use engine::graphics::elements::Quad;
use engine::graphics::projections::isometric;
use engine::graphics::Graphics;
use engine::handlers::{drag, yaw};
use terrain_gen::with_valleys::{heightmap_from_rises_with_valleys, ValleyParameters};

fn main() {
    let engine = glium_backend::engine::GliumEngine::new(
        Demo {
            frame: 0,
            drag_handler: drag::Handler::new(),
            yaw_handler: yaw::Handler::new(yaw::Parameters {
                initial_angle: 10,
                angles: 16,
                key_plus: KeyboardKey::E,
                key_minus: KeyboardKey::Q,
            }),
        },
        glium_backend::engine::Parameters {
            frame_duration: Duration::from_nanos(16_666_667),
        },
        glium_backend::graphics::Parameters {
            name: "Demo".to_string(),
            width: 512,
            height: 512,
            projection: Box::new(isometric::Projection::new(isometric::Parameters {
                pitch: PI / 4.0,
                yaw: 2.0 * PI * (10.0 / 16.0),
                scale: 1.0 / 64.0,
            })),
        },
    )
    .unwrap();

    engine.run();
}

struct Demo {
    frame: u64,
    drag_handler: drag::Handler,
    yaw_handler: yaw::Handler,
}

impl EventHandler for Demo {
    fn handle(&mut self, event: &Event, engine: &mut dyn Engine, graphics: &mut dyn Graphics) {
        if self.frame == 0 {
            let terrain = get_heightmap();

            let mut quads = Vec::with_capacity(
                (terrain.width() - 1) as usize * (terrain.height() - 1) as usize,
            );
            for x in 0..terrain.width() - 1 {
                for y in 0..terrain.height() - 1 {
                    let z = terrain[(x, y)];
                    let corners = [(0, 0), (1, 0), (1, 1), (0, 1)]
                        .iter()
                        .map(|(dx, dy)| {
                            [
                                (x + dx) as f32,
                                (y + dy) as f32,
                                terrain[(x + dx, y + dy)] * 32.0,
                            ]
                        })
                        .collect::<Vec<_>>();
                    quads.push(Quad {
                        corners: [corners[0], corners[1], corners[2], corners[3]],
                        color: Rgb::new(z, z, z),
                    });
                }
            }

            graphics.add_quads(&quads).unwrap();
            graphics.look_at(&[128.0, 128.0, 0.0], &(256, 256));
        } else if self.frame == 1 {
            graphics.screenshot("screenshot.png").unwrap();
        }

        self.drag_handler.handle(event, engine, graphics);
        self.yaw_handler.handle(event, engine, graphics);

        self.frame += 1;
    }
}

fn get_heightmap() -> Grid<f32> {
    let power = 8;
    let weights = (0..power + 1)
        .map(|i| 1.0f32 / 1.125f32.powf((power - i) as f32))
        .collect::<Vec<_>>();
    let rises = simplex_noise(power, 1987, &weights)
        .normalize()
        .map(|_, z| (0.5 - z).abs() / 0.5);

    heightmap_from_rises_with_valleys(
        &rises,
        ValleyParameters {
            height_threshold: 0.25,
            rain_threshold: 128,
            rise: 0.01,
            origin_fn: |xy| rises.is_border(xy),
        },
    )
}
