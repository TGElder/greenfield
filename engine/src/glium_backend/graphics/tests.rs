use std::env::temp_dir;
use std::f32::consts::PI;

use commons::color::Rgb;

use crate::engine::Engine;
use crate::events::{ButtonState, Event, EventHandler, KeyboardKey, MouseButton};
use crate::glium_backend::graphics;
use crate::graphics::elements::Quad;
use crate::graphics::projections::isometric;
use crate::handlers::{drag, resize, yaw, zoom};

use super::*;

fn cube_quads() -> Vec<Quad> {
    let la = [-0.5, -0.5, -0.5];
    let lb = [0.5, -0.5, -0.5];
    let lc = [0.5, 0.5, -0.5];
    let ld = [-0.5, 0.5, -0.5];
    let ua = [-0.5, -0.5, 0.5];
    let ub = [0.5, -0.5, 0.5];
    let uc = [0.5, 0.5, 0.5];
    let ud = [-0.5, 0.5, 0.5];

    vec![
        Quad {
            corners: [ld, lc, lb, la],
            color: Rgb::new(1.0, 0.0, 0.0),
        },
        Quad {
            corners: [ua, ub, uc, ud],
            color: Rgb::new(1.0, 0.0, 0.0),
        },
        Quad {
            corners: [ua, la, lb, ub],
            color: Rgb::new(0.0, 1.0, 0.0),
        },
        Quad {
            corners: [uc, lc, ld, ud],
            color: Rgb::new(0.0, 1.0, 0.0),
        },
        Quad {
            corners: [ub, lb, lc, uc],
            color: Rgb::new(0.0, 0.0, 1.0),
        },
        Quad {
            corners: [ud, ld, la, ua],
            color: Rgb::new(0.0, 0.0, 1.0),
        },
    ]
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
    graphics.add_quads(&cube_quads()).unwrap();
    graphics.render().unwrap();

    let temp_path = temp_dir().join("test.png");
    let temp_path = temp_path.to_str().unwrap();
    graphics.screenshot(temp_path).unwrap();

    // then
    let actual = image::open(temp_path).unwrap();
    let expected = image::open("test_resources/graphics/render_cube.png").unwrap();
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
    graphics.add_quads(&cube_quads()).unwrap();
    graphics.look_at(&[-0.5, -0.5, -0.5], &(192, 64));
    graphics.render().unwrap();

    let temp_path = temp_dir().join("test.png");
    let temp_path = temp_path.to_str().unwrap();
    graphics.screenshot(temp_path).unwrap();

    // then
    let actual = image::open(temp_path).unwrap();
    let expected = image::open("test_resources/graphics/look_at.png").unwrap();
    assert_eq!(actual, expected);

    // when
    graphics.look_at(&[-0.5, -0.5, -0.5], &(192, 64));
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

    graphics.add_quads(&cube_quads()).unwrap();
    graphics.render().unwrap();

    let mut drag_handler = drag::Handler::new();

    // when
    drag_handler.handle(
        &Event::MouseMoved((100, 150)),
        &mut MockEngine {},
        &mut graphics,
    );
    drag_handler.handle(
        &Event::MouseInput {
            button: MouseButton::Left,
            state: ButtonState::Pressed,
        },
        &mut MockEngine {},
        &mut graphics,
    );
    drag_handler.handle(
        &Event::MouseMoved((80, 170)),
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

    graphics.add_quads(&cube_quads()).unwrap();
    graphics.render().unwrap();

    let mut yaw_handler = yaw::Handler::new(yaw::Parameters {
        initial_angle: 5,
        angles: 16,
        key_plus: KeyboardKey::P,
        key_minus: KeyboardKey::M,
    });

    // when
    yaw_handler.handle(
        &Event::MouseMoved((100, 150)),
        &mut MockEngine {},
        &mut graphics,
    );
    yaw_handler.handle(
        &Event::KeyboardInput {
            key: KeyboardKey::P,
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
        &Event::MouseMoved((100, 150)),
        &mut MockEngine {},
        &mut graphics,
    );
    yaw_handler.handle(
        &Event::KeyboardInput {
            key: KeyboardKey::M,
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

    graphics.add_quads(&cube_quads()).unwrap();
    graphics.render().unwrap();

    let mut yaw_handler = zoom::Handler::new(zoom::Parameters {
        initial_level: 8,
        min_level: 7,
        max_level: 9,
        key_plus: KeyboardKey::P,
        key_minus: KeyboardKey::M,
    });

    // when
    yaw_handler.handle(
        &Event::MouseMoved((100, 150)),
        &mut MockEngine {},
        &mut graphics,
    );
    yaw_handler.handle(
        &Event::KeyboardInput {
            key: KeyboardKey::P,
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
    yaw_handler.handle(
        &Event::MouseMoved((100, 150)),
        &mut MockEngine {},
        &mut graphics,
    );
    yaw_handler.handle(
        &Event::KeyboardInput {
            key: KeyboardKey::M,
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

    graphics.add_quads(&cube_quads()).unwrap();

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
