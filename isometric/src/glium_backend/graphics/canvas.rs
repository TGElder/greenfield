use glium::Surface;

pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pub texture: glium::Texture2d,
    pub depth_buffer: glium::framebuffer::DepthRenderBuffer,
}

impl Canvas {
    pub fn new(display: &glium::Display, &(width, height): &(u32, u32)) -> Canvas {
        Canvas {
            width,
            height,
            texture: glium::texture::Texture2d::empty_with_format(
                display,
                glium::texture::UncompressedFloatFormat::F32F32F32F32,
                glium::texture::MipmapsOption::NoMipmap,
                width,
                height,
            )
            .unwrap(),
            depth_buffer: glium::framebuffer::DepthRenderBuffer::new(
                display,
                glium::texture::DepthFormat::F32,
                width,
                height,
            )
            .unwrap(),
        }
    }

    pub fn frame(&self, display: &glium::Display) -> glium::framebuffer::SimpleFrameBuffer {
        let mut out = glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(
            display,
            &self.texture,
            &self.depth_buffer,
        )
        .unwrap();
        out.clear_color(0.0, 0.0, 0.0, 0.0);
        out.clear_depth(1.0);
        out
    }
}
