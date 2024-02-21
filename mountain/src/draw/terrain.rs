use commons::color::Rgba;
use commons::geometry::{xy, xyz};
use commons::grid::Grid;

use commons::origin_grid::OriginGrid;
use engine::graphics::elements::{OverlayTriangles, TexturedPosition};

use engine::graphics::utils::{quad_normal, textured_triangles_from_textured_quads};
use engine::graphics::Graphics;

const WHITE: Rgba<u8> = Rgba::new(255, 255, 255, 255);

pub struct Drawing {
    overlay_texture: usize,
}

impl Drawing {
    pub fn init(graphics: &mut dyn Graphics, terrain: &Grid<f32>) -> Drawing {
        let slab_size = 256;
        let slabs = xy(
            (terrain.width() / slab_size) + 1,
            (terrain.height() / slab_size) + 1,
        );
        let colors = Grid::from_element(terrain.width() - 1, terrain.height() - 1, WHITE);

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

                        let normal = quad_normal(&corners);

                        let textured_positions = corners
                            .into_iter()
                            .map(|position| TexturedPosition {
                                position,
                                normal,
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
            let index = graphics.create_overlay_triangles().unwrap();

            let overlay_triangles = OverlayTriangles {
                base_texture,
                overlay_texture,
                triangles: textured_triangles_from_textured_quads(&quads),
            };
            graphics
                .draw_overlay_triangles(&index, &overlay_triangles)
                .unwrap();
        }

        Drawing { overlay_texture }
    }

    pub fn modify_overlay(
        &self,
        graphics: &mut dyn Graphics,
        image: &OriginGrid<Rgba<u8>>,
    ) -> Result<(), engine::graphics::errors::DrawError> {
        graphics.modify_texture(&self.overlay_texture, image)
    }
}
