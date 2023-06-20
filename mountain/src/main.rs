mod draw;
mod handlers;
mod init;
mod model;
mod network;
mod physics;
mod systems;

use std::collections::HashMap;
use std::f32::consts::PI;
use std::time::{Duration, Instant};

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
use crate::model::{skiing, Frame, Lift, Piste};
use crate::systems::{avatar_artist, framer, overlay, planner};

fn main() {
    let terrain = generate_heightmap();
    let engine = glium_backend::engine::GliumEngine::new(
        Game {
            components: Components {
                plans: HashMap::default(),
                frames: HashMap::default(),
                drawings: HashMap::default(),
                pistes: HashMap::default(),
                reserved: Grid::default(terrain.width(), terrain.height()),
                lifts: Vec::default(),
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
            },
            start: Instant::now(),
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
    start: Instant,
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
    pistes: HashMap<usize, Piste>,
    lifts: Vec<Lift>,
    terrain: Grid<f32>,
    reserved: Grid<bool>,
}

struct Drawings {
    terrain: draw::terrain::Drawing,
}

struct Handlers {
    add_skier: add_skier::Handler,
    piste_builder: piste_builder::Handler,
    lift_builder: lift_builder::Handler,
    selection: selection::Handler,
}

struct Systems {
    overlay: overlay::System,
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

        self.drag_handler.handle(event, engine, graphics);
        self.resize_handler.handle(event, engine, graphics);
        self.yaw_handler.handle(event, engine, graphics);
        self.zoom_handler.handle(event, engine, graphics);

        self.handlers
            .add_skier
            .handle(event, &self.mouse_xy, &mut self.components.plans, graphics);
        self.handlers.piste_builder.handle(
            event,
            &mut self.components.pistes,
            &mut self.handlers.selection,
            &mut self.systems.overlay,
        );
        self.handlers.lift_builder.handle(
            event,
            &mut self.components.lifts,
            &self.mouse_xy,
            &mut self.systems.overlay,
            graphics,
        );
        self.handlers
            .selection
            .handle(event, &self.mouse_xy, graphics, &mut self.systems.overlay);

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
        self.systems.overlay.run(
            graphics,
            self.drawings.as_ref().map(|drawings| &drawings.terrain),
            &self.components.pistes,
            &self.components.lifts,
            &self.handlers.selection,
        );
    }
}
