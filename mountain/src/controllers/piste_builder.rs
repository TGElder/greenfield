use std::collections::hash_map::Entry;
use std::collections::HashMap;

use commons::geometry::{xy, XYRectangle};
use commons::grid::{Grid, CORNERS_INVERSE};
use commons::origin_grid::OriginGrid;

use crate::controllers::Result::{self, Action, NoAction};
use crate::handlers::selection;

use crate::model::piste::{self, Piste};
use crate::services::id_allocator;
use crate::systems::{terrain_artist, tree_artist};

pub struct Controller {
    pub class: piste::Class,
}

pub struct Parameters<'a> {
    pub pistes: &'a mut HashMap<usize, Piste>,
    pub piste_map: &'a mut Grid<Option<usize>>,
    pub selection: &'a mut selection::Handler,
    pub terrain_artist: &'a mut terrain_artist::System,
    pub tree_artist: &'a mut tree_artist::System,
    pub id_allocator: &'a mut id_allocator::Service,
}

impl Controller {
    pub fn trigger(
        &mut self,
        Parameters {
            pistes,
            piste_map,
            selection,
            terrain_artist,
            tree_artist,
            id_allocator,
        }: Parameters<'_>,
    ) -> Result {
        let (Some(origin), Some(grid)) = (selection.cells.first(), &selection.grid) else {
            return NoAction;
        };

        let Ok(rectangle) = grid.rectangle() else {
            return NoAction;
        };

        let id = piste_map[origin].unwrap_or_else(|| id_allocator.next_id());

        // updating piste map

        for cell in grid.iter().filter(|cell| grid[cell]) {
            if piste_map[cell].is_none() {
                piste_map[cell] = Some(id)
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

        terrain_artist.update_overlay(rectangle);
        tree_artist.update();

        // clearing selection

        selection.clear_selection();

        Action
    }
}
