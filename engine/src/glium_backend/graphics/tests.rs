use std::env::temp_dir;
use std::f32::consts::PI;

use commons::color::Rgb;
use commons::geometry::Rectangle;
use winit::event_loop::EventLoop;
use winit::platform::x11::EventLoopBuilderExtX11;

use crate::binding::Binding;
use crate::engine::Engine;
use crate::events::{Button, ButtonState, Event, KeyboardKey, MouseButton};
use crate::glium_backend::graphics;
use crate::graphics::elements::Quad;
use crate::graphics::models::cube;
use crate::graphics::projections::isometric;
use crate::graphics::transform::{Recolor, Transform};
use crate::graphics::utils::{
    quad_normal, textured_triangles_from_textured_quads, transformation_matrix,
    triangles_from_quads, Transformation,
};
use crate::handlers::{self, drag, yaw, zoom};

use super::*;

fn event_loop() -> EventLoop<()> {
    let event_loop: EventLoop<()> = winit::event_loop::EventLoopBuilder::new()
        .with_any_thread(true)
        .build()
        .unwrap();
    event_loop
}

fn cube_triangles() -> Vec<Triangle<Rgb<f32>>> {
    let quads = cube::model().recolor(&|side| match side {
        cube::Side::Left => Rgb::new(1.0, 1.0, 0.0),
        cube::Side::Right => Rgb::new(0.0, 0.0, 1.0),
        cube::Side::Back => Rgb::new(1.0, 0.0, 1.0),
        cube::Side::Front => Rgb::new(0.0, 1.0, 0.0),
        cube::Side::Bottom => Rgb::new(0.0, 1.0, 1.0),
        cube::Side::Top => Rgb::new(1.0, 0.0, 0.0),
    });
    triangles_from_quads(&quads)
}

#[test]
fn render_cube() {
    // given
    let event_loop = event_loop();
    let mut graphics = GliumGraphics::new(
        graphics::Parameters {
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
            light_direction: xyz(-1.0, 0.0, 0.0),
        },
        &event_loop,
    )
    .unwrap();

    // when
    let index = graphics.create_triangles().unwrap();
    let triangles = cube_triangles();
    graphics.draw_triangles(&index, &triangles).unwrap();
    graphics.render().unwrap();

    let temp_path = temp_dir().join("test.png");
    let temp_path = temp_path.to_str().unwrap();
    graphics.screenshot(temp_path).unwrap();

    // then
    let actual = image::open(temp_path).unwrap();
    let expected = image::open("test_resources/graphics/render_cube.png").unwrap();
    assert_eq!(actual, expected);

    // when
    let rear_facing_triangles = triangles.transform(&transformation_matrix(Transformation {
        yaw: Some(-PI / 2.0),
        roll: Some(PI),
        ..Transformation::default()
    }));

    graphics
        .draw_triangles(&index, &rear_facing_triangles)
        .unwrap();
    graphics.render().unwrap();

    let temp_path = temp_dir().join("test.png");
    let temp_path = temp_path.to_str().unwrap();
    graphics.screenshot(temp_path).unwrap();

    // then
    let actual = image::open(temp_path).unwrap();
    let expected = image::open("test_resources/graphics/render_cube_rear.png").unwrap();
    assert_eq!(actual, expected);

    // finally
    clear(&mut graphics);
    event_loop.exit();
}

