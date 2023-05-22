use std::f32::consts::PI;

use commons::color::{Rgb, Rgba};
use commons::geometry::{xy, xyz, XY, XYZ};
use commons::grid::Grid;

use engine::graphics::elements::{OverlayQuads, TexturedPosition};

use engine::graphics::Graphics;

use nalgebra::Vector3;

pub struct Drawing {
    overlay_texture: usize,
}

impl Drawing {
    pub fn modify_overlay(
        &self,
        graphics: &mut dyn Graphics,
        from: &XY<u32>,
        image: &Grid<Rgba<u8>>,
    ) -> Result<(), engine::graphics::errors::DrawError> {
        graphics.modify_texture(&self.overlay_texture, from, image)
    }
}

pub fn draw(graphics: &mut dyn Graphics, terrain: &Grid<f32>) -> Drawing {
    let slab_size = 256;
    let slabs = xy(
        (terrain.width() / slab_size) + 1,
        (terrain.height() / slab_size) + 1,
    );
    let mut colors = Grid::from_element(
        terrain.width() - 1,
        terrain.height() - 1,
        Rgba::new(0, 0, 0, 0),
    );

    let mut to_draw = Vec::with_capacity((slabs.x * slabs.y) as usize);

    for x in 0..slabs.x {
        for y in 0..slabs.y {
            let slab = xy(x, y);
            let mut quads = Vec::with_capacity((slab_size * slab_size) as usize);
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
                                terrain[xy(x + d.x, y + d.y)],
                            )
                        })
                        .collect::<Vec<_>>();

                    colors[xy(x, y)] = color(&corners);

                    let textured_positions = corners
                        .into_iter()
                        .map(|position| TexturedPosition {
                            position,
                            texture_coordinates: xy(
                                position.x / colors.width() as f32,
                                position.y / colors.height() as f32,
                            ),
                        })
                        .collect::<Vec<_>>();
                    quads.push(textured_positions.try_into().unwrap());
                }
            }

            to_draw.push(quads);
        }
    }

    let base_texture = graphics.load_texture(&colors).unwrap();
    let overlay = Grid::from_element(
        terrain.width() - 1,
        terrain.height() - 1,
        Rgba::new(0, 0, 0, 0),
    );
    let overlay_texture = graphics.load_texture(&overlay).unwrap();

    for quads in to_draw {
        let index = graphics.create_overlay_quads().unwrap();

        let overlay_quads = OverlayQuads {
            base_texture,
            overlay_texture,
            quads,
        };
        graphics.draw_overlay_quads(&index, &overlay_quads).unwrap();
    }

    Drawing { overlay_texture }
}

fn color(corners: &[XYZ<f32>]) -> Rgba<u8> {
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

    fn to_u8(value: f32) -> u8 {
        (value * 255.0).round() as u8
    }

    Rgba::new(
        to_u8(base_color.r * shade),
        to_u8(base_color.g * shade),
        to_u8(base_color.b * shade),
        255,
    )
}
