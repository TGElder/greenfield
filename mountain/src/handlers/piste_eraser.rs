use std::collections::HashMap;

use commons::geometry::{xy, XYRectangle};
use commons::grid::{Grid, CORNERS_INVERSE};
use commons::origin_grid::OriginGrid;
use engine::binding::Binding;

use crate::handlers::{
    selection,
    HandlerResult::{self, EventConsumed, EventPersists},
};
use crate::model::piste::Piste;
use crate::systems::{terrain_artist, tree_artist};

pub struct Handler {
    pub binding: Binding,
}

pub struct Parameters<'a> {
    pub event: &'a engine::events::Event,
    pub pistes: &'a mut HashMap<usize, Piste>,
    pub piste_map: &'a mut Grid<Option<usize>>,
    pub selection: &'a mut selection::Handler,
    pub terrain_artist: &'a mut terrain_artist::System,
    pub tree_artist: &'a mut tree_artist::System,
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
        }: Parameters<'_>,
    ) -> HandlerResult {
        if !self.binding.binds_event(event) {
            return EventPersists;
        }

        let (Some(origin), Some(grid)) = (selection.cells.first(), &selection.grid) else {
            return EventPersists;
        };

        let Ok(rectangle) = grid.rectangle() else {
            return EventPersists;
        };

        let Some(id) = piste_map[origin] else {
            return EventPersists;
        };

        let Some(piste) = pistes.get(&id) else {
            return EventPersists;
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

        selection.clear_selection();

        EventConsumed
    }
}
