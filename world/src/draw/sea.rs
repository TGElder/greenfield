use commons::color::Rgb;
use commons::geometry::xyz;
use commons::grid::Grid;
use engine::graphics::elements::Quad;
use engine::graphics::utils::triangles_from_quads;
use engine::graphics::{DrawMode, Graphics};

pub fn draw(terrain: &Grid<f32>, sea_level: f32, graphics: &mut dyn Graphics) {
    let index = graphics.create_triangles().unwrap();
    let width = terrain.width() as f32;
    let height = terrain.height() as f32;
    graphics
        .draw_triangles(
            &index,
            DrawMode::Solid,
            &triangles_from_quads(&[Quad {
                corners: [
                    xyz(0.0, 0.0, sea_level),
                    xyz(width, 0.0, sea_level),
                    xyz(width, height, sea_level),
                    xyz(0.0, height, sea_level),
                ],
                color: Rgb::new(0.0, 0.0, 255.0),
            }]),
        )
        .unwrap();
}
