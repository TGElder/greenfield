use std::f32::consts::PI;
use std::time::Duration;

use commons::color::Color;
use commons::grid::Grid;
use commons::noise::simplex_noise;
use isometric::game::{self, Game};
use isometric::glium_backend::engine::{self, Engine};
use isometric::glium_backend::graphics::Graphics;
use isometric::graphics::elements::Quad;
use isometric::graphics::projections::isometric_projection;
use isometric::graphics::GraphicsBackend;
use terrain_gen::with_valleys::{heightmap_from_rises_with_valleys, ValleyParameters};

fn main() {
    let engine = Engine::new(engine::Parameters {
        frame_duration: Duration::from_nanos(16_666_667),
    });
    let mut graphics = Graphics::with_engine(
        isometric::glium_backend::graphics::Parameters {
            name: "Demo".to_string(),
            width: 1024,
            height: 768,
            projection: Box::new(isometric_projection::Projection::new(
                isometric_projection::Parameters {
                    pitch: PI / 4.0,
                    yaw: PI * (5.0 / 8.0),
                    scale: 1.0 / 256.0,
                },
            )),
        },
        &engine,
    );
    let terrain = get_heightmap();

    let mut quads =
        Vec::with_capacity((terrain.width() - 1) as usize * (terrain.height() - 1) as usize);
    for x in 0..terrain.width() - 1 {
        for y in 0..terrain.height() - 1 {
            let id = terrain.index((x, y)) as u32;
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
                id,
                corners: [corners[0], corners[1], corners[2], corners[3]],
                color: Color::rgb(z, z, z),
            });
        }
    }

    graphics.add_quads(&quads);

    engine.run(DoNothing { screenshot: 0 }, graphics);
}

struct DoNothing {
    screenshot: u64,
}

impl Game for DoNothing {
    fn update(&mut self, graphics: &mut dyn GraphicsBackend) -> game::State {
        if self.screenshot == 1 {
            graphics.screenshot("screenshot.png");
        }
        self.screenshot += 1;
        game::State::Running
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
