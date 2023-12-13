mod draw;
mod handlers;
mod init;
mod model;
mod network;
mod services;
mod systems;
mod utils;

use std::collections::{HashMap, HashSet};
use std::f32::consts::PI;
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

use commons::color::Rgba;
use commons::geometry::{xy, xyz, Rectangle, XYRectangle, XY};

use commons::grid::Grid;
use engine::binding::Binding;
use engine::engine::Engine;
use engine::events::{Button, ButtonState, Event, EventHandler, KeyboardKey, MouseButton};
use engine::glium_backend;

use engine::graphics::projections::isometric;
use engine::graphics::Graphics;
use engine::handlers::{drag, resize, yaw, zoom};
use serde::{Deserialize, Serialize};

use crate::handlers::{
    add_skier, entrance_builder, entrance_opener, lift_opener, lift_remover, piste_builder, save,
};
use crate::handlers::{lift_builder, selection};
use crate::init::generate_heightmap;
use crate::model::carousel::{Car, Carousel};
use crate::model::entrance::Entrance;
use crate::model::exit::Exit;
use crate::model::frame::Frame;
use crate::model::lift::Lift;
use crate::model::piste::{Piste, PisteCosts};
use crate::model::skiing;
use crate::services::id_allocator;
use crate::systems::{
    carousel, chair_framer, distance_cost_computer, entrance, entrance_artist, exit_computer,
    frame_wiper, lift_artist, model_artist, overlay, piste_adopter, planner, skiing_cost_computer,
    skiing_framer, target_scrubber, target_setter,
};

