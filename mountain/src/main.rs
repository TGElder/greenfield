#[macro_use]
extern crate lazy_static;

mod controllers;
mod draw;
mod gui;
mod handlers;
mod init;
mod model;
mod network;
mod services;
mod systems;
mod utils;
mod widgets;

use std::collections::{HashMap, HashSet};
use std::f32::consts::PI;
use std::fs::{create_dir_all, File};
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
use engine::handlers::{drag, yaw, zoom};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast::{self};

use crate::controllers::building_builder::FinalizeParameters;
use crate::controllers::{building_builder, lift_builder, piste_builder, piste_eraser};
use crate::gui::Widgets;
use crate::handlers::{lift_targeter, piste_build_mode, piste_highlighter, selection};
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
use crate::model::selection::Selection;
use crate::model::skier::{Clothes, Skier};
use crate::model::skiing::{self, State};
use crate::model::tree::Tree;
use crate::services::{id_allocator, mode};
use crate::systems::door::Parameters;
use crate::systems::{
    building_artist, carousel, chair_artist, chair_framer, door, door_artist, frame_artist,
    frame_wiper, gate, gate_artist, global_computer, global_target_setter, lift_artist, log,
    messenger, piste_adopter, planner, selection_rasterizer, skiing_framer, target_scrubber,
    target_setter, terrain_artist, tree_artist, window_artist,
};
use crate::utils::computer;
use crate::widgets::{building_editor, menu, toaster};

