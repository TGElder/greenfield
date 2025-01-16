use commons::color::Rgba;
use commons::geometry::{xy, xyz, XY};
use commons::grid::Grid;

use commons::origin_grid::OriginGrid;
use engine::graphics::elements::{OverlayTriangles, TexturedPosition};

use engine::graphics::utils::{quad_normal, textured_triangles_from_textured_quads};
use engine::graphics::{DrawMode, Graphics};

const WHITE: Rgba<u8> = Rgba::new(63, 155, 11, 255);

pub struct Drawing {
    width: u32,
    height: u32,
    base_texture: usize,
    overlay_texture: usize,
    slab_size: u32,
    slabs: Grid<usize>,
}

impl Drawing {
    pub fn init(graphics: &mut dyn Graphics, terrain: &Grid<f32>) -> Drawing {
        let slab_size = 256;
        let slabs = Grid::from_fn(
            (terrain.width() / slab_size) + 1,
            (terrain.height() / slab_size) + 1,
            |_| graphics.create_overlay_triangles().unwrap(),
        );
        let width = terrain.width() - 1;
        let height = terrain.height() - 1;

        let colors = Grid::from_element(width, height, WHITE);
        let base_texture = graphics.load_texture(&colors).unwrap();
        let overlay = Grid::from_element(width, height, Rgba::new(0, 0, 0, 0));
        let overlay_texture = graphics.load_texture(&overlay).unwrap();

        Drawing {
            width,
            height,
            base_texture,
            overlay_texture,
            slab_size,
            slabs,
        }
    }

    pub fn draw_geometry(&self, graphics: &mut dyn Graphics, terrain: &Grid<f32>) {
        self.slabs
            .iter()
            .for_each(|slab| self.draw_slab(graphics, terrain, &slab));
    }

    fn draw_slab(&self, graphics: &mut dyn Graphics, terrain: &Grid<f32>, slab: &XY<u32>) {
        let slab_size = self.slab_size;
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
                            position.x / self.width as f32,
                            position.y / self.height as f32,
                        ),
                    })
                    .collect::<Vec<_>>();
                quads.push(textured_positions.try_into().unwrap());
            }
        }
        let overlay_triangles = OverlayTriangles {
            base_texture: self.base_texture,
            overlay_texture: self.overlay_texture,
            triangles: textured_triangles_from_textured_quads(&quads),
        };
        let index = self.slabs[slab];
        graphics
            .draw_overlay_triangles(&index, DrawMode::Solid, &overlay_triangles)
            .unwrap();
    }

    pub fn _modify_overlay(
        &self,
        graphics: &mut dyn Graphics,
        image: &OriginGrid<Rgba<u8>>,
    ) -> Result<(), engine::graphics::errors::DrawError> {
        graphics.modify_texture(&self.overlay_texture, image)
    }
}
