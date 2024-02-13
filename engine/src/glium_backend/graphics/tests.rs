use std::env::temp_dir;
use std::f32::consts::PI;

use commons::color::Rgb;
use commons::geometry::Rectangle;
use nalgebra::Matrix4;

use crate::binding::Binding;
use crate::engine::Engine;
use crate::events::{Button, ButtonState, Event, EventHandler, KeyboardKey, MouseButton};
use crate::glium_backend::graphics;
use crate::graphics::elements::{OverlayQuads, Quad};
use crate::graphics::models::cube;
use crate::graphics::projections::isometric;
use crate::graphics::transform::Transform;
use crate::handlers::{drag, resize, yaw, zoom};

use super::*;

fn cube_quads() -> [Quad; 6] {
    cube::model(&|side| match side {
        cube::Side::Left => Rgb::new(1.0, 1.0, 0.0),
        cube::Side::Right => Rgb::new(0.0, 0.0, 1.0),
        cube::Side::Back => Rgb::new(1.0, 0.0, 1.0),
        cube::Side::Front => Rgb::new(0.0, 1.0, 0.0),
        cube::Side::Bottom => Rgb::new(0.0, 1.0, 1.0),
        cube::Side::Top => Rgb::new(1.0, 0.0, 0.0),
    })
}

