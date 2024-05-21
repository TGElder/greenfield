use std::collections::hash_map::Entry;
use std::collections::HashMap;

use commons::geometry::{xy, XYRectangle};
use commons::grid::{Grid, CORNERS_INVERSE};
use commons::origin_grid::OriginGrid;
use engine::binding::Binding;

use crate::handlers::selection;
use crate::model::piste::{self, Piste};
use crate::services::id_allocator;
use crate::systems::{terrain_artist, tree_artist};

pub struct Handler {
    pub bindings: Bindings,
    pub class: piste::Class,
}

pub struct Bindings {
    pub add: Binding,
    pub subtract: Binding,
}

pub struct Parameters<'a> {
    pub event: &'a engine::events::Event,
    pub pistes: &'a mut HashMap<usize, Piste>,
    pub piste_map: &'a mut Grid<Option<usize>>,
    pub selection: &'a mut selection::Handler,
    pub terrain_artist: &'a mut terrain_artist::System,
    pub tree_artist: &'a mut tree_artist::System,
    pub id_allocator: &'a mut id_allocator::Service,
}

impl Handler {
    pub fn handle(
        &mut self,
        Parameters {
            event,
            pistes,
            piste_map,
            selection,
            terrain_artist,
            tree_artist,
            id_allocator,
        }: Parameters<'_>,
    ) {
        let add = self.bindings.add.binds_event(event);
        let subtract = self.bindings.subtract.binds_event(event);
        if !(add || subtract) {
            return;
        }

        let (Some(origin), Some(grid)) = (selection.cells.first(), &selection.grid) else {
            return;
        };

        let Ok(rectangle) = grid.rectangle() else {
            return;
        };

        let id = piste_map[origin].unwrap_or_else(|| id_allocator.next_id());

        // updating piste map

        for cell in grid.iter().filter(|cell| grid[cell]) {
            if add && piste_map[cell].is_none() {
                piste_map[cell] = Some(id)
            } else if subtract && piste_map[cell] == Some(id) {
                piste_map[cell] = None
            }
        }

        // updating piste

        let point_grid = OriginGrid::from_rectangle(
            XYRectangle {
                from: rectangle.from,
                to: xy(rectangle.to.x + 1, rectangle.to.y + 1),
            },
            false,
        );
        let point_grid = point_grid.map(|point, _| {
            piste_map
                .offsets(point, &CORNERS_INVERSE)
                .any(|cell| piste_map[cell] == Some(id))
        });

        match pistes.entry(id) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().grid = entry.get().grid.paste(&point_grid);
            }
            Entry::Vacant(entry) => {
                entry.insert(Piste {
                    class: self.class,
                    grid: point_grid,
                });
            }
        }

        // updating art

        terrain_artist.update(rectangle);
        tree_artist.update();

        // clearing selection

        selection.clear_selection();
    }
}
