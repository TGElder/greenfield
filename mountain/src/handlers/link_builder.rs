use std::collections::HashMap;

use commons::geometry::{xy, XYRectangle};
use commons::grid::Grid;
use commons::origin_grid::OriginGrid;
use engine::binding::Binding;

use crate::handlers::selection;
use crate::model::link::Link;
use crate::services::id_allocator;
use crate::systems::overlay;

pub struct Handler {
    pub binding: Binding,
}

impl Handler {
    pub fn handle(
        &mut self,
        event: &engine::events::Event,
        piste_map: &Grid<Option<usize>>,
        links: &mut HashMap<usize, Link>,
        selection: &mut selection::Handler,
        overlay: &mut overlay::System,
        id_allocator: &mut id_allocator::Service,
    ) {
        if !self.binding.binds_event(event) {
            return;
        }

        let (Some(origin), Some(rectangle)) = (selection.origin, selection.rectangle) else {
            return;
        };

        let Some(from) = piste_map[origin] else {
            return;
        };
        let grid_to = xy(rectangle.to.x + 1, rectangle.to.y + 1);
        let to_position = if origin == rectangle.from {
            grid_to
        } else {
            rectangle.from
        };
        let Some(to) = piste_map[to_position] else {
            return;
        };

        let grid = OriginGrid::from_rectangle(
            XYRectangle {
                from: rectangle.from,
                to: grid_to,
            },
            true,
        );

        if grid.width() != 2 && grid.height() != 2 {
            return;
        }

        println!("Linking {} to {}", from, to);

        links.insert(
            id_allocator.next_id(),
            Link {
                grid,
                edges: HashMap::new(),
                from,
                to,
            },
        );

        overlay.update(XYRectangle {
            from: xy(
                rectangle.from.x.saturating_sub(1),
                rectangle.from.y.saturating_sub(1),
            ),
            to: xy(rectangle.to.x + 1, rectangle.to.y + 1),
        });
        selection.clear_selection();
    }
}