#[test]
fn render_cube() {
    // given
    let mut graphics = GliumGraphics::headless(graphics::Parameters {
        name: "Test".to_string(),
        width: 256,
        height: 256,
        projection: Box::new(isometric::Projection::new(isometric::Parameters {
            projection: isometric::ProjectionParameters {
                pitch: PI / 4.0,
                yaw: PI * (5.0 / 8.0),
            },
            scale: isometric::ScaleParameters {
                zoom: 256.0,
                z_max: 1.0,
                viewport: Rectangle {
                    width: 256,
                    height: 256,
                },
            },
        })),
    })
    .unwrap();

    // when
    let index = graphics.create_quads().unwrap();
    let quads = cube_quads();
    graphics.draw_quads(&index, &quads).unwrap();
    graphics.render().unwrap();

    let temp_path = temp_dir().join("test.png");
    let temp_path = temp_path.to_str().unwrap();
    graphics.screenshot(temp_path).unwrap();

    // then
    let actual = image::open(temp_path).unwrap();
    let expected = image::open("test_resources/graphics/render_cube.png").unwrap();
    assert_eq!(actual, expected);

    // when
    let roll: Matrix4<f32> = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, -1.0, 0.0, 0.0],
        [0.0, 0.0, -1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
    .into();
    let yaw: Matrix4<f32> = [
        [0.0, -1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
    .into();

    let transformation = yaw * roll;

    let rear_facing_quads = quads
        .iter()
        .map(|quad| quad.transform(&transformation))
        .collect::<Vec<_>>();
    graphics.draw_quads(&index, &rear_facing_quads).unwrap();
    graphics.render().unwrap();

    let temp_path = temp_dir().join("test.png");
    let temp_path = temp_path.to_str().unwrap();
    graphics.screenshot(temp_path).unwrap();

    // then
    let actual = image::open(temp_path).unwrap();
    let expected = image::open("test_resources/graphics/render_cube_rear.png").unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn instanced_cubes() {
    // given
    let mut graphics = GliumGraphics::headless(graphics::Parameters {
        name: "Test".to_string(),
        width: 256,
        height: 256,
        projection: Box::new(isometric::Projection::new(isometric::Parameters {
            projection: isometric::ProjectionParameters {
                pitch: PI / 4.0,
                yaw: PI * (5.0 / 8.0),
            },
            scale: isometric::ScaleParameters {
                zoom: 256.0,
                z_max: 1.0,
                viewport: Rectangle {
                    width: 256,
                    height: 256,
                },
            },
        })),
    })
    .unwrap();

    // when
    let index = graphics.create_instanced_triangles().unwrap();
    let quads = cube_quads();

    let shrink: Matrix4<f32> = [
        [0.5, 0.0, 0.0, 0.0],
        [0.0, 0.5, 0.0, 0.0],
        [0.0, 0.0, 0.5, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
    .into();
    let left: Matrix4<f32> = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [-0.5, 0.0, 0.0, 1.0],
    ]
    .into();
    let right: Matrix4<f32> = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.5, 0.0, 0.0, 1.0],
    ]
    .into();
    let identity = Matrix4::identity();

    graphics
        .draw_instanced_quads(
            &index,
            &quads,
            &[
                left * shrink * identity,  //
                right * shrink * identity, //
            ],
        )
        .unwrap();

    graphics.render().unwrap();

    let temp_path = temp_dir().join("test.png");
    let temp_path = temp_path.to_str().unwrap();
    graphics.screenshot(temp_path).unwrap();

    // then
    let actual = image::open(temp_path).unwrap();
    let expected = image::open("test_resources/graphics/instance_cubes.png").unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn render_billboard() {
    // given
    let mut graphics = GliumGraphics::headless(graphics::Parameters {
        name: "Test".to_string(),
        width: 256,
        height: 256,
        projection: Box::new(isometric::Projection::new(isometric::Parameters {
            projection: isometric::ProjectionParameters {
                pitch: PI / 4.0,
                yaw: PI * (5.0 / 8.0),
            },
            scale: isometric::ScaleParameters {
                zoom: 256.0,
                z_max: 1.0,
                viewport: Rectangle {
                    width: 256,
                    height: 256,
                },
            },
        })),
    })
    .unwrap();

    // when
    let texture = graphics
        .load_texture_from_file("test_resources/graphics/crab.png")
        .unwrap();
    let billboard = elements::Billboard {
        position: xyz(0.0, 0.0, 0.0),
        dimensions: Rectangle {
            width: 1.0,
            height: 1.0,
        },
        texture,
    };
    let index = graphics.create_billboards().unwrap();
    graphics.draw_billboard(&index, &billboard).unwrap();
    let index = graphics.create_quads().unwrap();
    graphics
        .draw_quads(
            &index,
            &[Quad {
                corners: [
                    xyz(-0.5, -0.5, 0.0),
                    xyz(0.5, -0.5, 0.0),
                    xyz(0.5, 0.5, 0.0),
                    xyz(-0.5, 0.5, 0.0),
                ],
                color: Rgb::new(0.0, 0.0, 1.0),
            }],
        )
        .unwrap();
    graphics.render().unwrap();

    let temp_path = temp_dir().join("test.png");
    let temp_path = temp_path.to_str().unwrap();
    graphics.screenshot(temp_path).unwrap();

    // then
    let actual = image::open(temp_path).unwrap();
    let expected = image::open("test_resources/graphics/render_billboard.png").unwrap();
    let difference = commons::image::difference(&actual, &expected).unwrap();
    let max_difference = (256 * 256 * (255 * 3)) / 1000;

    assert!(difference < max_difference);
}

#[test]
fn render_overlay_quads() {
    // given
    let mut graphics = GliumGraphics::headless(graphics::Parameters {
        name: "Test".to_string(),
        width: 256,
        height: 256,
        projection: Box::new(isometric::Projection::new(isometric::Parameters {
            projection: isometric::ProjectionParameters {
                pitch: PI / 4.0,
                yaw: PI * (1.0 / 8.0),
            },
            scale: isometric::ScaleParameters {
                zoom: 256.0,
                z_max: 1.0,
                viewport: Rectangle {
                    width: 256,
                    height: 256,
                },
            },
        })),
    })
    .unwrap();

    // when
    let base_texture = graphics
        .load_texture_from_file("test_resources/graphics/overlay_quads_base.png")
        .unwrap();
    let overlay_texture = graphics
        .load_texture(&Grid::from_vec(
            2,
            2,
            vec![
                Rgba::new(255, 255, 0, 255),
                Rgba::new(0, 0, 0, 0),
                Rgba::new(0, 0, 0, 0),
                Rgba::new(255, 255, 0, 255),
            ],
        ))
        .unwrap();

    fn textured_position(position: XYZ<f32>) -> TexturedPosition {
        TexturedPosition {
            position,
            texture_coordinates: xy(position.x + 0.5, position.y + 0.5),
        }
    }

    let aa = textured_position(xyz(-0.5, -0.5, 0.0));
    let ba = textured_position(xyz(0.0, -0.5, 0.0));
    let ca = textured_position(xyz(0.5, -0.5, 0.0));
    let ab = textured_position(xyz(-0.5, 0.0, 0.0));
    let bb = textured_position(xyz(0.0, 0.0, 0.5));
    let cb = textured_position(xyz(0.5, 0.0, 0.0));
    let ac = textured_position(xyz(-0.5, 0.5, 0.0));
    let bc = textured_position(xyz(0.0, 0.5, 0.0));
    let cc = textured_position(xyz(0.5, 0.5, 0.0));
    let quads = vec![
        [aa, ba, bb, ab],
        [ba, ca, cb, bb],
        [ab, bb, bc, ac],
        [bb, cb, cc, bc],
    ];

    let overlay_quads = OverlayQuads {
        base_texture,
        overlay_texture,
        quads,
    };
    let index = graphics.create_overlay_quads().unwrap();
    graphics.draw_overlay_quads(&index, &overlay_quads).unwrap();
    graphics.render().unwrap();

    let temp_dir = temp_dir();
    let temp_path = temp_dir.join("render_overlay_quads.png");
    let temp_path = temp_path.to_str().unwrap();
    graphics.screenshot(temp_path).unwrap();

    // then
    let actual = image::open(temp_path).unwrap();
    let expected = image::open("test_resources/graphics/render_overlay_quads.png").unwrap();
    assert_eq!(actual, expected);

    // when
    graphics
        .modify_texture(
            &overlay_texture,
            &OriginGrid::new(
                xy(0, 1),
                Grid::from_vec(
                    2,
                    1,
                    vec![Rgba::new(255, 255, 0, 255), Rgba::new(255, 255, 0, 255)],
                ),
            ),
        )
        .unwrap();
    graphics
        .modify_texture(
            &overlay_texture,
            &OriginGrid::new(
                xy(0, 0),
                Grid::from_vec(
                    1,
                    2,
                    vec![Rgba::new(255, 255, 0, 255), Rgba::new(255, 255, 0, 255)],
                ),
            ),
        )
        .unwrap();
    graphics.render().unwrap();

    let temp_path = temp_dir.join("render_overlay_quads_modified.png");
    let temp_path = temp_path.to_str().unwrap();
    graphics.screenshot(temp_path).unwrap();

    // then
    let actual = image::open(temp_path).unwrap();
    let expected =
        image::open("test_resources/graphics/render_overlay_quads_modified.png").unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn look_at() {
    // given
    let mut graphics = GliumGraphics::headless(graphics::Parameters {
        name: "Test".to_string(),
        width: 256,
        height: 256,
        projection: Box::new(isometric::Projection::new(isometric::Parameters {
            projection: isometric::ProjectionParameters {
                pitch: PI / 4.0,
                yaw: PI * (5.0 / 8.0),
            },
            scale: isometric::ScaleParameters {
                zoom: 256.0,
                z_max: 1.0,
                viewport: Rectangle {
                    width: 256,
                    height: 256,
                },
            },
        })),
    })
    .unwrap();

    // when
    let index = graphics.create_quads().unwrap();
    graphics.draw_quads(&index, &cube_quads()).unwrap();
    graphics.look_at(&xyz(-0.5, -0.5, -0.5), &xy(192, 64));
    graphics.render().unwrap();

    let temp_path = temp_dir().join("test.png");
    let temp_path = temp_path.to_str().unwrap();
    graphics.screenshot(temp_path).unwrap();

    // then
    let actual = image::open(temp_path).unwrap();
    let expected = image::open("test_resources/graphics/look_at.png").unwrap();
    assert_eq!(actual, expected);

    // when
    graphics.look_at(&xyz(-0.5, -0.5, -0.5), &xy(192, 64));
    graphics.render().unwrap();
    graphics.screenshot(temp_path).unwrap();

    // then
    let actual = image::open(temp_path).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn drag_handler() {
    struct MockEngine {}

    impl Engine for MockEngine {
        fn shutdown(&mut self) {}
    }

    // given
    let mut graphics = GliumGraphics::headless(graphics::Parameters {
        name: "Test".to_string(),
        width: 256,
        height: 256,
        projection: Box::new(isometric::Projection::new(isometric::Parameters {
            projection: isometric::ProjectionParameters {
                pitch: PI / 4.0,
                yaw: PI * (5.0 / 8.0),
            },
            scale: isometric::ScaleParameters {
                zoom: 256.0,
                z_max: 1.0,
                viewport: Rectangle {
                    width: 256,
                    height: 256,
                },
            },
        })),
    })
    .unwrap();

    let index = graphics.create_quads().unwrap();
    graphics.draw_quads(&index, &cube_quads()).unwrap();
    graphics.render().unwrap();

    let mut drag_handler = drag::Handler::new(drag::Bindings {
        start_dragging: Binding::Single {
            button: Button::Mouse(MouseButton::Left),
            state: ButtonState::Pressed,
        },
        stop_dragging: Binding::Single {
            button: Button::Mouse(MouseButton::Left),
            state: ButtonState::Released,
        },
    });

    // when
    drag_handler.handle(
        &Event::MouseMoved(xy(100, 150)),
        &mut MockEngine {},
        &mut graphics,
    );
    drag_handler.handle(
        &Event::Button {
            button: Button::Mouse(MouseButton::Left),
            state: ButtonState::Pressed,
        },
        &mut MockEngine {},
        &mut graphics,
    );
    drag_handler.handle(
        &Event::MouseMoved(xy(80, 170)),
        &mut MockEngine {},
        &mut graphics,
    );
    graphics.render().unwrap();

    let temp_path = temp_dir().join("test.png");
    let temp_path = temp_path.to_str().unwrap();
    graphics.screenshot(temp_path).unwrap();

    // then
    let actual = image::open(temp_path).unwrap();
    let expected = image::open("test_resources/graphics/drag_handler.png").unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn yaw_handler() {
    struct MockEngine {}

    impl Engine for MockEngine {
        fn shutdown(&mut self) {}
    }

    // given
    let mut graphics = GliumGraphics::headless(graphics::Parameters {
        name: "Test".to_string(),
        width: 256,
        height: 256,
        projection: Box::new(isometric::Projection::new(isometric::Parameters {
            projection: isometric::ProjectionParameters {
                pitch: PI / 4.0,
                yaw: PI * (5.0 / 8.0),
            },
            scale: isometric::ScaleParameters {
                zoom: 256.0,
                z_max: 1.0,
                viewport: Rectangle {
                    width: 256,
                    height: 256,
                },
            },
        })),
    })
    .unwrap();

    let index = graphics.create_quads().unwrap();
    graphics.draw_quads(&index, &cube_quads()).unwrap();
    graphics.render().unwrap();

    let mut yaw_handler = yaw::Handler::new(yaw::Parameters {
        initial_angle: 5,
        angles: 16,
        bindings: yaw::Bindings {
            plus: Binding::Single {
                button: Button::Keyboard(KeyboardKey::Plus),
                state: ButtonState::Pressed,
            },
            minus: Binding::Single {
                button: Button::Keyboard(KeyboardKey::Minus),
                state: ButtonState::Pressed,
            },
        },
    });

    // when
    yaw_handler.handle(
        &Event::MouseMoved(xy(100, 150)),
        &mut MockEngine {},
        &mut graphics,
    );
    yaw_handler.handle(
        &Event::Button {
            button: Button::Keyboard(KeyboardKey::Plus),
            state: ButtonState::Pressed,
        },
        &mut MockEngine {},
        &mut graphics,
    );
    graphics.render().unwrap();

    let temp_path = temp_dir().join("test.png");
    let temp_path = temp_path.to_str().unwrap();
    graphics.screenshot(temp_path).unwrap();

    // then
    let actual = image::open(temp_path).unwrap();
    let expected = image::open("test_resources/graphics/yaw_handler_1.png").unwrap();
    assert_eq!(actual, expected);

    // when
    yaw_handler.handle(
        &Event::MouseMoved(xy(100, 150)),
        &mut MockEngine {},
        &mut graphics,
    );
    yaw_handler.handle(
        &Event::Button {
            button: Button::Keyboard(KeyboardKey::Minus),
            state: ButtonState::Pressed,
        },
        &mut MockEngine {},
        &mut graphics,
    );
    graphics.render().unwrap();

    let temp_path = temp_dir().join("test.png");
    let temp_path = temp_path.to_str().unwrap();
    graphics.screenshot(temp_path).unwrap();

    // then
    let actual = image::open(temp_path).unwrap();
    let expected = image::open("test_resources/graphics/yaw_handler_2.png").unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn zoom_handler() {
    struct MockEngine {}

    impl Engine for MockEngine {
        fn shutdown(&mut self) {}
    }

    // given
    let mut graphics = GliumGraphics::headless(graphics::Parameters {
        name: "Test".to_string(),
        width: 256,
        height: 256,
        projection: Box::new(isometric::Projection::new(isometric::Parameters {
            projection: isometric::ProjectionParameters {
                pitch: PI / 4.0,
                yaw: PI * (5.0 / 8.0),
            },
            scale: isometric::ScaleParameters {
                zoom: 256.0,
                z_max: 1.0,
                viewport: Rectangle {
                    width: 256,
                    height: 256,
                },
            },
        })),
    })
    .unwrap();

    let index = graphics.create_quads().unwrap();
    graphics.draw_quads(&index, &cube_quads()).unwrap();
    graphics.render().unwrap();

    let mut zoom_handler = zoom::Handler::new(zoom::Parameters {
        initial_level: 8,
        min_level: 7,
        max_level: 9,
        bindings: zoom::Bindings {
            plus: Binding::Single {
                button: Button::Keyboard(KeyboardKey::Plus),
                state: ButtonState::Pressed,
            },
            minus: Binding::Single {
                button: Button::Keyboard(KeyboardKey::Minus),
                state: ButtonState::Pressed,
            },
        },
    });

    // when
    zoom_handler.handle(
        &Event::MouseMoved(xy(100, 150)),
        &mut MockEngine {},
        &mut graphics,
    );
    zoom_handler.handle(
        &Event::Button {
            button: Button::Keyboard(KeyboardKey::Plus),
            state: ButtonState::Pressed,
        },
        &mut MockEngine {},
        &mut graphics,
    );
    graphics.render().unwrap();

    let temp_path = temp_dir().join("test.png");
    let temp_path = temp_path.to_str().unwrap();
    graphics.screenshot(temp_path).unwrap();

    // then
    let actual = image::open(temp_path).unwrap();
    let expected = image::open("test_resources/graphics/zoom_handler_1.png").unwrap();
    assert_eq!(actual, expected);

    // when
    zoom_handler.handle(
        &Event::MouseMoved(xy(100, 150)),
        &mut MockEngine {},
        &mut graphics,
    );
    zoom_handler.handle(
        &Event::Button {
            button: Button::Keyboard(KeyboardKey::Minus),
            state: ButtonState::Pressed,
        },
        &mut MockEngine {},
        &mut graphics,
    );
    graphics.render().unwrap();

    let temp_path = temp_dir().join("test.png");
    let temp_path = temp_path.to_str().unwrap();
    graphics.screenshot(temp_path).unwrap();

    // then
    let actual = image::open(temp_path).unwrap();
    let expected = image::open("test_resources/graphics/zoom_handler_2.png").unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn resize_handler() {
    struct MockEngine {}

    impl Engine for MockEngine {
        fn shutdown(&mut self) {}
    }

    // given
    let mut graphics = GliumGraphics::headless(graphics::Parameters {
        name: "Test".to_string(),
        width: 512,
        height: 256,
        projection: Box::new(isometric::Projection::new(isometric::Parameters {
            projection: isometric::ProjectionParameters {
                pitch: PI / 4.0,
                yaw: PI * (5.0 / 8.0),
            },
            scale: isometric::ScaleParameters {
                zoom: 256.0,
                z_max: 1.0,
                viewport: Rectangle {
                    width: 256,
                    height: 256,
                },
            },
        })),
    })
    .unwrap();

    let index = graphics.create_quads().unwrap();
    graphics.draw_quads(&index, &cube_quads()).unwrap();

    let mut resize_hander = resize::Handler::new();

    // when
    resize_hander.handle(
        &Event::WindowResize(Rectangle {
            width: 512,
            height: 256,
        }),
        &mut MockEngine {},
        &mut graphics,
    );
    graphics.render().unwrap();

    let temp_path = temp_dir().join("test.png");
    let temp_path = temp_path.to_str().unwrap();
    graphics.screenshot(temp_path).unwrap();

    // then
    let actual = image::open(temp_path).unwrap();
    let expected = image::open("test_resources/graphics/resize_handler.png").unwrap();
    assert_eq!(actual, expected);
}
