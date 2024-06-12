use commons::geometry::xy;
use commons::grid::Grid;
use commons::scale::Scale;
use engine::binding::Binding;

use crate::handlers::selection;
use crate::systems::terrain_artist;

pub struct Handler {
    pub binding: Binding,
}

impl Handler {
    pub fn handle(
        &self,
        event: &engine::events::Event,
        selection: &mut selection::Handler,
        terrain: &mut Grid<f32>,
        terrain_artist: &mut terrain_artist::System,
        graphics: &mut dyn engine::graphics::Graphics,
    ) {
        if !self.binding.binds_event(event) {
            return;
        }

        if selection.cells.len() < 2 {
            return;
        }

        let Some(grid) = &selection.grid else {
            return;
        };

        let from = selection.cells[0];
        let to = selection.cells[1];
        let z_from = terrain[from];
        let z_to = terrain[to];
        let from_f32 = xy(from.x as f32, from.y as f32);
        let to_f32 = xy(to.x as f32, to.y as f32);

        let vector = (to_f32 - from_f32).normalize();
        let projection_to_z =
            Scale::new((from_f32.dot(&vector), to_f32.dot(&vector)), (z_from, z_to));

        for point in grid.iter() {
            if grid[point] {
                let point_f32 = xy(point.x as f32, point.y as f32);
                let projection = point_f32.dot(&vector);
                terrain[point] = projection_to_z.scale(projection);
            }
        }

        selection.clear_selection();
    }
}
