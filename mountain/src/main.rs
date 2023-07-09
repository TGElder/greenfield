mod draw;
mod handlers;
mod init;
mod model;
mod network;
mod physics;
mod services;
mod systems;

use std::collections::HashMap;
use std::f32::consts::PI;
use std::time::Duration;

use commons::color::Rgba;
use commons::geometry::{xy, xyz, Rectangle, XY};

use commons::grid::Grid;
use engine::binding::Binding;
use engine::engine::Engine;
use engine::events::{Button, ButtonState, Event, EventHandler, KeyboardKey, MouseButton};
use engine::glium_backend;

use engine::graphics::projections::isometric;
use engine::graphics::Graphics;
use engine::handlers::{drag, resize, yaw, zoom};

use crate::handlers::{add_skier, piste_builder};
use crate::handlers::{lift_builder, selection};
use crate::init::generate_heightmap;
use crate::model::{skiing, Frame, Lift, Piste, PisteCosts};
use crate::services::id_allocator;
use crate::systems::{avatar_artist, cost_computer, framer, lift, lift_entry, overlay, planner};

fn main() {
    let terrain = generate_heightmap();
    let engine = glium_backend::engine::GliumEngine::new(
        Game {
            components: Components {
                plans: HashMap::default(),
                locations: HashMap::default(),
                targets: HashMap::default(),
                frames: HashMap::default(),
                drawings: HashMap::default(),
                pistes: HashMap::default(),
                piste_costs: HashMap::default(),
                lifts: HashMap::default(),
                reserved: Grid::default(terrain.width(), terrain.height()),
                terrain,
            },
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
                selection: selection::Handler::new(Binding::Single {
                    button: Button::Mouse(MouseButton::Right),
                    state: ButtonState::Pressed,
                }),
            },
            systems: Systems {
                overlay: overlay::System::new(overlay::Colors {
                    selection: Rgba::new(255, 255, 0, 128),
                    piste: Rgba::new(0, 0, 255, 128),
                    lift: Rgba::new(0, 0, 0, 255),
                }),
                planner: planner::System::new(),
            },
            services: Services {
                clock: services::clock::Service::new(),
                id_allocator: id_allocator::Service::new(),
            },
            mouse_xy: None,
            drag_handler: drag::Handler::new(drag::Bindings {
                start_dragging: Binding::Single {
                    button: Button::Mouse(MouseButton::Left),
                    state: ButtonState::Pressed,
                },
                stop_dragging: Binding::Single {
                    button: Button::Mouse(MouseButton::Left),
                    state: ButtonState::Released,
                },
            }),
            resize_handler: resize::Handler::new(),
            yaw_handler: yaw::Handler::new(yaw::Parameters {
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
            zoom_handler: zoom::Handler::new(zoom::Parameters {
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
    services: Services,
    mouse_xy: Option<XY<u32>>,
    drag_handler: drag::Handler,
    resize_handler: resize::Handler,
    yaw_handler: yaw::Handler,
    zoom_handler: zoom::Handler,
}

struct Components {
    plans: HashMap<usize, skiing::Plan>,
    locations: HashMap<usize, usize>,
    targets: HashMap<usize, usize>,
    frames: HashMap<usize, Frame>,
    drawings: HashMap<usize, usize>,
    pistes: HashMap<usize, Piste>,
    piste_costs: HashMap<usize, PisteCosts>,
    lifts: HashMap<usize, Lift>,
    terrain: Grid<f32>,
    reserved: Grid<bool>,
}

struct Drawings {
    terrain: draw::terrain::Drawing,
}

struct Handlers {
    add_skier: add_skier::Handler,
    clock: handlers::clock::Handler,
    piste_builder: piste_builder::Handler,
    lift_builder: lift_builder::Handler,
    selection: selection::Handler,
}

struct Systems {
    overlay: overlay::System,
    planner: planner::System,
}

struct Services {
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

    // Temporary until we have proper logic for setting locations and targets
    fn set_locations_and_targets(&mut self) {
        let Some(piste) = self.components.pistes.keys().next() else {return};
        let Some(lift) = self.components.lifts.keys().max() else {return};
        for (i, _) in self.components.plans.iter() {
            self.components.locations.entry(*i).or_insert(*piste);
            self.components.targets.entry(*i).or_insert(*lift);
        }
    }
}

impl EventHandler for Game {
    fn handle(&mut self, event: &Event, engine: &mut dyn Engine, graphics: &mut dyn Graphics) {
        match event {
            Event::Init => self.init(graphics),
            Event::MouseMoved(xy) => self.mouse_xy = Some(*xy),
            _ => (),
        }

        self.drag_handler.handle(event, engine, graphics);
        self.resize_handler.handle(event, engine, graphics);
        self.yaw_handler.handle(event, engine, graphics);
        self.zoom_handler.handle(event, engine, graphics);

        self.handlers.add_skier.handle(
            event,
            &self.mouse_xy,
            &mut self.components.plans,
            &mut self.services.id_allocator,
            graphics,
        );
        self.handlers.clock.handle(event, &mut self.services.clock);
        self.handlers.piste_builder.handle(
            event,
            &mut self.components.pistes,
            &mut self.handlers.selection,
            &mut self.systems.overlay,
            &mut self.services.id_allocator,
        );
        self.handlers.lift_builder.handle(
            event,
            &mut self.components.lifts,
            &self.mouse_xy,
            &mut self.systems.overlay,
            &mut self.services.id_allocator,
            graphics,
        );
        self.handlers
            .selection
            .handle(event, &self.mouse_xy, graphics, &mut self.systems.overlay);

        self.set_locations_and_targets();

        self.systems.planner.run(systems::planner::Parameters {
            terrain: &self.components.terrain,
            micros: &self.services.clock.get_micros(),
            plans: &mut self.components.plans,
            locations: &self.components.locations,
            targets: &self.components.targets,
            costs: &self.components.piste_costs,
            pistes: &self.components.pistes,
            reserved: &mut self.components.reserved,
        });
        lift_entry::run(
            &self.components.plans,
            &self.components.targets,
            &self.components.lifts,
            &mut self.components.locations,
        );
        lift::run(
            &self.components.lifts,
            &mut self.components.locations,
            &mut self.components.reserved,
            &mut self.components.plans,
        );
        framer::run(
            &self.components.terrain,
            &self.services.clock.get_micros(),
            &self.components.plans,
            &mut self.components.frames,
        );
        avatar_artist::run(
            graphics,
            &self.components.frames,
            &mut self.components.drawings,
        );
        self.systems.overlay.run(
            graphics,
            self.drawings.as_ref().map(|drawings| &drawings.terrain),
            &self.components.pistes,
            &self.components.piste_costs,
            &self.components.lifts,
            &self.handlers.selection,
        );

        const COMPUTE_COSTS_BINDING: Binding = Binding::Single {
            button: Button::Keyboard(KeyboardKey::C),
            state: ButtonState::Pressed,
        };

        if COMPUTE_COSTS_BINDING.binds_event(event) {
            cost_computer::run(
                &self.components.terrain,
                &self.components.pistes,
                &mut self.components.piste_costs,
                &self.components.lifts,
            );
        }
    }
}
