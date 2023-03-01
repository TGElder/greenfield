use std::f32::consts::PI;
use std::time::Duration;

use commons::color::Rgb;
use commons::geometry::{xy, xyz, Rectangle, XYZ};
use commons::grid::Grid;
use commons::noise::simplex_noise;
use engine::engine::Engine;
use engine::events::{ButtonState, Event, EventHandler, KeyboardKey};
use engine::glium_backend;
use engine::graphics::elements::Quad;
use engine::graphics::projections::isometric;
use engine::graphics::Graphics;
use engine::handlers::{drag, resize, yaw, zoom};
use nalgebra::Vector3;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use terrain_gen::heightmap_from_rises;

fn main() {
    let engine = glium_backend::engine::GliumEngine::new(
        Demo {
            drag_handler: drag::Handler::new(),
            resize_handler: resize::Handler::new(),
            yaw_handler: yaw::Handler::new(yaw::Parameters {
                initial_angle: 5,
                angles: 16,
                key_plus: KeyboardKey::E,
                key_minus: KeyboardKey::Q,
            }),
            zoom_handler: zoom::Handler::new(zoom::Parameters {
                initial_level: -1,
                min_level: -2,
                max_level: 8,
                key_plus: KeyboardKey::Plus,
                key_minus: KeyboardKey::Minus,
            }),
            pool_size: 16,
            candidate: None,
            candidates: first_generation(16),
            evaluated: Vec::with_capacity(16),
            seed: 1986,
        },
        glium_backend::engine::Parameters {
            frame_duration: Duration::from_nanos(16_666_667),
        },
        glium_backend::graphics::Parameters {
            name: "Demo".to_string(),
            width: 512,
            height: 512,
            projection: Box::new(isometric::Projection::new(isometric::Parameters {
                projection: isometric::ProjectionParameters {
                    pitch: PI / 4.0,
                    yaw: PI * (5.0 / 8.0),
                },
                scale: isometric::ScaleParameters {
                    zoom: 0.5,
                    z_max: 1.0 / 512.0,
                    viewport: Rectangle {
                        width: 512,
                        height: 512,
                    },
                },
            })),
        },
    )
    .unwrap();

    engine.run();
}

struct Demo {
    drag_handler: drag::Handler,
    resize_handler: resize::Handler,
    yaw_handler: yaw::Handler,
    zoom_handler: zoom::Handler,
    pool_size: usize,
    candidates: Vec<Candidate>,
    candidate: Option<Candidate>,
    evaluated: Vec<Candidate>,
    seed: i32,
}

impl EventHandler for Demo {
    fn handle(&mut self, event: &Event, engine: &mut dyn Engine, graphics: &mut dyn Graphics) {
        if self.candidate.is_none() {
            if self.candidates.is_empty() {
                println!("New generation");
                self.candidates = next_generation(&self.evaluated, self.pool_size);
                self.evaluated.clear();
            }
            println!("{} left to evaluate", self.candidates.len());
            let candidate = self.candidates.pop().unwrap();
            println!("Evaluating {:?}", candidate.weights);

            let terrain = get_heightmap(&candidate.weights, self.seed);
            self.candidate = Some(candidate);

            let mut quads = Vec::with_capacity(
                (terrain.width() - 1) as usize * (terrain.height() - 1) as usize,
            );
            for x in 0..terrain.width() - 1 {
                for y in 0..terrain.height() - 1 {
                    let corners = [xy(0, 0), xy(1, 0), xy(1, 1), xy(0, 1)]
                        .iter()
                        .map(|d| {
                            xyz(
                                (x + d.x) as f32,
                                (y + d.y) as f32,
                                terrain[xy(x + d.x, y + d.y)] * 512.0,
                            )
                        })
                        .collect::<Vec<_>>();

                    quads.push(Quad {
                        color: color(&corners),
                        corners: [corners[0], corners[1], corners[2], corners[3]],
                    });
                }
            }

            graphics.add_quads(&quads).unwrap();
            graphics.look_at(
                &xyz(
                    terrain.width() as f32 / 2.0,
                    terrain.height() as f32 / 2.0,
                    0.0,
                ),
                &xy(640, 720),
            );
        }

        self.drag_handler.handle(event, engine, graphics);
        self.resize_handler.handle(event, engine, graphics);
        self.yaw_handler.handle(event, engine, graphics);
        self.zoom_handler.handle(event, engine, graphics);

        let score = if self.candidate.is_some() {
            match event {
                Event::KeyboardInput {
                    key,
                    state: ButtonState::Pressed,
                } => match key {
                    KeyboardKey::Key0 => 0.0,
                    KeyboardKey::Key1 => 0.1,
                    KeyboardKey::Key2 => 0.2,
                    KeyboardKey::Key3 => 0.3,
                    KeyboardKey::Key4 => 0.4,
                    KeyboardKey::Key5 => 0.5,
                    KeyboardKey::Key6 => 0.6,
                    KeyboardKey::Key7 => 0.7,
                    KeyboardKey::Key8 => 0.8,
                    KeyboardKey::Key9 => 0.9,
                    _ => return,
                },
                _ => return,
            }
        } else {
            return;
        };

        let mut candidate = self.candidate.take().unwrap();
        candidate.strength = score;
        self.evaluated.push(candidate);
        self.seed = thread_rng().gen();
        graphics.clear();
    }
}