#[test]
fn render_cube_dynamic() {
    // given
    let event_loop = event_loop();
    let mut graphics = GliumGraphics::new(
        graphics::Parameters {
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
            light_direction: xyz(-1.0, 0.0, 0.0),
        },
        &event_loop,
    )
    .unwrap();

    // when
    let triangles = cube_triangles();
    let triangles = triangles.transform(&transformation_matrix(Transformation {
        scale: Some(xyz(0.5, 0.5, 0.5)),
        ..Transformation::default()
    }));

    let index_1 = graphics.create_dynamic_triangles(&triangles.len()).unwrap();
    let index_2 = graphics.create_dynamic_triangles(&triangles.len()).unwrap();
    graphics
        .update_dynamic_triangles(&index_1, Some(&triangles))
        .unwrap();
    graphics.render().unwrap();

    let temp_path = temp_dir().join("test.png");
    let temp_path = temp_path.to_str().unwrap();
    graphics.screenshot(temp_path).unwrap();

    // then
    let actual = image::open(temp_path).unwrap();
    let expected = image::open("test_resources/graphics/render_cube_dynamic.png").unwrap();
    assert_eq!(actual, expected);

    // when
    let triangles_left = triangles.transform(&transformation_matrix(Transformation {
        translation: Some(xyz(-0.5, 0.0, 0.0)),
        ..Transformation::default()
    }));
    let triangles_right = triangles.transform(&transformation_matrix(Transformation {
        translation: Some(xyz(0.5, 0.0, 0.0)),
        ..Transformation::default()
    }));

    graphics
        .update_dynamic_triangles(&index_1, Some(&triangles_left))
        .unwrap();
    graphics
        .update_dynamic_triangles(&index_2, Some(&triangles_right))
        .unwrap();
    graphics.render().unwrap();

    let temp_path = temp_dir().join("test.png");
    let temp_path = temp_path.to_str().unwrap();
    graphics.screenshot(temp_path).unwrap();

    // then
    let actual = image::open(temp_path).unwrap();
    let expected =
        image::open("test_resources/graphics/render_cube_dynamic_two_cubes.png").unwrap();
    assert_eq!(actual, expected);

    // when
    graphics.update_dynamic_triangles(&index_1, None).unwrap();
    graphics.render().unwrap();

    let temp_path = temp_dir().join("test.png");
    let temp_path = temp_path.to_str().unwrap();
    graphics.screenshot(temp_path).unwrap();

    // then
    let actual = image::open(temp_path).unwrap();
    let expected =
        image::open("test_resources/graphics/render_cube_dynamic_one_cube_invisible.png").unwrap();
    assert_eq!(actual, expected);

    // finally
    clear(&mut graphics);
    event_loop.exit();
}

