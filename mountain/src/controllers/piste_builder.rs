use std::collections::hash_map::Entry;
use std::collections::HashMap;

use commons::geometry::{xy, XYRectangle};
use commons::grid::{Grid, CORNERS_INVERSE};
use commons::origin_grid::OriginGrid;

use crate::controllers::Result::{self, Action, NoAction};

use crate::model::open;
use crate::model::piste::{self, Piste};
use crate::model::selection::Selection;
use crate::services::id_allocator;
use crate::systems::{messenger, terrain_artist, tree_artist};

pub struct Controller {
    pub class: piste::Class,
    enabled: bool,
}

pub struct Parameters<'a> {
    pub pistes: &'a mut HashMap<usize, Piste>,
    pub piste_map: &'a mut Grid<Option<usize>>,
    pub open: &'a mut HashMap<usize, open::Status>,
    pub selection: &'a mut Selection,
    pub terrain_artist: &'a mut terrain_artist::System,
    pub tree_artist: &'a mut tree_artist::System,
    pub id_allocator: &'a mut id_allocator::Service,
    pub messenger: &'a mut messenger::System,
}

impl Controller {
    pub fn new(class: piste::Class, enabled: bool) -> Controller {
        Controller { class, enabled }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn trigger(
        &mut self,
        Parameters {
            pistes,
            piste_map,
            open,
            selection,
            terrain_artist,
            tree_artist,
            id_allocator,
            messenger,
        }: Parameters<'_>,
    ) -> Result {
        if !self.enabled {
            return NoAction;
        }

        let (Some(origin), Some(grid)) = (selection.cells.first(), &selection.grid) else {
            return NoAction;
        };

        let Ok(rectangle) = grid.rectangle() else {
            return NoAction;
        };

        let piste_id = piste_map[origin].unwrap_or_else(|| id_allocator.next_id());

        if let Some(open::Status::Closed) = open.get(&piste_id) {
        } else {
            messenger.send("Cannot edit piste: Piste is not closed");
            return NoAction;
        }
        // updating piste map

        for cell in grid.iter().filter(|cell| grid[cell]) {
            if piste_map[cell].is_none() {
                piste_map[cell] = Some(piste_id)
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
                .any(|cell| piste_map[cell] == Some(piste_id))
        });

        match pistes.entry(piste_id) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().grid = entry.get().grid.paste(&point_grid);
            }
            Entry::Vacant(entry) => {
                entry.insert(Piste {
                    class: self.class,
                    grid: point_grid,
                });
                open.insert(piste_id, open::Status::Closed);
            }
        }

        // updating art

        terrain_artist.update_overlay(rectangle);
        tree_artist.update();

        // clearing selection

        selection.cells.clear();

        Action
    }
}
