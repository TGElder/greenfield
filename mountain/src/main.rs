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

use ::network::algorithms::find_min::MinPath;
use engine::graphics::projections::isometric;
use engine::graphics::Graphics;
use engine::handlers::{drag, resize, yaw, zoom};

use crate::draw::{draw_avatar, draw_terrain};
use crate::init::generate_heightmap;
use crate::model::{Avatar, Frame, State, DIRECTIONS};
use crate::network::{get_t_v_a, min_time, TerrainNetwork};

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
    avatars: Vec<Avatar>,
    avatar_indices: Vec<usize>,
    reservation: Vec<HashSet<XY<u32>>>,
    terrain: Grid<f32>,
    from: Vec<network::State>,
    reserved: HashSet<XY<u32>>,
}

impl EventHandler for Game {
    fn handle(&mut self, event: &Event, engine: &mut dyn Engine, graphics: &mut dyn Graphics) {
        match event {
            Event::Init => self.init(graphics),
            Event::MouseMoved(xy) => self.mouse_xy = Some(*xy),
            Event::KeyboardInput {
                key: KeyboardKey::F,
                state: ButtonState::Pressed,
            } => self.add_avatar(graphics),
            _ => (),
        }

        if let Some(state) = &mut self.state {
            for i in 0..state.avatars.len() {
                let avatar = &state.avatars[i];
                if let Avatar::Moving(frames) = avatar {
                    if let Some(frame) = frames.last() {
                        let start = self.start.elapsed().as_micros();
                        if frame.arrival_micros as u128 <= start {
                            for reserved in &state.reservation[i] {
                                state.reserved.remove(reserved);
                            }
                            state.reservation[i].clear();
                            Self::set_to(state, &self.start, i);
                        }
                    }
                }
            }
        }

        if let Some(state) = &mut self.state {
            for i in 0..state.avatars.len() {
                let avatar = &state.avatars[i];
                if let Avatar::Moving(frames) = avatar {
                    if frames.is_empty() {
                        for reserved in &state.reservation[i] {
                            state.reserved.remove(reserved);
                        }
                        state.reservation[i].clear();
                        Self::set_to(state, &self.start, i);
                    }
                }
            }
        }

        if let Some(state) = &self.state {
            for (avatar, avatar_index) in state.avatars.iter().zip(&state.avatar_indices) {
                draw_avatar(
                    avatar,
                    &(self.start.elapsed().as_micros() as u64),
                    graphics,
                    avatar_index,
                );
            }
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

        draw_terrain(&terrain, graphics);

        graphics.look_at(
            &xyz(
                terrain.width() as f32 / 2.0,
                terrain.height() as f32 / 2.0,
                0.0,
            ),
            &xy(256, 256),
        );

        self.state = Some(GameState {
            avatars: vec![],
            avatar_indices: vec![],
            reservation: vec![],
            terrain,
            from: vec![],
            reserved: HashSet::new(),
        });
    }

    fn add_avatar(&mut self, graphics: &mut dyn Graphics) {
        let Some(state) = &mut self.state else {return};
        let Some(mouse) = self.mouse_xy else {return};
        let Ok(world) = graphics.world_xyz_at(&mouse) else {return};
        let avatar = Avatar::_Static(State {
            position: xyz(0.0, 0.0, 0.0),
            angle: 0.0,
        });
        state.avatars.push(avatar);
        state.from.push(network::State {
            position: xy(world.x.round() as u32, world.y.round() as u32),
            travel_direction: model::Direction::North,
            body_direction: model::Direction::North,
            velocity: 0,
        });
        let avatar_index = graphics.create_quads().unwrap();
        state.avatar_indices.push(avatar_index);
        state.reservation.push(HashSet::new());
        println!("Added avatar {}", state.avatar_indices.len() - 1);
        Self::set_to(state, &self.start, state.avatar_indices.len() - 1);
    }

    fn set_to(state: &mut GameState, start: &Instant, index: usize) {
        println!("Finding path for avatar {}", index);
        let from = &mut state.from[index];

        let network = TerrainNetwork::new(&state.terrain, &state.reserved);
        let path = network.min_path(*from, 16, &|state, network| network.terrain[state.position]);
        let Some(path) = path else {
            println!("No path for avatar {} @ {:?}", index, from);
            state.avatars[index] = Avatar::Moving(vec![]);
            from.velocity = 0;
            return
        };
        if path.is_empty() {
            println!("No path for avatar {} @ {:?}", index,  from);
            state.avatars[index] = Avatar::Moving(vec![]);
            from.velocity = 0;
            return;
        }

        let mut start = start.elapsed().as_micros();
        let mut frames = Vec::with_capacity(path.len());

        for edge in path.iter() {
            let x = edge.from.position.x as f32;
            let y = edge.from.position.y as f32;
            let z = state.terrain[edge.from.position];
            frames.push(Frame {
                arrival_micros: start as u64,
                state: State {
                    position: xyz(x, y, z),
                    angle: edge.to.travel_direction.angle(),
                },
            });
            state.reserved.insert(edge.from.position);
            state.reservation[index].insert(edge.from.position);
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
                    angle: last.to.body_direction.angle(),
                },
            });
            state.reserved.insert(last.to.position);
            state.reservation[index].insert(last.to.position);
        }

        println!("Found path for avatar {}", index);

        state.from[index] = path.last().map(|edge| edge.to).unwrap();
        state.avatars[index] = Avatar::Moving(frames);
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
