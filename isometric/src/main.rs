mod game;
mod glium_backend;
mod graphics;

use std::time::Duration;

use commons::color::Color;
use commons::grid::Grid;
use commons::noise::simplex_noise;
use terrain_gen::with_valleys::{heightmap_from_rises_with_valleys, ValleyParameters};

use crate::game::Game;
use crate::glium_backend::engine::{self, Engine};
use crate::glium_backend::graphics::Graphics;
use crate::graphics::elements::Triangle;
use crate::graphics::GraphicsBackend;

fn main() {
    let engine = Engine::new(engine::Parameters {
        frame_duration: Duration::from_nanos(16_666_667),
    });
    let mut graphics = Graphics::with_engine(
        glium_backend::graphics::Parameters {
            name: "Demo".to_string(),
            width: 1024.0,
            height: 768.0,
        },
        &engine,
    );
    let terrain = get_heightmap();

    let mut triangles =
        Vec::with_capacity((terrain.width() - 1) as usize * (terrain.height() - 1) as usize * 2);
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
            triangles.push(Triangle {
                id,
                corners: [corners[0], corners[2], corners[1]],
                color: Color::rgb(z, z, z),
            });
            triangles.push(Triangle {
                id,
                corners: [corners[0], corners[3], corners[2]],
                color: Color::rgb(z, z, z),
            });
        }
    }

    graphics.add_primitive(&triangles);

    engine.run(DoNothing {}, graphics);
}

struct DoNothing {}

impl Game for DoNothing {
    fn update(&mut self, _: &mut dyn GraphicsBackend) {}
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
