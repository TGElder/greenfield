mod draw;
mod init;
mod model;
mod network;
mod physics;
mod systems;

use std::collections::HashMap;
use std::f32::consts::PI;
use std::time::{Duration, Instant};

use commons::geometry::{xy, xyz, Rectangle};

use commons::grid::Grid;
use engine::engine::Engine;
use engine::events::{Event, EventHandler, KeyboardKey};
use engine::glium_backend;

use engine::graphics::projections::isometric;
use engine::graphics::Graphics;
use engine::handlers::{drag, resize, yaw, zoom};
use maplit::hashmap;

use crate::draw::draw_terrain;
use crate::init::generate_heightmap;
use crate::model::{Behavior, Frame};
use crate::network::skiing;
use crate::systems::{avatar_artist, framer};

struct Game {
    components: Option<Components>, // Avoid None
    start: Instant,
    drag_handler: drag::Handler,
    resize_handler: resize::Handler,
    yaw_handler: yaw::Handler,
    zoom_handler: zoom::Handler,
}

struct Components {
    terrain: Grid<f32>,
    behaviors: HashMap<usize, Behavior>,
    frames: HashMap<usize, Frame>,
    drawings: HashMap<usize, usize>,
}

impl EventHandler for Game {
    fn handle(&mut self, event: &Event, engine: &mut dyn Engine, graphics: &mut dyn Graphics) {
        if let Event::Init = *event {
            let terrain = generate_heightmap();

            draw_terrain(&terrain, graphics);

            graphics.look_at(
                &xyz(
                    terrain.width() as f32 / 2.0,
                    terrain.height() as f32 / 2.0,
                    0.0,
                ),
                &xy(256, 256),
            );

            self.components = Some(Components {
                terrain,
                behaviors: hashmap! {
                    0 => Behavior::Moving(vec![
                        model::Event{ micros: 0, state: skiing::State { position: xy(256, 256), velocity: 0, travel_direction: model::Direction::NorthEast } },
                        model::Event{ micros: 60_000_000, state: skiing::State { position: xy(257, 257), velocity: 0, travel_direction: model::Direction::NorthEast } },
                    ]),
                },
                frames: HashMap::default(),
                drawings: HashMap::default(),
            });
        }

        if let Some(components) = &mut self.components {
            framer::run(
                &components.terrain,
                &self.start.elapsed().as_micros(),
                &components.behaviors,
                &mut components.frames,
            );
            avatar_artist::run(graphics, &components.frames, &mut components.drawings);
        }

        self.drag_handler.handle(event, engine, graphics);
        self.resize_handler.handle(event, engine, graphics);
        self.yaw_handler.handle(event, engine, graphics);
        self.zoom_handler.handle(event, engine, graphics);
    }
}

fn main() {
    let engine = glium_backend::engine::GliumEngine::new(
        Game {
            drag_handler: drag::Handler::new(),
            resize_handler: resize::Handler::new(),
            yaw_handler: yaw::Handler::new(yaw::Parameters {
                initial_angle: 5,
                angles: 16,
                key_plus: KeyboardKey::E,
                key_minus: KeyboardKey::Q,
            }),
            zoom_handler: zoom::Handler::new(zoom::Parameters {
                initial_level: 1,
                min_level: 1,
                max_level: 8,
                key_plus: KeyboardKey::Plus,
                key_minus: KeyboardKey::Minus,
            }),
            components: None,
            start: Instant::now(),
        },
        glium_backend::engine::Parameters {
            frame_duration: Duration::from_nanos(16_666_667),
        },
        glium_backend::graphics::Parameters {
            name: "The Mountain".to_string(),
            width: 512,
            height: 512,
            projection: Box::new(isometric::Projection::new(isometric::Parameters {
                projection: isometric::ProjectionParameters {
                    pitch: PI / 4.0,
                    yaw: PI * (5.0 / 8.0),
                },
                scale: isometric::ScaleParameters {
                    zoom: 2.0,
                    z_max: 1.0 / 32.0,
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
