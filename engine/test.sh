cargo test glium_backend::graphics::tests::render_cube -- --exact &&
cargo test glium_backend::graphics::tests::render_cube_dynamic -- --exact  &&
cargo test glium_backend::graphics::tests::instanced_cubes -- --exact &&
cargo test glium_backend::graphics::tests::render_billboard -- --exact &&
cargo test glium_backend::graphics::tests::render_overlay_quads -- --exact &&
cargo test glium_backend::graphics::tests::look_at -- --exact &&
cargo test glium_backend::graphics::tests::drag_handler -- --exact &&
cargo test glium_backend::graphics::tests::yaw_handler -- --exact &&
cargo test glium_backend::graphics::tests::zoom_handler -- --exact &&
cargo test glium_backend::graphics::tests::resize_handler -- --exact