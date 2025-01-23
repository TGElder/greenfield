use commons::color::Rgb;
use commons::geometry::xyz;
use commons::grid::Grid;
use engine::graphics::models::cube;
use engine::graphics::transform::Recolor;
use engine::graphics::utils::{transformation_matrix, triangles_from_quads, Transformation};
use engine::graphics::{DrawMode, Graphics};

const COLOR: Rgb<f32> = Rgb::new(1.0, 0.0, 0.0);

pub fn draw(towns: &Grid<bool>, tile_heights: &Grid<f32>, graphics: &mut dyn Graphics) {
    let quads = cube::model().recolor(&|_| COLOR);
    let triangles = triangles_from_quads(&quads);
    let index = graphics
        .create_instanced_triangles(DrawMode::Hologram, &triangles)
        .unwrap();
    let matrices = towns
        .iter()
        .filter_map(|xy| {
            if towns[xy] {
                Some(Some(transformation_matrix(Transformation {
                    translation: Some(xyz(
                        xy.x as f32 + 0.5,
                        xy.y as f32 + 0.5,
                        tile_heights[xy] + 0.5,
                    )),
                    ..Transformation::default()
                })))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    graphics
        .update_instanced_triangles(&index, &matrices)
        .unwrap();
}
