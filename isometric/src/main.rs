use std::ffi::CStr;

mod gl {
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new()
    .with_title("Isometric")
    .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0));
    let windowed_context = glutin::ContextBuilder::new()
    .build_windowed(window_builder, &event_loop)
    .unwrap();

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };


    let context = windowed_context.context();

    gl::load_with(|ptr| context.get_proc_address(ptr) as *const _);

    let version = unsafe {
        let data = CStr::from_ptr(gl::GetString(gl::VERSION) as *const _).to_bytes().to_vec();
        String::from_utf8(data).unwrap()
    };

    println!("OpenGL version {}", version);

    loop {
    }
}
    