#[macro_use]
extern crate lazy_static;

mod draw;
mod gui;
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

use commons::color::{Rgb, Rgba};
use commons::geometry::{xy, xyz, Rectangle, XY};

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
    building_builder, building_remover, door_builder, gate_builder, gate_opener, gate_remover,
    lift_opener, lift_remover, lift_targeter, piste_builder, piste_computer, piste_highlighter,
    save,
};
use crate::handlers::{lift_builder, selection};
use crate::init::terrain::generate_heightmap;
use crate::init::trees::generate_trees;
use crate::model::ability::Ability;
use crate::model::building::Building;
use crate::model::carousel::{Car, Carousel};
use crate::model::costs::Costs;
use crate::model::door::Door;
use crate::model::entrance::Entrance;
use crate::model::exit::Exit;
use crate::model::frame::Frame;
use crate::model::gate::Gate;
use crate::model::hash_vec::HashVec;
use crate::model::lift::Lift;
use crate::model::piste::{self, Piste};
use crate::model::reservation::Reservation;
use crate::model::skier::{Clothes, Skier};
use crate::model::skiing::{self, State};
use crate::model::tree::Tree;
use crate::services::id_allocator;
use crate::systems::door::Parameters;
use crate::systems::{
    building_artist, carousel, chair_artist, chair_framer, door, door_artist, frame_artist,
    frame_wiper, gate, gate_artist, global_computer, global_target_setter, lift_artist,
    piste_adopter, planner, skiing_framer, target_scrubber, target_setter, terrain_artist,
    tree_artist, window_artist,
};
use crate::utils::computer;