fn main() {
    let components = get_components();

    let engine = glium_backend::engine::GliumEngine::new(
        Game {
            drawings: None,
            handlers: Handlers {
                add_skier: add_skier::Handler {
                    binding: Binding::Single {
                        button: Button::Keyboard(KeyboardKey::F),
                        state: ButtonState::Pressed,
                    },
                },
                clock: handlers::clock::Handler::new(handlers::clock::Bindings {
                    slow_down: Binding::Single {
                        button: Button::Keyboard(KeyboardKey::Comma),
                        state: ButtonState::Pressed,
                    },
                    speed_up: Binding::Single {
                        button: Button::Keyboard(KeyboardKey::Period),
                        state: ButtonState::Pressed,
                    },
                }),
                drag: drag::Handler::new(drag::Bindings {
                    start_dragging: Binding::Single {
                        button: Button::Mouse(MouseButton::Left),
                        state: ButtonState::Pressed,
                    },
                    stop_dragging: Binding::Single {
                        button: Button::Mouse(MouseButton::Left),
                        state: ButtonState::Released,
                    },
                }),
                piste_builder: piste_builder::Handler {
                    bindings: piste_builder::Bindings {
                        add: Binding::Single {
                            button: Button::Keyboard(KeyboardKey::V),
                            state: ButtonState::Pressed,
                        },
                        subtract: Binding::Single {
                            button: Button::Keyboard(KeyboardKey::X),
                            state: ButtonState::Pressed,
                        },
                    },
                },
                lift_builder: lift_builder::Handler::new(Binding::Single {
                    button: Button::Keyboard(KeyboardKey::L),
                    state: ButtonState::Pressed,
                }),
                lift_opener: lift_opener::Handler {
                    binding: Binding::Single {
                        button: Button::Keyboard(KeyboardKey::O),
                        state: ButtonState::Pressed,
                    },
                },
                lift_remover: lift_remover::Handler {
                    binding: Binding::Single {
                        button: Button::Keyboard(KeyboardKey::X),
                        state: ButtonState::Pressed,
                    },
                },
                entrance_builder: entrance_builder::Handler::new(Binding::Single {
                    button: Button::Keyboard(KeyboardKey::N),
                    state: ButtonState::Pressed,
                }),
                entrance_opener: entrance_opener::Handler {
                    binding: Binding::Single {
                        button: Button::Keyboard(KeyboardKey::O),
                        state: ButtonState::Pressed,
                    },
                },
                resize: resize::Handler::new(),
                save: save::Handler {
                    binding: Binding::Single {
                        button: Button::Keyboard(KeyboardKey::P),
                        state: ButtonState::Pressed,
                    },
                },
                selection: selection::Handler::new(Binding::Single {
                    button: Button::Mouse(MouseButton::Right),
                    state: ButtonState::Pressed,
                }),
                yaw: yaw::Handler::new(yaw::Parameters {
                    initial_angle: 5,
                    angles: 16,
                    bindings: yaw::Bindings {
                        plus: Binding::Single {
                            button: Button::Keyboard(KeyboardKey::E),
                            state: ButtonState::Pressed,
                        },
                        minus: Binding::Single {
                            button: Button::Keyboard(KeyboardKey::Q),
                            state: ButtonState::Pressed,
                        },
                    },
                }),
                zoom: zoom::Handler::new(zoom::Parameters {
                    initial_level: 1,
                    min_level: 1,
                    max_level: 8,
                    bindings: zoom::Bindings {
                        plus: Binding::Multi(vec![
                            Binding::Single {
                                button: Button::Keyboard(KeyboardKey::Plus),
                                state: ButtonState::Pressed,
                            },
                            Binding::Single {
                                button: Button::Mouse(MouseButton::WheelUp),
                                state: ButtonState::Pressed,
                            },
                        ]),
                        minus: Binding::Multi(vec![
                            Binding::Single {
                                button: Button::Keyboard(KeyboardKey::Minus),
                                state: ButtonState::Pressed,
                            },
                            Binding::Single {
                                button: Button::Mouse(MouseButton::WheelDown),
                                state: ButtonState::Pressed,
                            },
                        ]),
                    },
                }),
            },
            systems: Systems {
                overlay: overlay::System {
                    updates: vec![XYRectangle {
                        from: xy(0, 0),
                        to: xy(
                            components.terrain.width() - 2,
                            components.terrain.height() - 2,
                        ), // -2 because bottom right corner is width - 1, height - 1 and the overlay is on cells which also reduce each dimension by one
                    }],
                    colors: overlay::Colors {
                        selection: Rgba::new(255, 255, 0, 128),
                        piste: Rgba::new(0, 0, 255, 128),
                    },
                },
                planner: planner::System::new(),
                carousel: carousel::System::new(),
            },
            mouse_xy: None,
            components,
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

fn get_components() -> Components {
    if let Some(loaded_components) = load_components("default.save") {
        loaded_components
    } else {
        new_components()
    }
}

fn load_components(path: &str) -> Option<Components> {
    let file = File::open(path).ok()?;
    bincode::deserialize_from(BufReader::new(file)).ok()
}

fn new_components() -> Components {
    let terrain = generate_heightmap();
    Components {
        plans: HashMap::default(),
        locations: HashMap::default(),
        targets: HashMap::default(),
        frames: HashMap::default(),
        drawings: HashMap::default(),
        pistes: HashMap::default(),
        distance_costs: HashMap::default(),
        skiing_costs: HashMap::default(),
        lifts: HashMap::default(),
        carousels: HashMap::default(),
        cars: HashMap::default(),
        entrances: HashMap::default(),
        reserved: Grid::default(terrain.width(), terrain.height()),
        piste_map: Grid::default(terrain.width(), terrain.height()),
        exits: HashMap::default(),
        open: HashSet::default(),
        terrain,
        services: Services {
            clock: services::clock::Service::new(),
            id_allocator: id_allocator::Service::new(),
        },
    }
}

struct Game {
    components: Components,
    drawings: Option<Drawings>,
    handlers: Handlers,
    systems: Systems,
    mouse_xy: Option<XY<u32>>,
}

#[derive(Serialize, Deserialize)]
pub struct Components {
    plans: HashMap<usize, skiing::Plan>,
    locations: HashMap<usize, usize>,
    targets: HashMap<usize, usize>,
    #[serde(skip)]
    frames: HashMap<usize, Option<Frame>>,
    #[serde(skip)]
    drawings: HashMap<usize, usize>,
    pistes: HashMap<usize, Piste>,
    distance_costs: HashMap<usize, PisteCosts>,
    skiing_costs: HashMap<usize, PisteCosts>,
    lifts: HashMap<usize, Lift>,
    cars: HashMap<usize, Car>,
    carousels: HashMap<usize, Carousel>,
    entrances: HashMap<usize, Entrance>,
    exits: HashMap<usize, Vec<Exit>>,
    open: HashSet<usize>,
    terrain: Grid<f32>,
    reserved: Grid<bool>,
    piste_map: Grid<Option<usize>>,
    services: Services,
}

struct Drawings {
    terrain: draw::terrain::Drawing,
}

struct Handlers {
    add_skier: add_skier::Handler,
    clock: handlers::clock::Handler,
    drag: drag::Handler,
    entrance_builder: entrance_builder::Handler,
    entrance_opener: entrance_opener::Handler,
    piste_builder: piste_builder::Handler,
    resize: resize::Handler,
    lift_builder: lift_builder::Handler,
    lift_opener: lift_opener::Handler,
    lift_remover: lift_remover::Handler,
    save: save::Handler,
    selection: selection::Handler,
    yaw: yaw::Handler,
    zoom: zoom::Handler,
}

struct Systems {
    overlay: overlay::System,
    planner: planner::System,
    carousel: carousel::System,
}

#[derive(Serialize, Deserialize)]
pub struct Services {
    clock: services::clock::Service,
    id_allocator: id_allocator::Service,
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
}

impl EventHandler for Game {
    fn handle(&mut self, event: &Event, engine: &mut dyn Engine, graphics: &mut dyn Graphics) {
        match event {
            Event::Init => self.init(graphics),
            Event::MouseMoved(xy) => self.mouse_xy = Some(*xy),
            _ => (),
        }

        self.handlers.drag.handle(event, engine, graphics);
        self.handlers.resize.handle(event, engine, graphics);
        self.handlers.yaw.handle(event, engine, graphics);
        self.handlers.zoom.handle(event, engine, graphics);

        self.handlers.add_skier.handle(
            event,
            &self.mouse_xy,
            &mut self.components.plans,
            &mut self.components.services.id_allocator,
            graphics,
        );
        self.handlers
            .clock
            .handle(event, &mut self.components.services.clock);
        self.handlers.piste_builder.handle(
            event,
            &mut self.components.pistes,
            &mut self.components.piste_map,
            &mut self.handlers.selection,
            &mut self.systems.overlay,
            &mut self.components.services.id_allocator,
        );
        self.handlers
            .lift_builder
            .handle(handlers::lift_builder::Parameters {
                event,
                mouse_xy: &self.mouse_xy,
                terrain: &self.components.terrain,
                lifts: &mut self.components.lifts,
                open: &mut self.components.open,
                overlay: &mut self.systems.overlay,
                id_allocator: &mut self.components.services.id_allocator,
                carousels: &mut self.components.carousels,
                cars: &mut self.components.cars,
                graphics,
            });
        self.handlers.lift_remover.handle(
            event,
            &self.mouse_xy,
            &self.components.open,
            &self.components.locations,
            &self.components.targets,
            &mut self.components.lifts,
            &mut self.components.carousels,
            &mut self.components.cars,
            &mut self.components.distance_costs,
            &mut self.components.skiing_costs,
            &mut self.components.frames,
            &mut self.components.drawings,
            &mut self.components.exits,
            graphics,
        );

        self.handlers
            .entrance_builder
            .handle(handlers::entrance_builder::Parameters {
                event,
                piste_map: &self.components.piste_map,
                selection: &mut self.handlers.selection,
                overlay: &mut self.systems.overlay,
                id_allocator: &mut self.components.services.id_allocator,
                entrances: &mut self.components.entrances,
                open: &mut self.components.open,
            });
        self.handlers.lift_opener.handle(
            event,
            &self.mouse_xy,
            &self.components.lifts,
            &mut self.components.open,
            graphics,
        );
        self.handlers.entrance_opener.handle(
            event,
            &self.mouse_xy,
            &self.components.entrances,
            &mut self.components.open,
            graphics,
        );
        self.handlers.save.handle(event, &mut self.components);
        self.handlers
            .selection
            .handle(event, &self.mouse_xy, graphics, &mut self.systems.overlay);

        self.systems.carousel.run(systems::carousel::Parameters {
            micros: &self.components.services.clock.get_micros(),
            lifts: &self.components.lifts,
            open: &self.components.open,
            carousels: &self.components.carousels,
            reserved: &mut self.components.reserved,
            plans: &mut self.components.plans,
            locations: &mut self.components.locations,
            targets: &mut self.components.targets,
            cars: &mut self.components.cars,
        });

        target_scrubber::run(&self.components.open, &mut self.components.targets);
        piste_adopter::run(
            &self.components.plans,
            &self.components.piste_map,
            &mut self.components.locations,
        );
        target_setter::run(
            &self.components.terrain,
            &self.components.plans,
            &self.components.locations,
            &self.components.exits,
            &self.components.open,
            &mut self.components.targets,
        );

        entrance::run(
            &self.components.plans,
            &self.components.entrances,
            &self.components.open,
            &mut self.components.targets,
            &mut self.components.locations,
        );
        self.systems.planner.run(systems::planner::Parameters {
            terrain: &self.components.terrain,
            micros: &self.components.services.clock.get_micros(),
            plans: &mut self.components.plans,
            locations: &self.components.locations,
            targets: &self.components.targets,
            pistes: &self.components.pistes,
            distance_costs: &self.components.distance_costs,
            skiing_costs: &self.components.skiing_costs,
            reserved: &mut self.components.reserved,
        });
        frame_wiper::run(&mut self.components.frames);
        skiing_framer::run(
            &self.components.terrain,
            &self.components.services.clock.get_micros(),
            &self.components.plans,
            &mut self.components.frames,
        );
        chair_framer::run(
            &self.components.carousels,
            &self.components.lifts,
            &self.components.cars,
            &self.components.locations,
            &mut self.components.frames,
        );
        model_artist::run(
            graphics,
            &self.components.frames,
            &mut self.components.drawings,
        );
        lift_artist::run(
            graphics,
            &self.components.lifts,
            &mut self.components.drawings,
        );
        entrance_artist::run(
            graphics,
            &self.components.entrances,
            &self.components.terrain,
            &self.components.piste_map,
            &mut self.components.drawings,
        );

        self.systems.overlay.run(
            graphics,
            self.drawings.as_ref().map(|drawings| &drawings.terrain),
            &self.components.piste_map,
            &self.handlers.selection,
        );

        const COMPUTE_COSTS_BINDING: Binding = Binding::Single {
            button: Button::Keyboard(KeyboardKey::C),
            state: ButtonState::Pressed,
        };

        if COMPUTE_COSTS_BINDING.binds_event(event) {
            exit_computer::run(
                &self.components.pistes,
                &self.components.lifts,
                &self.components.entrances,
                &mut self.components.exits,
            );
            distance_cost_computer::run(
                &self.components.terrain,
                &self.components.pistes,
                &self.components.exits,
                &mut self.components.distance_costs,
            );
            skiing_cost_computer::run(
                &self.components.terrain,
                &self.components.pistes,
                &self.components.exits,
                &self.components.distance_costs,
                &mut self.components.skiing_costs,
            );
        }
    }
}
