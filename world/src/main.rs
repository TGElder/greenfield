use std::f32::consts::PI;
use std::time::Duration;

use commons::geometry::{xyz, Rectangle};
use commons::grid::Grid;
use engine::binding::Binding;
use engine::events::{Button, ButtonState, KeyboardKey, MouseButton};
use engine::glium_backend;
use engine::graphics::projections::isometric;
use engine::handlers::{drag, yaw, zoom};

use crate::draw::sea;
use crate::draw::terrain::Drawing;

mod draw;
mod init;

struct Game {
    components: Components,
    handlers: Handlers,
    bindings: Bindings,
    drawing: Option<Drawing>,
}

struct Components {
    sea_level: f32,
    terrain: Grid<f32>,
}

struct Handlers {
    drag: drag::Handler,
    yaw: yaw::Handler,
    zoom: zoom::Handler,
}

struct Bindings {
    drag: drag::Bindings,
    yaw: yaw::Bindings,
    zoom: zoom::Bindings,
}

fn main() {
    let max_z = 4096.0;

    let engine = glium_backend::engine::GliumEngine::new(
        Game {
            components: Components {
                terrain: init::terrain::generate_heightmap(init::terrain::Parameters {
                    power: 11,
                    seed: 0,
                }),
                sea_level: 16.0,
            },
            drawing: None,
            handlers: Handlers {
                drag: drag::Handler::default(),
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
            bindings: Bindings {
                drag: drag::Bindings {
                    start_dragging: Binding::Single {
                        button: Button::Mouse(MouseButton::Left),
                        state: ButtonState::Pressed,
                    },
                    stop_dragging: Binding::Single {
                        button: Button::Mouse(MouseButton::Left),
                        state: ButtonState::Released,
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
        },
        glium_backend::engine::Parameters {
            frame_duration: Duration::from_nanos(16_666_667),
        },
        glium_backend::graphics::Parameters {
            name: "Greenworld".to_string(),
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

impl engine::events::EventHandler for Game {
    fn handle(
        &mut self,
        event: &engine::events::Event,
        engine: &mut dyn engine::engine::Engine,
        graphics: &mut dyn engine::graphics::Graphics,
    ) {
        if let engine::events::Event::Init = event {
            let drawing = Drawing::init(graphics, &self.components.terrain);
            drawing.draw_geometry(graphics, &self.components.terrain);
            self.drawing = Some(drawing);

            sea::draw(
                &self.components.terrain,
                self.components.sea_level,
                graphics,
            );
        }

        self.handlers
            .drag
            .handle(&self.bindings.drag, event, engine, graphics);
        self.handlers
            .yaw
            .handle(&self.bindings.yaw, event, engine, graphics);
        self.handlers
            .zoom
            .handle(&self.bindings.zoom, event, engine, graphics);
        engine::handlers::resize::handle(event, engine, graphics);
    }
}
