use std::collections::HashMap;

use commons::geometry::{xy, XYRectangle};
use commons::grid::{Grid, CORNERS_INVERSE};
use commons::origin_grid::OriginGrid;

use crate::controllers::Result::{self, Action, NoAction};

use crate::model::piste::Piste;
use crate::model::selection::Selection;
use crate::systems::{terrain_artist, tree_artist};

pub struct Controller {
    enabled: bool,
}

pub struct Parameters<'a> {
    pub pistes: &'a mut HashMap<usize, Piste>,
    pub piste_map: &'a mut Grid<Option<usize>>,
    pub selection: &'a mut Selection,
    pub terrain_artist: &'a mut terrain_artist::System,
    pub tree_artist: &'a mut tree_artist::System,
}

impl Controller {
    pub fn new(enabled: bool) -> Controller {
        Controller { enabled }
    }

    pub fn is_enabled(&self) -> &bool {
        &self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn trigger(
        &self,
        Parameters {
            pistes,
            piste_map,
            selection,
            terrain_artist,
            tree_artist,
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

        let Some(id) = piste_map[origin] else {
            return NoAction;
        };

        let Some(piste) = pistes.get(&id) else {
            return NoAction;
        };

        // updating piste map

        for cell in grid.iter().filter(|cell| grid[cell]) {
            if piste_map[cell] == Some(id) {
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

        piste.grid.paste(&point_grid);

        // updating art

        terrain_artist.update_overlay(rectangle);
        tree_artist.update();

        // clearing selection

        selection.cells.clear();

        Action
    }
}
