use std::collections::{HashMap, HashSet};
use std::f32::consts::PI;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};

use commons::color::Rgba;
use commons::geometry::{xy, xyz, Rectangle, XY};
use commons::grid::Grid;
use commons::origin_grid::OriginGrid;
use engine::binding::Binding;
use engine::events::{Button, ButtonState, KeyboardKey, MouseButton};
use engine::glium_backend;
use engine::graphics::projections::isometric;
use engine::handlers::{drag, yaw, zoom};

use crate::draw::terrain::{Drawing, ROAD};
use crate::draw::{sea, town};
use crate::init::resources::generate_resources;
use crate::init::towns::generate_towns;
use crate::model::allocation::Allocation;
use crate::model::path::Path;
use crate::model::resource::Resource;
use crate::model::source::Source;
use crate::system::{allocation, demand, paths_between_towns, roads, routes, sources, traffic};
use crate::utils::tile_heights;

mod draw;
mod init;
mod model;
mod system;
mod utils;

struct Game {
    handlers: Handlers,
    bindings: Bindings,
    drawing: Option<Drawing>,
    compute_tx: Sender<Components>,
    draw_rx: Receiver<Components>,
}

struct Components {
    sea_level: f32,
    cliff_rise: f32,
    terrain: Grid<f32>,
    tile_heights: Grid<f32>,
    towns: Grid<bool>,
    resources: Grid<Option<Resource>>,
    markets: Grid<Vec<Source>>,
    demand: Grid<Vec<Source>>,
    paths: HashMap<(XY<u32>, XY<u32>), Path>,
    routes: HashMap<(XY<u32>, XY<u32>), Path>,
    allocation: Vec<Allocation>,
    traffic: Grid<usize>,
    roads: Grid<bool>,
    links: HashSet<(XY<u32>, XY<u32>)>,
}

struct Handlers {
    drag: drag::Handler,
    yaw: yaw::Handler,
    zoom: zoom::Handler,
    resource_artist: draw::resource::Artist,
}

struct Bindings {
    drag: drag::Bindings,
    yaw: yaw::Bindings,
    zoom: zoom::Bindings,
}

fn main() {
    let max_z = 4096.0;

    let sea_level = 16.0;
    let cliff_rise = 1.0;
    println!("Generating terrain");
    let terrain =
        init::terrain::generate_heightmap(init::terrain::Parameters { power: 10, seed: 0 });

    println!("Computing tile heights");
    let tile_heights = tile_heights(&terrain);

    println!("Generating resources");
    let resources = generate_resources(10, &tile_heights, sea_level, cliff_rise);

    println!("Placing towns");
    let towns = generate_towns(&tile_heights, sea_level, cliff_rise, 1024);

    println!("Generating demand");
    let mut demand = tile_heights.map(|_, _| vec![]);
    demand::run(&towns, &mut demand);

    let (compute_tx, compute_rx) = mpsc::channel::<Components>();
    let (draw_tx, draw_rx) = mpsc::channel::<Components>();

    let components = Components {
        sea_level,
        cliff_rise,
        terrain,
        resources,
        towns,
        markets: tile_heights.map(|_, _| vec![]),
        demand,
        paths: HashMap::default(),
        routes: HashMap::default(),
        allocation: vec![],
        traffic: tile_heights.map(|_, _| 0),
        roads: tile_heights.map(|_, _| false),
        links: HashSet::new(),
        tile_heights,
    };

    draw_tx.send(components).unwrap();

    thread::spawn(move || loop {
        if let Ok(mut components) = compute_rx.try_recv() {
            println!("Computing paths between towns");
            let start = Instant::now();
            components.paths.clear();
            paths_between_towns::run(
                &components.towns,
                components.sea_level,
                components.cliff_rise,
                &components.tile_heights,
                &components.roads,
                &mut components.paths,
            );
            println!(
                "Computed paths between towns in {}ms",
                start.elapsed().as_millis()
            );

            println!("Computing routes");
            let start = Instant::now();
            components.routes.clear();
            routes::run(&components.paths, &mut components.routes);
            println!("Computed routes in {}ms", start.elapsed().as_millis());

            println!("Computing sources");
            let start = Instant::now();
            components.markets = components.tile_heights.map(|_, _| vec![]);
            sources::run(
                &components.towns,
                components.sea_level,
                components.cliff_rise,
                &components.tile_heights,
                &components.roads,
                &components.resources,
                &mut components.markets,
                &mut components.paths,
            );
            println!("Computed sources in {}ms", start.elapsed().as_millis());

            println!("Allocating");
            let start = Instant::now();
            allocation::run(
                &components.markets,
                &components.demand,
                &components.routes,
                &mut components.allocation,
            );
            println!("Computed allocation in {}ms", start.elapsed().as_millis());

            println!("Computing traffic");
            components.traffic = components.tile_heights.map(|_, _| 0);
            let start = Instant::now();
            traffic::run(
                &components.allocation,
                &components.paths,
                &components.routes,
                &mut components.traffic,
            );
            println!("Computed traffc in {}ms", start.elapsed().as_millis());

            println!("Computing roads");
            let start = Instant::now();
            roads::run(
                &components.allocation,
                &components.paths,
                &components.routes,
                &components.towns,
                &mut components.roads,
                &mut components.links,
            );
            println!("Computed roads in {}ms", start.elapsed().as_millis());

            draw_tx.send(components).unwrap();
        }
    });

    let engine = glium_backend::engine::GliumEngine::new(
        Game {
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
                resource_artist: draw::resource::Artist::default(),
            },
            compute_tx,
            draw_rx,
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
            ambient_light: 0.5,
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
        if let engine::events::Event::Tick = event {
            if let Ok(components) = self.draw_rx.try_recv() {
                if self.drawing.is_none() {
                    let drawing = Drawing::init(
                        graphics,
                        &components.terrain,
                        &components.tile_heights,
                        components.cliff_rise,
                    );
                    drawing.draw_geometry(graphics, &components.terrain);
                    self.drawing = Some(drawing);

                    sea::draw(&components.terrain, components.sea_level, graphics);

                    town::draw(&components.towns, &components.tile_heights, graphics);

                    self.handlers.resource_artist.init(graphics);
                    self.handlers.resource_artist.draw(
                        &components.resources,
                        &components.tile_heights,
                        graphics,
                    );
                }
                if let Some(drawing) = &self.drawing {
                    let traffic = components.traffic.map(|_, value| *value as f32).normalize();
                    let overlay = components.roads.map(|xy, is_road| {
                        if *is_road {
                            Rgba::new(
                                ROAD.r,
                                ROAD.g,
                                ROAD.b,
                                127 + (traffic[xy] * 128.0).round() as u8,
                            )
                        } else {
                            // Rgba::new(0, 0, 0, 0)
                            Rgba::new(0, 0, 0, (traffic[xy] * 255.0).round() as u8)
                        }
                    });
                    let overlay = OriginGrid::new(xy(0, 0), overlay);

                    drawing.modify_overlay(graphics, &overlay).unwrap();
                }

                self.compute_tx.send(components).unwrap();
            }
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