#[test]
fn instanced_cubes() {
    // given
    let event_loop = event_loop();
    let mut graphics = GliumGraphics::new(
        graphics::Parameters {
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
            light_direction: xyz(-1.0, 0.0, 0.0),
        },
        &event_loop,
    )
    .unwrap();

    // when
    let triangles = cube_triangles();
    let index = graphics.create_instanced_triangles(&triangles).unwrap();

    let left_transformation = transformation_matrix(Transformation {
        translation: Some(xyz(-0.5, 0.0, 0.0)),
        scale: Some(xyz(0.5, 0.5, 0.5)),
        ..Transformation::default()
    });
    let right_transformation = transformation_matrix(Transformation {
        translation: Some(xyz(0.5, 0.0, 0.0)),
        scale: Some(xyz(0.5, 0.5, 0.5)),
        ..Transformation::default()
    });

    graphics
        .update_instanced_triangles(
            &index,
            &[Some(left_transformation), Some(right_transformation)],
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

    // when
    graphics
        .update_instanced_triangles(
            &index,
            &[
                None,                       //
                Some(right_transformation), //
            ],
        )
        .unwrap();

    graphics.render().unwrap();

    let temp_path = temp_dir().join("test.png");
    let temp_path = temp_path.to_str().unwrap();
    graphics.screenshot(temp_path).unwrap();

    // then
    let actual = image::open(temp_path).unwrap();
    let expected =
        image::open("test_resources/graphics/instance_cubes_with_invisible_cube.png").unwrap();
    assert_eq!(actual, expected);

    // when
    graphics
        .update_instanced_triangles(
            &index,
            &[
                Some(right_transformation), //
            ],
        )
        .unwrap();

    graphics.render().unwrap();

    let temp_path = temp_dir().join("test.png");
    let temp_path = temp_path.to_str().unwrap();
    graphics.screenshot(temp_path).unwrap();

    // then
    let actual = image::open(temp_path).unwrap();
    let expected =
        image::open("test_resources/graphics/instance_cubes_with_single_cube.png").unwrap();
    assert_eq!(actual, expected);

    // finally
    clear(&mut graphics);
    event_loop.exit();
}

#[test]
fn render_billboard() {
    // given
    let event_loop = event_loop();
    let mut graphics = GliumGraphics::new(
        graphics::Parameters {
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
            light_direction: xyz(-1.0, 0.0, 0.0),
        },
        &event_loop,
    )
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
    let index = graphics.create_triangles().unwrap();
    let triangles = triangles_from_quads(&[Quad {
        corners: [
            xyz(-0.5, -0.5, 0.0),
            xyz(0.5, -0.5, 0.0),
            xyz(0.5, 0.5, 0.0),
            xyz(-0.5, 0.5, 0.0),
        ],
        color: Rgb::new(0.0, 0.0, 1.0),
    }]);
    graphics.draw_triangles(&index, &triangles).unwrap();
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

    // finally
    clear(&mut graphics);
    event_loop.exit();
}

#[test]
fn render_overlay_quads() {
    // given
    let event_loop = event_loop();
    let mut graphics = GliumGraphics::new(
        graphics::Parameters {
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
            light_direction: xyz(-1.0, 0.0, 0.0),
        },
        &event_loop,
    )
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

    fn textured_position(position: XYZ<f32>, normal: XYZ<f32>) -> TexturedPosition {
        TexturedPosition {
            position,
            normal,
            texture_coordinates: xy(position.x + 0.5, position.y + 0.5),
        }
    }

    let aa = xyz(-0.5, -0.5, 0.0);
    let ba = xyz(0.0, -0.5, 0.0);
    let ca = xyz(0.5, -0.5, 0.0);
    let ab = xyz(-0.5, 0.0, 0.0);
    let bb = xyz(0.0, 0.0, 0.5);
    let cb = xyz(0.5, 0.0, 0.0);
    let ac = xyz(-0.5, 0.5, 0.0);
    let bc = xyz(0.0, 0.5, 0.0);
    let cc = xyz(0.5, 0.5, 0.0);
    let quads = [
        [aa, ba, bb, ab],
        [ba, ca, cb, bb],
        [ab, bb, bc, ac],
        [bb, cb, cc, bc],
    ]
    .into_iter()
    .map(|quad| {
        let normal = quad_normal(&quad);
        [
            textured_position(quad[0], normal),
            textured_position(quad[1], normal),
            textured_position(quad[2], normal),
            textured_position(quad[3], normal),
        ]
    })
    .collect::<Vec<_>>();

    let overlay_triangles = OverlayTriangles {
        base_texture,
        overlay_texture,
        triangles: textured_triangles_from_textured_quads(&quads),
    };
    let index = graphics.create_overlay_triangles().unwrap();
    graphics
        .draw_overlay_triangles(&index, &overlay_triangles)
        .unwrap();
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

    // finally
    clear(&mut graphics);
    event_loop.exit();
}

#[test]
fn look_at() {
    // given
    let event_loop = event_loop();
    let mut graphics = GliumGraphics::new(
        graphics::Parameters {
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
            light_direction: xyz(-1.0, 0.0, 0.0),
        },
        &event_loop,
    )
    .unwrap();

    // when
    let index = graphics.create_triangles().unwrap();
    let triangles = cube_triangles();
    graphics.draw_triangles(&index, &triangles).unwrap();
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

    // finally
    clear(&mut graphics);
    event_loop.exit();
}

#[test]
fn drag_handler() {
    struct MockEngine {}

    impl Engine for MockEngine {
        fn shutdown(&mut self) {}
    }

    // given
    let event_loop = event_loop();
    let mut graphics = GliumGraphics::new(
        graphics::Parameters {
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
            light_direction: xyz(-1.0, 0.0, 0.0),
        },
        &event_loop,
    )
    .unwrap();

    let index = graphics.create_triangles().unwrap();
    let triangles = cube_triangles();
    graphics.draw_triangles(&index, &triangles).unwrap();
    graphics.render().unwrap();

    let mut drag_handler = drag::Handler::default();
    let bindings = drag::Bindings {
        start_dragging: Binding::Single {
            button: Button::Mouse(MouseButton::Left),
            state: ButtonState::Pressed,
        },
        stop_dragging: Binding::Single {
            button: Button::Mouse(MouseButton::Left),
            state: ButtonState::Released,
        },
    };

    // when
    drag_handler.handle(
        &bindings,
        &Event::MouseMoved(xy(100, 150)),
        &mut MockEngine {},
        &mut graphics,
    );
    drag_handler.handle(
        &bindings,
        &Event::Button {
            button: Button::Mouse(MouseButton::Left),
            state: ButtonState::Pressed,
        },
        &mut MockEngine {},
        &mut graphics,
    );
    drag_handler.handle(
        &bindings,
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

    // finally
    clear(&mut graphics);
    event_loop.exit();
}

#[test]
fn yaw_handler() {
    struct MockEngine {}

    impl Engine for MockEngine {
        fn shutdown(&mut self) {}
    }

    // given
    let event_loop = event_loop();
    let mut graphics = GliumGraphics::new(
        graphics::Parameters {
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
            light_direction: xyz(-1.0, 0.0, 0.0),
        },
        &event_loop,
    )
    .unwrap();

    let index = graphics.create_triangles().unwrap();
    let triangles = cube_triangles();
    graphics.draw_triangles(&index, &triangles).unwrap();
    graphics.render().unwrap();

    let mut yaw_handler = yaw::Handler::new(yaw::Parameters {
        initial_angle: 5,
        angles: 16,
    });
    let bindings = yaw::Bindings {
        plus: Binding::Single {
            button: Button::Keyboard(KeyboardKey::from("+")),
            state: ButtonState::Pressed,
        },
        minus: Binding::Single {
            button: Button::Keyboard(KeyboardKey::from("-")),
            state: ButtonState::Pressed,
        },
    };

    // when
    yaw_handler.handle(
        &bindings,
        &Event::MouseMoved(xy(100, 150)),
        &mut MockEngine {},
        &mut graphics,
    );
    yaw_handler.handle(
        &bindings,
        &Event::Button {
            button: Button::Keyboard(KeyboardKey::from("+")),
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
        &bindings,
        &Event::MouseMoved(xy(100, 150)),
        &mut MockEngine {},
        &mut graphics,
    );
    yaw_handler.handle(
        &bindings,
        &Event::Button {
            button: Button::Keyboard(KeyboardKey::from("-")),
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

    // finally
    clear(&mut graphics);
    event_loop.exit();
}

#[test]
fn zoom_handler() {
    struct MockEngine {}

    impl Engine for MockEngine {
        fn shutdown(&mut self) {}
    }

    // given
    let event_loop = event_loop();
    let mut graphics = GliumGraphics::new(
        graphics::Parameters {
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
            light_direction: xyz(-1.0, 0.0, 0.0),
        },
        &event_loop,
    )
    .unwrap();

    let index = graphics.create_triangles().unwrap();
    let triangles = cube_triangles();
    graphics.draw_triangles(&index, &triangles).unwrap();
    graphics.render().unwrap();

    let mut zoom_handler = zoom::Handler::new(zoom::Parameters {
        initial_level: 8,
        min_level: 7,
        max_level: 9,
    });
    let bindings = zoom::Bindings {
        plus: Binding::Single {
            button: Button::Keyboard(KeyboardKey::from("+")),
            state: ButtonState::Pressed,
        },
        minus: Binding::Single {
            button: Button::Keyboard(KeyboardKey::from("-")),
            state: ButtonState::Pressed,
        },
    };

    // when
    zoom_handler.handle(
        &bindings,
        &Event::MouseMoved(xy(100, 150)),
        &mut MockEngine {},
        &mut graphics,
    );
    zoom_handler.handle(
        &bindings,
        &Event::Button {
            button: Button::Keyboard(KeyboardKey::from("+")),
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
        &bindings,
        &Event::MouseMoved(xy(100, 150)),
        &mut MockEngine {},
        &mut graphics,
    );
    zoom_handler.handle(
        &bindings,
        &Event::Button {
            button: Button::Keyboard(KeyboardKey::from("-")),
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

    // finally
    clear(&mut graphics);
    event_loop.exit();
}

#[test]
fn resize_handler() {
    struct MockEngine {}

    impl Engine for MockEngine {
        fn shutdown(&mut self) {}
    }

    // given
    let event_loop = event_loop();
    let mut graphics = GliumGraphics::new(
        graphics::Parameters {
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
            light_direction: xyz(-1.0, 0.0, 0.0),
        },
        &event_loop,
    )
    .unwrap();

    let index = graphics.create_triangles().unwrap();
    let triangles = cube_triangles();
    graphics.draw_triangles(&index, &triangles).unwrap();

    // when
    handlers::resize::handle(
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

    // finally
    event_loop.exit();
}

fn clear(graphics: &mut dyn Graphics) {
    // when
    graphics.clear();
    graphics.render().unwrap();

    let temp_path = temp_dir().join("test.png");
    let temp_path = temp_path.to_str().unwrap();
    graphics.screenshot(temp_path).unwrap();

    // then
    let actual = image::open(temp_path).unwrap();
    let expected = image::open("test_resources/graphics/clear.png").unwrap();
    let difference = commons::image::difference(&actual, &expected).unwrap();
    let max_difference = (256 * 256 * (255 * 3)) / 1000;

    assert!(difference < max_difference);
}
