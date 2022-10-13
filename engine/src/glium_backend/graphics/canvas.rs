use std::error::Error;

pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pub texture: glium::Texture2d,
    pub depth_buffer: glium::framebuffer::DepthRenderBuffer,
}

impl Canvas {
    pub fn new(
        facade: &dyn glium::backend::Facade,
        &(width, height): &(u32, u32),
    ) -> Result<Canvas, Box<dyn Error>> {
        Ok(Canvas {
            width,
            height,
            texture: glium::texture::Texture2d::empty_with_format(
                facade,
                glium::texture::UncompressedFloatFormat::F32F32F32F32,
                glium::texture::MipmapsOption::NoMipmap,
                width,
                height,
            )?,
            depth_buffer: glium::framebuffer::DepthRenderBuffer::new(
                facade,
                glium::texture::DepthFormat::F32,
                width,
                height,
            )?,
        })
    }

    pub fn frame(
        &self,
        facade: &dyn glium::backend::Facade,
    ) -> Result<glium::framebuffer::SimpleFrameBuffer, Box<dyn Error>> {
        let mut out = glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(
            facade,
            &self.texture,
            &self.depth_buffer,
        )?;
        glium::Surface::clear_color(&mut out, 0.0, 0.0, 0.0, 0.0);
        glium::Surface::clear_depth(&mut out, 1.0);
        Ok(out)
    }

    pub fn save_texture(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let raw_image: glium::texture::RawImage2d<'_, f32> = self
            .texture
            .main_level()
            .first_layer()
            .into_image(None)
            .ok_or_else(|| "Canvas texture is a cubemap - this should not happen")?
            .raw_read::<_, (f32, f32, f32)>(&glium::Rect {
                left: 0,
                width: self.texture.width(),
                bottom: 0,
                height: self.texture.height(),
            });

        let data_gamma_corrected = raw_image.data.iter().map(|x| x.powf(1.0 / 2.2)).collect();

        let image =
            image::ImageBuffer::from_vec(raw_image.width, raw_image.height, data_gamma_corrected)
                .ok_or_else(|| {
                format!(
                    "Canvas texture data does not fit into buffer of size {}x{}",
                    raw_image.width, raw_image.height
                )
            })?;
        let image =
            image::DynamicImage::ImageRgb8(image::DynamicImage::ImageRgb32F(image).into_rgb8());
        let image = image.flipv();
        image.save(path)?;

        Ok(())
    }
}
