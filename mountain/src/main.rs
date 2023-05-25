mod draw;
mod handlers;
mod init;
mod model;
mod network;
mod physics;
mod systems;

use std::collections::{HashMap, HashSet};
use std::f32::consts::PI;
use std::time::{Duration, Instant};

use commons::color::Rgba;
use commons::geometry::{xy, xyz, PositionedRectangle, Rectangle, XY, XYZ};

use commons::grid::Grid;
use commons::unsafe_ordering::unsafe_ordering;
use engine::engine::Engine;
use engine::events::{ButtonState, Event, EventHandler, KeyboardKey};
use engine::glium_backend;

use engine::graphics::projections::isometric;
use engine::graphics::Graphics;
use engine::handlers::{drag, resize, yaw, zoom};

use crate::handlers::selection::{self, Handler};
use crate::init::generate_heightmap;
use crate::model::skiing::State;
use crate::model::{skiing, Direction, Frame, DIRECTIONS};
use crate::network::skiing::{SkiingInNetwork, SkiingNetwork};
use crate::systems::selection_artist::SelectionArtist;
use crate::systems::{avatar_artist, framer, planner};

use ::network::algorithms::costs_to_target::CostsToTarget;

fn main() {
    let terrain = generate_heightmap();
    let engine = glium_backend::engine::GliumEngine::new(
        Game {
            components: Components {
                plans: HashMap::default(),
                frames: HashMap::default(),
                drawings: HashMap::default(),
                reserved: Grid::default(terrain.width(), terrain.height()),
                terrain,
            },
            drawings: None,
            handlers: Handlers {
                selection: selection::Handler {
                    origin: None,
                    key: KeyboardKey::X,
                },
            },
            systems: Systems {
                selection_artist: SelectionArtist {
                    drawn_selection: None,
                    selection_color: Rgba::new(255, 255, 0, 128),
                },
            },
            start: Instant::now(),
            mouse_xy: None,
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
            selection: None,
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
                    z_max: 1.0 / 128.0,
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

struct Game {
    components: Components,
    drawings: Option<Drawings>,
    handlers: Handlers,
    systems: Systems,
    start: Instant,
    selection: Option<PositionedRectangle<u32>>,
    mouse_xy: Option<XY<u32>>,
    drag_handler: drag::Handler,
    resize_handler: resize::Handler,
    yaw_handler: yaw::Handler,
    zoom_handler: zoom::Handler,
}

struct Components {
    plans: HashMap<usize, skiing::Plan>,
    frames: HashMap<usize, Frame>,
    drawings: HashMap<usize, usize>,
    terrain: Grid<f32>,
    reserved: Grid<bool>,
}

struct Drawings {
    terrain: draw::terrain::Drawing,
}

struct Handlers {
    selection: Handler,
}

struct Systems {
    selection_artist: SelectionArtist,
}

impl Game {
    fn init(&mut self, graphics: &mut dyn Graphics) {
        let terrain = &self.components.terrain;
        self.drawings = Some(Drawings {
            terrain: draw::terrain::draw(graphics, terrain),
        });

        graphics.look_at(
            &xyz(
                terrain.width() as f32 / 2.0,
                terrain.height() as f32 / 2.0,
                0.0,
            ),
            &xy(256, 256),
        );
    }

    fn add_skier(&mut self, graphics: &mut dyn Graphics) {
        let Some(mouse_xy) = self.mouse_xy else {return};
        let Ok(XYZ { x, y, .. }) = graphics.world_xyz_at(&mouse_xy) else {return};

        self.components.plans.insert(
            self.components.plans.len(),
            skiing::Plan::Stationary(skiing::State {
                position: xy(x.round() as u32, y.round() as u32),
                velocity: 0,
                travel_direction: model::Direction::NorthEast,
            }),
        );
    }
}

impl EventHandler for Game {
    fn handle(&mut self, event: &Event, engine: &mut dyn Engine, graphics: &mut dyn Graphics) {
        match event {
            Event::Init => self.init(graphics),
            Event::MouseMoved(xy) => self.mouse_xy = Some(*xy),
            Event::KeyboardInput {
                key: KeyboardKey::F,
                state: ButtonState::Pressed,
            } => self.add_skier(graphics),
            Event::KeyboardInput {
                key: KeyboardKey::C,
                state: ButtonState::Pressed,
            } => {
                if let Some(selection) = self.selection {
                    let skiing_network = SkiingNetwork {
                        terrain: &self.components.terrain,
                        reserved: &Grid::default(
                            self.components.terrain.width(),
                            self.components.terrain.height(),
                        ),
                    };
                    println!("Getting positions");
                    let mut positions = Vec::with_capacity(
                        (selection.width() * selection.height()).try_into().unwrap(),
                    );
                    for x in 0..selection.width() {
                        for y in 0..selection.height() {
                            positions.push(xy(selection.from.x + x, selection.from.y + y));
                        }
                    }
                    println!("Getting lowest position");
                    let lowest_position = positions
                        .iter()
                        .min_by(|a, b| {
                            unsafe_ordering(
                                &self.components.terrain[*a],
                                &self.components.terrain[*b],
                            )
                        })
                        .unwrap();
                    println!("Lowest position is {:?}", lowest_position);
                    let lowest_states = DIRECTIONS
                        .iter()
                        .map(|travel_direction| State {
                            position: *lowest_position,
                            velocity: 2,
                            travel_direction: *travel_direction,
                        })
                        .collect::<HashSet<_>>();

                    println!("Computing in network");
                    let in_network = SkiingInNetwork::for_positions(&skiing_network, &positions);
                    println!("Computing costs to target");
                    let result = in_network.costs_to_target(&lowest_states);
                    println!("Done = {:?}", result);
                }
            }
            _ => (),
        }

        planner::run(
            &self.components.terrain,
            &self.start.elapsed().as_micros(),
            &mut self.components.plans,
            &mut self.components.reserved,
        );
        framer::run(
            &self.components.terrain,
            &self.start.elapsed().as_micros(),
            &self.components.plans,
            &mut self.components.frames,
        );
        avatar_artist::run(
            graphics,
            &self.components.frames,
            &mut self.components.drawings,
        );
        self.systems.selection_artist.run(
            graphics,
            self.drawings.as_ref().map(|drawings| &drawings.terrain),
            &self.selection,
        );

        self.drag_handler.handle(event, engine, graphics);
        self.resize_handler.handle(event, engine, graphics);
        self.yaw_handler.handle(event, engine, graphics);
        self.zoom_handler.handle(event, engine, graphics);
        self.handlers
            .selection
            .handle(event, &self.mouse_xy, &mut self.selection, graphics);
    }
}
