use std::f32::consts::PI;
use std::time::Duration;

use commons::color::Rgb;
use commons::geometry::{xy, xyz, Rectangle, XYZ};
use commons::grid::Grid;
use commons::noise::simplex_noise;
use engine::engine::Engine;
use engine::events::{Event, EventHandler, KeyboardKey};
use engine::glium_backend;
use engine::graphics::elements::Quad;
use engine::graphics::projections::isometric;
use engine::graphics::Graphics;
use engine::handlers::{drag, resize, yaw, zoom};
use nalgebra::Vector3;
use terrain_gen::with_valleys::{heightmap_from_rises_with_valleys, ValleyParameters};

fn main() {
    let engine = glium_backend::engine::GliumEngine::new(
        Demo {
            frame: 0,
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
        },
        glium_backend::engine::Parameters {
            frame_duration: Duration::from_nanos(16_666_667),
        },
        glium_backend::graphics::Parameters {
            name: "Demo".to_string(),
            width: 512,
            height: 512,
            projection: Box::new(isometric::Projection::new(isometric::Parameters {
                projection: isometric::ProjectionParameters {
                    pitch: PI / 4.0,
                    yaw: PI * (5.0 / 8.0),
                },
                scale: isometric::ScaleParameters {
                    zoom: 2.0,
                    z_max: 1.0 / 32.0,
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

struct Demo {
    frame: u64,
    drag_handler: drag::Handler,
    resize_handler: resize::Handler,
    yaw_handler: yaw::Handler,
    zoom_handler: zoom::Handler,
}

impl EventHandler for Demo {
    fn handle(&mut self, event: &Event, engine: &mut dyn Engine, graphics: &mut dyn Graphics) {
        if self.frame == 0 {
            let terrain = get_heightmap();
            draw_terrain(&terrain, graphics);
            graphics.look_at(
                &xyz(
                    terrain.width() as f32 / 2.0,
                    terrain.height() as f32 / 2.0,
                    0.0,
                ),
                &xy(256, 256),
            );
        } else if self.frame == 1 {
            graphics.screenshot("screenshot.png").unwrap();
        }

        self.drag_handler.handle(event, engine, graphics);
        self.resize_handler.handle(event, engine, graphics);
        self.yaw_handler.handle(event, engine, graphics);
        self.zoom_handler.handle(event, engine, graphics);

        self.frame += 1;
    }
}

fn draw_terrain(terrain: &Grid<f32>, graphics: &mut dyn Graphics) {
    let slab_size = 256;
    let slabs = xy(
        (terrain.width() / slab_size) + 1,
        (terrain.height() / slab_size) + 1,
    );
    for x in 0..slabs.x {
        for y in 0..slabs.y {
            let slab = xy(x, y);
            let mut quads = Vec::with_capacity(
                (terrain.width() - 1) as usize * (terrain.height() - 1) as usize,
            );
            for x in 0..slab_size {
                let x = slab.x * slab_size + x;
                if x >= terrain.width() - 1 {
                    break;
                }
                for y in 0..slab_size {
                    let y = slab.y * slab_size + y;
                    if y >= terrain.height() - 1 {
                        break;
                    }
                    let corners = [xy(0, 0), xy(1, 0), xy(1, 1), xy(0, 1)]
                        .iter()
                        .map(|d| {
                            xyz(
                                (x + d.x) as f32,
                                (y + d.y) as f32,
                                terrain[xy(x + d.x, y + d.y)] * 32.0,
                            )
                        })
                        .collect::<Vec<_>>();

                    quads.push(Quad {
                        color: color(&corners),
                        corners: [corners[0], corners[1], corners[2], corners[3]],
                    });
                }
            }

            graphics.add_quads(&quads).unwrap();
        }
    }
}

fn get_heightmap() -> Grid<f32> {
    let power = 8;
    let weights = (0..power + 1)
        .map(|i| 1.0f32 / 1.125f32.powf((power - i) as f32))
        .collect::<Vec<_>>();
    let rises = simplex_noise(power, 1987, &weights)
        .normalize()
        .map(|_, z| (0.5 - z).abs() / 0.5);

    heightmap_from_rises_with_valleys(
        &rises,
        ValleyParameters {
            height_threshold: 0.25,
            rain_threshold: 128,
            rise: 0.01,
            origin_fn: |xy| rises.is_border(xy),
        },
    )
}

fn color(corners: &[XYZ<f32>]) -> Rgb<f32> {
    let light_direction: Vector3<f32> = Vector3::new(1.0, 0.0, 0.0);
    let base_color: Rgb<f32> = Rgb::new(1.0, 1.0, 1.0);

    let corners = corners
        .iter()
        .map(|XYZ { x, y, z }| Vector3::new(*x, *y, *z))
        .collect::<Vec<_>>();

    let u = corners[0] - corners[2];
    let v = corners[1] - corners[3];
    let normal = u.cross(&v);
    let angle = normal.angle(&light_direction);
    let shade = angle / PI;
    Rgb::new(
        base_color.r * shade,
        base_color.g * shade,
        base_color.b * shade,
    )
}