struct Candidate {
    weights: Vec<f32>,
    strength: f32,
}

fn first_generation(count: usize) -> Vec<Candidate> {
    // let power = 11;
    // let mut rng = thread_rng();
    let mut result = Vec::with_capacity(count);
    for _ in 0..count {
        let weights = vec![
            0.4014246, 0.78097993, 0.41403908, 1.2097175, 1.5679939, 1.0129398, 3.3901482,
            0.08857876, 5.661517, 1.7988696, 6.4280505, 5.25,
        ];
        result.push(Candidate {
            weights,
            strength: 1.0,
        });
    }
    next_generation(&result, count)
}

fn next_generation(candidates: &[Candidate], count: usize) -> Vec<Candidate> {
    let mut rng = thread_rng();
    let mut result = Vec::with_capacity(count);
    for _ in 0..count {
        let a = candidates
            .choose_weighted(&mut rng, |candidate| candidate.strength)
            .unwrap();
        let b = candidates
            .choose_weighted(&mut rng, |candidate| candidate.strength)
            .unwrap();

        let weight_count = a.weights.len();
        let mut weights = Vec::with_capacity(weight_count);
        for i in 0..weight_count {
            if rng.gen::<f32>() <= 0.1 {
                // mutate
                weights.push(rng.gen());
            } else if rng.gen::<f32>() <= 0.25 {
                // merge
                weights.push(a.weights[i] + b.weights[i] / 2.0);
            } else if rng.gen::<f32>() <= 0.5 {
                // copy
                weights.push(a.weights[i])
            } else {
                weights.push(b.weights[i])
            }
        }
        result.push(Candidate {
            weights,
            strength: 0.0,
        })
    }
    result
}

fn get_heightmap(weights: &[f32], seed: i32) -> Grid<f32> {
    let power = 11;
    let rises = simplex_noise(power, seed, weights)
        .normalize()
        .map(|_, z| (0.5 - z).abs() / 0.5);

    heightmap_from_rises(&rises, |xy| xy.x == 0)
}

fn color(corners: &[XYZ<f32>]) -> Rgb<f32> {
    let light_direction: Vector3<f32> = Vector3::new(1.0, 0.0, 0.0);
    let base_color: Rgb<f32> = Rgb::new(1.0, 1.0, 1.0);

    let corners = corners
        .iter()
        .map(|XYZ { x, y, z }| Vector3::new(*x, *y, *z))
        .collect::<Vec<_>>();

    let u = corners[0] - corners[2];
    let v = corners[1] - corners[3];
    let normal = u.cross(&v);
    let angle = normal.angle(&light_direction);
    let shade = angle / PI;
    Rgb::new(
        base_color.r * shade,
        base_color.g * shade,
        base_color.b * shade,
    )
}