fn main() {
    let max_z = 4096.0;

    let components = new_components(DEFAULT_NEW_GAME_PARAMETERS);

    let engine = glium_backend::engine::GliumEngine::new(
        new_game(components, None),
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

fn load_components(path: &str) -> Option<Components> {
    let file = File::open(path).ok()?;
    bincode::deserialize_from(BufReader::new(file)).ok()
}

fn new_game(components: Components, save_file: Option<String>) -> Game {
    let (tx, _) = broadcast::channel(1000);
    Game {
        controllers: Controllers {
            building_builder: building_builder::Controller::new(),
            path_builder: piste_builder::Controller::new(piste::Class::Path, true),
            piste_builder: piste_builder::Controller::new(piste::Class::Piste, true),
            piste_eraser: piste_eraser::Controller::new(false),
            lift_builder: lift_builder::Controller::new(),
        },
        handlers: Handlers {
            clock: handlers::clock::Handler::new(),
            drag: drag::Handler::default(),
            piste_highlighter: piste_highlighter::Handler::default(),
            selection: selection::Handler::new(),
            yaw: yaw::Handler::new(yaw::Parameters {
                initial_angle: 216,
                angles: 720,
                step_angles: 60,
            }),
            zoom: zoom::Handler::new(zoom::Parameters {
                initial_level: 1,
                min_level: -1,
                max_level: 8,
            }),
        },
        widgets: Widgets {
            building_editor: building_editor::Widget::default(),
            menu: menu::Widget::default(),
            piste_build_mode: widgets::piste_build_mode::Widget::default(),
            toaster: toaster::Widget::new(log::System::new(
                tx.subscribe(),
                log::Parameters {
                    max_duration: Duration::from_secs(5),
                    max_length: 8,
                },
            )),
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
            messenger: messenger::System::new(tx),
            tree_artist: tree_artist::System::new(),
            window_artist: window_artist::System::new(),
        },
        bindings: Bindings {
            action: Binding::Single {
                button: Button::Mouse(MouseButton::Left),
                state: ButtonState::Pressed,
            },
            clock_handler: handlers::clock::Bindings {
                slow_down: Binding::Single {
                    button: Button::Keyboard(KeyboardKey::from(",")),
                    state: ButtonState::Pressed,
                },
                speed_up: Binding::Single {
                    button: Button::Keyboard(KeyboardKey::from(".")),
                    state: ButtonState::Pressed,
                },
            },
            compute: Binding::Single {
                button: Button::Keyboard(KeyboardKey::from("c")),
                state: ButtonState::Pressed,
            },
            drag: drag::Bindings {
                start_dragging: Binding::Single {
                    button: Button::Mouse(MouseButton::Right),
                    state: ButtonState::Pressed,
                },
                stop_dragging: Binding::Single {
                    button: Button::Mouse(MouseButton::Right),
                    state: ButtonState::Released,
                },
            },
            main_menu: Binding::Single {
                button: Button::Keyboard(KeyboardKey::Escape),
                state: ButtonState::Released,
            },
            mode: HashMap::from([
                (
                    mode::Mode::Open,
                    Binding::Single {
                        button: Button::Keyboard(KeyboardKey::from("o")),
                        state: ButtonState::Pressed,
                    },
                ),
                (
                    mode::Mode::Query,
                    Binding::Single {
                        button: Button::Keyboard(KeyboardKey::from("?")),
                        state: ButtonState::Pressed,
                    },
                ),
                (
                    mode::Mode::Piste,
                    Binding::Single {
                        button: Button::Keyboard(KeyboardKey::from("p")),
                        state: ButtonState::Pressed,
                    },
                ),
                (
                    mode::Mode::Path,
                    Binding::Single {
                        button: Button::Keyboard(KeyboardKey::from("w")),
                        state: ButtonState::Pressed,
                    },
                ),
                (
                    mode::Mode::Lift,
                    Binding::Single {
                        button: Button::Keyboard(KeyboardKey::from("l")),
                        state: ButtonState::Pressed,
                    },
                ),
                (
                    mode::Mode::Gate,
                    Binding::Single {
                        button: Button::Keyboard(KeyboardKey::from("g")),
                        state: ButtonState::Pressed,
                    },
                ),
                (
                    mode::Mode::Building,
                    Binding::Single {
                        button: Button::Keyboard(KeyboardKey::from("h")),
                        state: ButtonState::Pressed,
                    },
                ),
                (
                    mode::Mode::Door,
                    Binding::Single {
                        button: Button::Keyboard(KeyboardKey::from("d")),
                        state: ButtonState::Pressed,
                    },
                ),
                (
                    mode::Mode::Demolish,
                    Binding::Single {
                        button: Button::Keyboard(KeyboardKey::Backspace),
                        state: ButtonState::Pressed,
                    },
                ),
            ]),
            piste_mode: piste_build_mode::Bindings {
                build: Binding::Single {
                    button: Button::Keyboard(KeyboardKey::Shift),
                    state: ButtonState::Released,
                },
                erase: Binding::Single {
                    button: Button::Keyboard(KeyboardKey::Shift),
                    state: ButtonState::Pressed,
                },
            },
            selection: selection::Bindings {
                first_cell: Binding::Single {
                    button: Button::Mouse(MouseButton::Left),
                    state: ButtonState::Pressed,
                },
                second_cell: Binding::Single {
                    button: Button::Mouse(MouseButton::Left),
                    state: ButtonState::Released,
                },
                start_clearing: Binding::Single {
                    button: Button::Mouse(MouseButton::Right),
                    state: ButtonState::Pressed,
                },
                finish_clearing: Binding::Single {
                    button: Button::Mouse(MouseButton::Right),
                    state: ButtonState::Released,
                },
            },
            target_lift: Binding::Single {
                button: Button::Keyboard(KeyboardKey::from("x")),
                state: ButtonState::Pressed,
            },
            view: handlers::view::Bindings {
                toggle_pistes: Binding::Single {
                    button: Button::Keyboard(KeyboardKey::from("P")),
                    state: ButtonState::Pressed,
                },
                toggle_trees: Binding::Single {
                    button: Button::Keyboard(KeyboardKey::from("t")),
                    state: ButtonState::Pressed,
                },
                toggle_skier_ability: Binding::Single {
                    button: Button::Keyboard(KeyboardKey::from("a")),
                    state: ButtonState::Pressed,
                },
            },
            yaw: yaw::Bindings {
                step_plus: Binding::Single {
                    button: Button::Keyboard(KeyboardKey::from("e")),
                    state: ButtonState::Pressed,
                },
                step_minus: Binding::Single {
                    button: Button::Keyboard(KeyboardKey::from("q")),
                    state: ButtonState::Pressed,
                },
                mouse_yaw_enable: Binding::Single {
                    button: Button::Mouse(MouseButton::Middle),
                    state: ButtonState::Pressed,
                },
                mouse_yaw_disable: Binding::Single {
                    button: Button::Mouse(MouseButton::Middle),
                    state: ButtonState::Released,
                },
            },
            zoom: zoom::Bindings {
                plus: Binding::Multi(vec![
                    Binding::Single {
                        button: Button::Keyboard(KeyboardKey::from("+")),
                        state: ButtonState::Pressed,
                    },
                    Binding::Single {
                        button: Button::Mouse(MouseButton::WheelUp),
                        state: ButtonState::Pressed,
                    },
                ]),
                minus: Binding::Multi(vec![
                    Binding::Single {
                        button: Button::Keyboard(KeyboardKey::from("-")),
                        state: ButtonState::Pressed,
                    },
                    Binding::Single {
                        button: Button::Mouse(MouseButton::WheelDown),
                        state: ButtonState::Pressed,
                    },
                ]),
            },
        },
        config: Config {
            save_file,
            save_directory: "./saves/".to_string(),
            save_extension: "save".to_string(),
        },
        mouse_xy: None,
        components,
        file_to_load: None,
    }
}

pub struct NewGameParameters {
    terrain: init::terrain::Parameters,
    trees: init::trees::Parameters,
}

const DEFAULT_NEW_GAME_PARAMETERS: NewGameParameters = NewGameParameters {
    terrain: init::terrain::Parameters { power: 11, seed: 0 },
    trees: init::trees::Parameters {
        power: 11,
        tree_line_elevation: 512.0,
    },
};

fn new_components(parameters: NewGameParameters) -> Components {
    let terrain = generate_heightmap(parameters.terrain);
    let trees = generate_trees(&terrain, parameters.trees);
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
        selection: Selection::default(),
        services: Services {
            clock: services::clock::Service::new(),
            id_allocator: id_allocator::Service::new(),
            mode: mode::Service::default(),
        },
    }
}

struct Game {
    components: Components,
    controllers: Controllers,
    handlers: Handlers,
    systems: Systems,
    bindings: Bindings,
    config: Config,
    widgets: Widgets,
    mouse_xy: Option<XY<u32>>,
    file_to_load: Option<String>,
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
    #[serde(skip)]
    selection: Selection,
    services: Services,
}

struct Controllers {
    building_builder: building_builder::Controller,
    lift_builder: lift_builder::Controller,
    path_builder: piste_builder::Controller,
    piste_builder: piste_builder::Controller,
    piste_eraser: piste_eraser::Controller,
}

struct Handlers {
    clock: handlers::clock::Handler,
    drag: drag::Handler,
    piste_highlighter: piste_highlighter::Handler,
    selection: selection::Handler,
    yaw: yaw::Handler,
    zoom: zoom::Handler,
}

struct Systems {
    building_artist: building_artist::System,
    carousel: carousel::System,
    chair_artist: chair_artist::System,
    global_computer: global_computer::System,
    messenger: messenger::System,
    skier_colors: systems::skier_colors::System,
    terrain_artist: terrain_artist::System,
    tree_artist: tree_artist::System,
    window_artist: window_artist::System,
}

pub struct Bindings {
    action: Binding,
    clock_handler: handlers::clock::Bindings,
    compute: Binding,
    drag: drag::Bindings,
    piste_mode: piste_build_mode::Bindings,
    main_menu: Binding,
    mode: HashMap<mode::Mode, Binding>,
    selection: selection::Bindings,
    target_lift: Binding,
    view: handlers::view::Bindings,
    yaw: yaw::Bindings,
    zoom: zoom::Bindings,
}

pub struct Config {
    save_file: Option<String>,
    save_directory: String,
    save_extension: String,
}

#[derive(Serialize, Deserialize)]
pub struct Services {
    clock: services::clock::Service,
    id_allocator: id_allocator::Service,
    #[serde(skip)]
    mode: services::mode::Service,
}

impl Game {
    fn init(&mut self, graphics: &mut dyn Graphics) {
        self.try_create_save_directory();
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

    fn try_create_save_directory(&mut self) {
        if let Err(e) = create_dir_all(self.config.save_directory.clone()) {
            let message = format!(
                "Could not create save directory {}",
                self.config.save_directory
            );
            eprintln!("{}: {}", message, e);
            self.systems.messenger.send(message);
        }
    }

    fn load(&mut self, file: String, graphics: &mut dyn Graphics) {
        let components = load_components(&format!(
            "{}{}.{}",
            self.config.save_directory, &file, self.config.save_extension
        ));
        match components {
            Some(components) => {
                *self = new_game(components, Some(file));
                graphics.clear();
                self.init(graphics);
            }
            None => {
                self.systems
                    .messenger
                    .send(format!("Could not load {}", file));
            }
        }
    }
}

impl EventHandler for Game {
    fn handle(&mut self, event: &Event, engine: &mut dyn Engine, graphics: &mut dyn Graphics) {
        if let Some(file) = &self.file_to_load {
            self.load(file.clone(), graphics);
        }

        match event {
            Event::Init => self.init(graphics),
            Event::MouseMoved(xy) => self.mouse_xy = Some(*xy),
            _ => (),
        }

        self.handlers
            .drag
            .handle(&self.bindings.drag, event, engine, graphics);
        engine::handlers::resize::handle(event, engine, graphics);
        self.handlers
            .yaw
            .handle(&self.bindings.yaw, event, engine, graphics);
        self.handlers
            .zoom
            .handle(&self.bindings.zoom, event, engine, graphics);

        self.handlers.clock.handle(
            &self.bindings.clock_handler,
            event,
            &mut self.components.services.clock,
        );

        handlers::piste_build_mode::handle(
            &self.bindings.piste_mode,
            event,
            &mut self.controllers.path_builder,
            &mut self.controllers.piste_builder,
            &mut self.controllers.piste_eraser,
        );

        self.components.services.mode.get_handler()(event, self, graphics);

        handlers::lift_targeter::handle(
            &self.bindings.target_lift,
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
        handlers::piste_computer::handle(handlers::piste_computer::Parameters {
            binding: &self.bindings.compute,
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
        selection_rasterizer::run(selection_rasterizer::Parameters {
            terrain: &self.components.terrain,
            selection: &mut self.components.selection,
            terrain_artist: &mut self.systems.terrain_artist,
        });
        handlers::mode::handle(
            event,
            &self.bindings,
            &mut self.components.services.mode,
            &mut self.components.selection,
        );
        handlers::view::handle(
            event,
            handlers::view::Parameters {
                bindings: &self.bindings.view,
                terrain_artist: &mut self.systems.terrain_artist,
                tree_artist: &mut self.systems.tree_artist,
                skier_colors: &mut self.systems.skier_colors,
                graphics,
            },
        );

        self.controllers
            .building_builder
            .finalize(FinalizeParameters {
                terrain: &self.components.terrain,
                id_allocator: &mut self.components.services.id_allocator,
                buildings: &mut self.components.buildings,
                locations: &mut self.components.locations,
                skiers: &mut self.components.skiers,
                building_artist: &mut self.systems.building_artist,
                window_artist: &mut self.systems.window_artist,
                messenger: &mut self.systems.messenger,
            });

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
                selection: &self.components.selection,
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

        gui::run(self, event, engine, graphics);
    }
}
