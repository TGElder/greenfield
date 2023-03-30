mod draw;
mod init;
mod model;
mod network;

use std::collections::HashSet;
use std::f32::consts::PI;
use std::time::{Duration, Instant};

use commons::geometry::{xy, xyz, Rectangle, XY};

use commons::grid::Grid;
use engine::engine::Engine;
use engine::events::{ButtonState, Event, EventHandler, KeyboardKey};
use engine::glium_backend;

use ::network::algorithms::find_path::FindPath;
use engine::graphics::projections::isometric;
use engine::graphics::Graphics;
use engine::handlers::{drag, resize, yaw, zoom};

use crate::draw::{draw_avatar, draw_terrain};
use crate::init::generate_heightmap;
use crate::model::{Avatar, Frame, State, DIRECTIONS};
use crate::network::TerrainNetwork;

struct Game {
    state: Option<GameState>,
    start: Instant,
    drag_handler: drag::Handler,
    resize_handler: resize::Handler,
    yaw_handler: yaw::Handler,
    zoom_handler: zoom::Handler,
    mouse_xy: Option<XY<u32>>,
}

struct GameState {
    avatar: Avatar,
    avatar_index: usize,
    terrain: Grid<f32>,
    from: Option<HashSet<network::State>>,
}

impl EventHandler for Game {
    fn handle(&mut self, event: &Event, engine: &mut dyn Engine, graphics: &mut dyn Graphics) {
        match event {
            Event::Init => self.init(graphics),
            Event::MouseMoved(xy) => self.mouse_xy = Some(*xy),
            Event::KeyboardInput {
                key: KeyboardKey::F,
                state: ButtonState::Pressed,
            } => self.set_from(graphics),
            Event::KeyboardInput {
                key: KeyboardKey::T,
                state: ButtonState::Pressed,
            } => self.set_to(graphics),
            _ => (),
        }

        if let Some(GameState {
            avatar,
            avatar_index,
            ..
        }) = &self.state
        {
            draw_avatar(
                avatar,
                &(self.start.elapsed().as_micros() as u64),
                graphics,
                avatar_index,
            );
        };

        self.drag_handler.handle(event, engine, graphics);
        self.resize_handler.handle(event, engine, graphics);
        self.yaw_handler.handle(event, engine, graphics);
        self.zoom_handler.handle(event, engine, graphics);
    }
}

impl Game {
    fn init(&mut self, graphics: &mut dyn Graphics) {
        let terrain = generate_heightmap();
        let avatar = Avatar::Moving(vec![
            Frame {
                arrival_micros: 0,
                state: State {
                    position: xyz(256.0, 256.0, terrain[xy(256, 256)]),
                    angle: PI * (1.0 / 16.0),
                },
            },
            Frame {
                arrival_micros: 60_000_000,
                state: State {
                    position: xyz(257.0, 256.0, terrain[xy(257, 256)]),
                    angle: PI * (1.0 / 16.0),
                },
            },
        ]);

        draw_terrain(&terrain, graphics);
        let avatar_index = graphics.create_quads().unwrap();

        graphics.look_at(
            &xyz(
                terrain.width() as f32 / 2.0,
                terrain.height() as f32 / 2.0,
                0.0,
            ),
            &xy(256, 256),
        );

        self.state = Some(GameState {
            avatar,
            avatar_index,
            terrain,
            from: None,
        });
    }

    fn set_from(&mut self, graphics: &mut dyn Graphics) {
        let Some(state) = &mut self.state else {return};
        let Some(mouse) = self.mouse_xy else {return};
        let Ok(world) = graphics.world_xyz_at(&mouse) else {return};
        state.from = Some(
            DIRECTIONS
                .iter()
                .map(|direction| network::State {
                    position: xy(world.x.round() as u32, world.y.round() as u32),
                    direction: *direction,
                    velocity: 0,
                })
                .collect(),
        );
    }

    fn set_to(&mut self, graphics: &mut dyn Graphics) {
        let Some(state) = &mut self.state else {return};
        let Some(from) = &state.from else {return};
        let Some(mouse) = self.mouse_xy else {return};
        let Ok(world) = graphics.world_xyz_at(&mouse) else {return};
        let to_position = xy(world.x.round() as u32, world.y.round() as u32);
        let to = DIRECTIONS
            .iter()
            .flat_map(|direction| {
                (0..8).map(|velocity| network::State {
                    position: to_position,
                    direction: *direction,
                    velocity: velocity as u8,
                })
            })
            .collect::<HashSet<_>>();

        let network = TerrainNetwork::new(&state.terrain);
        let path = network.find_path(from.clone(), to, &|state| {
            (to_position.x.abs_diff(state.position.x) as u64
                + to_position.y.abs_diff(state.position.y) as u64)
                * 45454
        });
        let Some(path) = path else {return};
        let mut start = self.start.elapsed().as_micros();
        let mut frames = Vec::with_capacity(path.len());

        for edge in path.iter() {
            let x = edge.from.position.x as f32;
            let y = edge.from.position.y as f32;
            let z = state.terrain[edge.from.position];
            frames.push(Frame {
                arrival_micros: start as u64,
                state: State {
                    position: xyz(x, y, z),
                    angle: edge.to.direction.angle(),
                },
            });
            start += edge.cost as u128
        }

        if let Some(last) = path.last() {
            let x = last.to.position.x as f32;
            let y = last.to.position.y as f32;
            let z = state.terrain[last.to.position];
            frames.push(Frame {
                arrival_micros: start as u64,
                state: State {
                    position: xyz(x, y, z),
                    angle: last.to.direction.angle(),
                },
            });
        }

        state.avatar = Avatar::Moving(frames);

        dbg!(&state.avatar);
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
            state: None,
            start: Instant::now(),
            mouse_xy: None,
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