fn main() {
    let components = get_components();
    let max_z = components.terrain.max();

    let engine = glium_backend::engine::GliumEngine::new(
        Game {
            handlers: Handlers {
                building_builder: building_builder::Handler::new(building_builder::Bindings {
                    start_building: Binding::Single {
                        button: Button::Keyboard(KeyboardKey::B),
                        state: ButtonState::Pressed,
                    },
                    finish_building: Binding::Single {
                        button: Button::Keyboard(KeyboardKey::B),
                        state: ButtonState::Pressed,
                    },
                    decrease_height: Binding::Single {
                        button: Button::Keyboard(KeyboardKey::BracketLeft),
                        state: ButtonState::Pressed,
                    },
                    increase_height: Binding::Single {
                        button: Button::Keyboard(KeyboardKey::BracketRight),
                        state: ButtonState::Pressed,
                    },
                    toggle_roof: Binding::Single {
                        button: Button::Keyboard(KeyboardKey::Slash),
                        state: ButtonState::Pressed,
                    },
                }),
                building_remover: building_remover::Handler {
                    binding: Binding::Single {
                        button: Button::Keyboard(KeyboardKey::X),
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
                door_builder: door_builder::Handler {
                    binding: Binding::Single {
                        button: Button::Keyboard(KeyboardKey::R),
                        state: ButtonState::Pressed,
                    },
                },
                drag: drag::Handler::new(drag::Bindings {
                    start_dragging: Binding::Single {
                        button: Button::Mouse(MouseButton::Right),
                        state: ButtonState::Pressed,
                    },
                    stop_dragging: Binding::Single {
                        button: Button::Mouse(MouseButton::Right),
                        state: ButtonState::Released,
                    },
                }),
                gate_builder: gate_builder::Handler::new(Binding::Single {
                    button: Button::Keyboard(KeyboardKey::N),
                    state: ButtonState::Pressed,
                }),
                gate_opener: gate_opener::Handler {
                    binding: Binding::Single {
                        button: Button::Keyboard(KeyboardKey::O),
                        state: ButtonState::Pressed,
                    },
                },
                gate_remover: gate_remover::Handler {
                    binding: Binding::Single {
                        button: Button::Keyboard(KeyboardKey::X),
                        state: ButtonState::Pressed,
                    },
                },
                path_builder: piste_builder::Handler {
                    class: piste::Class::Path,
                    bindings: piste_builder::Bindings {
                        add: Binding::Single {
                            button: Button::Keyboard(KeyboardKey::H),
                            state: ButtonState::Pressed,
                        },
                        subtract: Binding::Single {
                            button: Button::Keyboard(KeyboardKey::X),
                            state: ButtonState::Pressed,
                        },
                    },
                },
                piste_builder: piste_builder::Handler {
                    class: piste::Class::Piste,
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
                piste_computer: piste_computer::Handler {
                    binding: Binding::Single {
                        button: Button::Keyboard(KeyboardKey::C),
                        state: ButtonState::Pressed,
                    },
                },
                piste_highlighter: piste_highlighter::Handler::default(),
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
                lift_targeter: lift_targeter::Handler {
                    binding: Binding::Single {
                        button: Button::Keyboard(KeyboardKey::G),
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
                selection: selection::Handler::new(selection::Bindings {
                    first_cell: Binding::Single {
                        button: Button::Mouse(MouseButton::Left),
                        state: ButtonState::Pressed,
                    },
                    second_cell: Binding::Single {
                        button: Button::Mouse(MouseButton::Left),
                        state: ButtonState::Released,
                    },
                    clear: Binding::Single {
                        button: Button::Keyboard(KeyboardKey::Escape),
                        state: ButtonState::Pressed,
                    },
                }),
                skier_debugger: handlers::skier_debugger::Handler {
                    binding: Binding::Single {
                        button: Button::Keyboard(KeyboardKey::I),
                        state: ButtonState::Pressed,
                    },
                },
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
                    min_level: -1,
                    max_level: 8,
                    bindings: zoom::Bindings {
                        plus: Binding::Multi(vec![
                            Binding::Single {
                                button: Button::Keyboard(KeyboardKey::Equal),
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
                building_artist: building_artist::System::new(),
                carousel: carousel::System::new(),
                chair_artist: chair_artist::System::new(),
                global_computer: global_computer::System::new(),
                skier_colors: systems::skier_colors::System::new(
                    systems::skier_colors::AbilityColors {
                        intermedite: Rgb::new(0.01, 0.41, 0.76),
                        advanced: Rgb::new(0.86, 0.01, 0.01),
                        expert: Rgb::new(0.01, 0.01, 0.01),
                    },
                ),
                terrain_artist: terrain_artist::System::new(terrain_artist::Colors {
                    piste: terrain_artist::AbilityColors {
                        beginner: Rgba::new(0, 98, 19, 128),
                        intermedite: Rgba::new(3, 105, 194, 128),
                        advanced: Rgba::new(219, 2, 3, 128),
                        expert: Rgba::new(3, 2, 3, 128),
                        ungraded: Rgba::new(238, 76, 2, 128),
                    },
                    highlight: terrain_artist::AbilityColors {
                        beginner: Rgba::new(0, 98, 19, 192),
                        intermedite: Rgba::new(3, 105, 194, 192),
                        advanced: Rgba::new(219, 2, 3, 192),
                        expert: Rgba::new(3, 2, 3, 192),
                        ungraded: Rgba::new(238, 76, 2, 192),
                    },
                    cliff: Rgba::new(6, 6, 6, 128),
                }),
                tree_artist: tree_artist::System::new(),
                window_artist: window_artist::System::new(),
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
                    z_max: 1.0 / max_z,
                    viewport: Rectangle {
                        width: 512,
                        height: 512,
                    },
                },
            })),
            light_direction: xyz(0.707_106_77, 0.424_264_07, -0.565_685_45),
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
    let power = 11;
    let terrain = generate_heightmap(power);
    let trees = generate_trees(power, &terrain);
    Components {
        skiers: HashMap::default(),
        plans: HashMap::default(),
        locations: HashMap::default(),
        targets: HashMap::default(),
        global_targets: HashMap::default(),
        frames: HashMap::default(),
        drawings: HashMap::default(),
        pistes: HashMap::default(),
        costs: HashMap::default(),
        global_costs: Costs::new(),
        lifts: HashMap::default(),
        carousels: HashMap::default(),
        cars: HashMap::default(),
        gates: HashMap::default(),
        reservations: Grid::default(terrain.width(), terrain.height()),
        piste_map: Grid::default(terrain.width(), terrain.height()),
        exits: HashMap::default(),
        entrances: HashMap::default(),
        abilities: HashMap::default(),
        clothes: HashMap::default(),
        buildings: HashMap::default(),
        doors: HashMap::default(),
        open: HashSet::default(),
        highlights: HashSet::default(),
        terrain,
        trees,
        planning_queue: HashVec::new(),
        services: Services {
            clock: services::clock::Service::new(),
            id_allocator: id_allocator::Service::new(),
        },
    }
}

struct Game {
    components: Components,
    handlers: Handlers,
    systems: Systems,
    mouse_xy: Option<XY<u32>>,
}

#[derive(Serialize, Deserialize)]
pub struct Components {
    skiers: HashMap<usize, Skier>,
    plans: HashMap<usize, skiing::Plan>,
    locations: HashMap<usize, usize>,
    targets: HashMap<usize, usize>,
    global_targets: HashMap<usize, usize>,
    #[serde(skip)]
    frames: HashMap<usize, Option<Frame>>,
    #[serde(skip)]
    drawings: HashMap<usize, usize>,
    #[serde(skip)]
    clothes: HashMap<usize, Clothes<Rgb<f32>>>,
    pistes: HashMap<usize, Piste>,
    costs: HashMap<usize, Costs<State>>,
    global_costs: Costs<usize>,
    lifts: HashMap<usize, Lift>,
    cars: HashMap<usize, Car>,
    carousels: HashMap<usize, Carousel>,
    gates: HashMap<usize, Gate>,
    entrances: HashMap<usize, Entrance>,
    exits: HashMap<usize, Exit>,
    abilities: HashMap<usize, Ability>,
    buildings: HashMap<usize, Building>,
    doors: HashMap<usize, Door>,
    open: HashSet<usize>,
    #[serde(skip)]
    highlights: HashSet<usize>,
    terrain: Grid<f32>,
    trees: Grid<Option<Tree>>,
    reservations: Grid<HashMap<usize, Reservation>>,
    piste_map: Grid<Option<usize>>,
    planning_queue: HashVec<usize>,
    services: Services,
}

struct Handlers {
    building_builder: building_builder::Handler,
    building_remover: building_remover::Handler,
    clock: handlers::clock::Handler,
    door_builder: door_builder::Handler,
    drag: drag::Handler,
    gate_builder: gate_builder::Handler,
    gate_opener: gate_opener::Handler,
    gate_remover: gate_remover::Handler,
    lift_builder: lift_builder::Handler,
    lift_opener: lift_opener::Handler,
    lift_remover: lift_remover::Handler,
    lift_targeter: lift_targeter::Handler,
    path_builder: piste_builder::Handler,
    piste_builder: piste_builder::Handler,
    piste_computer: piste_computer::Handler,
    piste_highlighter: piste_highlighter::Handler,
    resize: resize::Handler,
    save: save::Handler,
    selection: selection::Handler,
    skier_debugger: handlers::skier_debugger::Handler,
    yaw: yaw::Handler,
    zoom: zoom::Handler,
}

struct Systems {
    building_artist: building_artist::System,
    carousel: carousel::System,
    chair_artist: chair_artist::System,
    global_computer: global_computer::System,
    skier_colors: systems::skier_colors::System,
    terrain_artist: terrain_artist::System,
    tree_artist: tree_artist::System,
    window_artist: window_artist::System,
}

#[derive(Serialize, Deserialize)]
pub struct Services {
    clock: services::clock::Service,
    id_allocator: id_allocator::Service,
}

impl Game {
    fn init(&mut self, graphics: &mut dyn Graphics) {
        let terrain = &self.components.terrain;
        self.systems.chair_artist.init(graphics);
        self.systems.terrain_artist.init(graphics, terrain);
        self.systems.tree_artist.init(graphics);
        self.systems.window_artist.init(graphics);
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

        self.handlers
            .clock
            .handle(event, &mut self.components.services.clock);
        self.handlers
            .path_builder
            .handle(handlers::piste_builder::Parameters {
                event,
                pistes: &mut self.components.pistes,
                piste_map: &mut self.components.piste_map,
                selection: &mut self.handlers.selection,
                terrain_artist: &mut self.systems.terrain_artist,
                tree_artist: &mut self.systems.tree_artist,
                id_allocator: &mut self.components.services.id_allocator,
            });
        self.handlers
            .piste_builder
            .handle(handlers::piste_builder::Parameters {
                event,
                pistes: &mut self.components.pistes,
                piste_map: &mut self.components.piste_map,
                selection: &mut self.handlers.selection,
                terrain_artist: &mut self.systems.terrain_artist,
                tree_artist: &mut self.systems.tree_artist,
                id_allocator: &mut self.components.services.id_allocator,
            });
        self.handlers
            .lift_builder
            .handle(handlers::lift_builder::Parameters {
                event,
                mouse_xy: &self.mouse_xy,
                terrain: &self.components.terrain,
                piste_map: &self.components.piste_map,
                lifts: &mut self.components.lifts,
                open: &mut self.components.open,
                id_allocator: &mut self.components.services.id_allocator,
                carousels: &mut self.components.carousels,
                cars: &mut self.components.cars,
                exits: &mut self.components.exits,
                entrances: &mut self.components.entrances,
                reservations: &mut self.components.reservations,
                graphics,
            });
        self.handlers
            .building_builder
            .handle(handlers::building_builder::Parameters {
                event,
                terrain: &self.components.terrain,
                selection: &mut self.handlers.selection,
                id_allocator: &mut self.components.services.id_allocator,
                buildings: &mut self.components.buildings,
                locations: &mut self.components.locations,
                skiers: &mut self.components.skiers,
                building_artist: &mut self.systems.building_artist,
                tree_artist: &mut self.systems.tree_artist,
                window_artist: &mut self.systems.window_artist,
            });
        self.handlers
            .door_builder
            .handle(handlers::door_builder::Parameters {
                event,
                pistes: &self.components.pistes,
                buildings: &self.components.buildings,
                terrain: &self.components.terrain,
                selection: &mut self.handlers.selection,
                id_allocator: &mut self.components.services.id_allocator,
                doors: &mut self.components.doors,
                entrances: &mut self.components.entrances,
            });

        self.handlers
            .gate_builder
            .handle(handlers::gate_builder::Parameters {
                event,
                piste_map: &self.components.piste_map,
                terrain: &self.components.terrain,
                selection: &mut self.handlers.selection,
                terrain_artist: &mut self.systems.terrain_artist,
                id_allocator: &mut self.components.services.id_allocator,
                gates: &mut self.components.gates,
                entrances: &mut self.components.entrances,
                exits: &mut self.components.exits,
                open: &mut self.components.open,
                reservations: &mut self.components.reservations,
            });
        self.handlers.building_remover.handle(
            event,
            &self.mouse_xy,
            graphics,
            &mut self.components,
            &mut self.systems,
        );
        self.handlers
            .gate_remover
            .handle(event, &self.mouse_xy, graphics, &mut self.components);
        self.handlers
            .lift_remover
            .handle(event, &self.mouse_xy, graphics, &mut self.components);

        self.handlers.lift_opener.handle(
            event,
            &self.mouse_xy,
            &self.components.lifts,
            &mut self.components.open,
            &mut self.systems.global_computer,
            graphics,
        );
        self.handlers.gate_opener.handle(
            event,
            &self.mouse_xy,
            &self.components.gates,
            &mut self.components.open,
            &mut self.systems.global_computer,
            graphics,
        );
        self.handlers.lift_targeter.handle(
            event,
            lift_targeter::Parameters {
                mouse_xy: &self.mouse_xy,
                lifts: &self.components.lifts,
                skiers: &self.components.skiers,
                targets: &mut self.components.targets,
                global_targets: &mut self.components.global_targets,
                graphics,
            },
        );
        self.handlers.save.handle(event, &mut self.components);
        self.handlers.selection.handle(
            event,
            &self.mouse_xy,
            &self.components.terrain,
            graphics,
            &mut self.systems.terrain_artist,
        );
        self.handlers
            .piste_computer
            .handle(handlers::piste_computer::Parameters {
                event,
                mouse_xy: &self.mouse_xy,
                terrain: &self.components.terrain,
                pistes: &self.components.pistes,
                piste_map: &self.components.piste_map,
                entrances: &mut self.components.entrances,
                exits: &mut self.components.exits,
                reservations: &self.components.reservations,
                costs: &mut self.components.costs,
                abilities: &mut self.components.abilities,
                clock: &mut self.components.services.clock,
                global_computer: &mut self.systems.global_computer,
                terrain_artist: &mut self.systems.terrain_artist,
                graphics,
            });

        self.handlers.skier_debugger.handle(
            event,
            handlers::skier_debugger::Parameters {
                mouse_xy: &self.mouse_xy,
                reservations: &self.components.reservations,
                plans: &self.components.plans,
                locations: &self.components.locations,
                targets: &self.components.targets,
                global_targets: &self.components.global_targets,
                graphics,
            },
        );

        self.systems
            .global_computer
            .run(computer::global_costs::Parameters {
                lifts: &self.components.lifts,
                carousels: &self.components.carousels,
                entrances: &self.components.entrances,
                exits: &self.components.exits,
                costs: &self.components.costs,
                abilities: &self.components.abilities,
                open: &self.components.open,
                global_costs: &mut self.components.global_costs,
            });

        self.systems.carousel.run(systems::carousel::Parameters {
            micros: &self.components.services.clock.get_micros(),
            lifts: &self.components.lifts,
            open: &self.components.open,
            carousels: &self.components.carousels,
            reservations: &mut self.components.reservations,
            plans: &mut self.components.plans,
            locations: &mut self.components.locations,
            targets: &mut self.components.targets,
            global_targets: &mut self.components.global_targets,
            cars: &mut self.components.cars,
        });

        target_scrubber::run(&self.components.open, &mut self.components.targets);
        piste_adopter::run(
            &self.components.plans,
            &self.components.piste_map,
            &self.components.skiers,
            &self.components.abilities,
            &mut self.components.locations,
        );
        global_target_setter::run(global_target_setter::Parameters {
            skiers: &self.components.skiers,
            plans: &self.components.plans,
            locations: &self.components.locations,
            entrances: &self.components.entrances,
            pistes: &self.components.pistes,
            costs: &self.components.costs,
            global_costs: &self.components.global_costs,
            global_targets: &mut self.components.global_targets,
        });
        target_setter::run(target_setter::Parameters {
            skiers: &self.components.skiers,
            plans: &self.components.plans,
            locations: &self.components.locations,
            global_costs: &self.components.global_costs,
            costs: &self.components.costs,
            open: &self.components.open,
            global_targets: &mut self.components.global_targets,
            targets: &mut self.components.targets,
        });

        door::run(Parameters {
            doors: &self.components.doors,
            reservations: &mut self.components.reservations,
            locations: &mut self.components.locations,
            plans: &mut self.components.plans,
        });
        gate::run(
            &self.components.plans,
            &self.components.gates,
            &self.components.entrances,
            &self.components.open,
            &mut self.components.targets,
            &mut self.components.global_targets,
            &mut self.components.locations,
        );
        planner::run(systems::planner::Parameters {
            terrain: &self.components.terrain,
            micros: &self.components.services.clock.get_micros(),
            skiers: &mut self.components.skiers,
            locations: &self.components.locations,
            targets: &self.components.targets,
            pistes: &self.components.pistes,
            costs: &self.components.costs,
            plans: &mut self.components.plans,
            reservations: &mut self.components.reservations,
            planning_queue: &mut self.components.planning_queue,
        });
        frame_wiper::run(&mut self.components.frames);
        self.systems
            .skier_colors
            .run(&self.components.skiers, &mut self.components.clothes);
        skiing_framer::run(
            &self.components.terrain,
            &self.components.services.clock.get_micros(),
            &self.components.plans,
            &self.components.clothes,
            &mut self.components.frames,
        );
        chair_framer::run(
            &self.components.carousels,
            &self.components.lifts,
            &self.components.cars,
            &self.components.locations,
            &self.components.clothes,
            &mut self.components.frames,
        );
        self.systems.building_artist.run(
            graphics,
            &self.components.buildings,
            &self.components.terrain,
            &mut self.components.drawings,
        );
        door_artist::run(
            graphics,
            &self.components.doors,
            &self.components.buildings,
            &self.components.terrain,
            &mut self.components.drawings,
        );
        frame_artist::run(
            graphics,
            &self.components.frames,
            &mut self.components.drawings,
        );
        lift_artist::run(
            graphics,
            &self.components.lifts,
            &mut self.components.drawings,
        );
        gate_artist::run(
            graphics,
            &self.components.gates,
            &self.components.entrances,
            &self.components.terrain,
            &self.components.piste_map,
            &mut self.components.drawings,
        );
        self.handlers
            .piste_highlighter
            .handle(handlers::piste_highlighter::Parameters {
                event,
                mouse_xy: &self.mouse_xy,
                pistes: &self.components.pistes,
                piste_map: &self.components.piste_map,
                highlights: &mut self.components.highlights,
                terrain_artist: &mut self.systems.terrain_artist,
                graphics,
            });

        self.systems
            .chair_artist
            .run(&self.components.frames, graphics);
        self.systems
            .terrain_artist
            .run(systems::terrain_artist::Parameters {
                terrain: &self.components.terrain,
                piste_map: &self.components.piste_map,
                highlights: &self.components.highlights,
                abilities: &self.components.abilities,
                selection: &self.handlers.selection,
                graphics,
            });
        self.systems.tree_artist.run(
            &self.components.trees,
            &self.components.terrain,
            &self.components.piste_map,
            &self.components.buildings,
            graphics,
        );
        self.systems
            .window_artist
            .run(&self.components.buildings, graphics);

        gui::run(self, engine, graphics);
    }
}
