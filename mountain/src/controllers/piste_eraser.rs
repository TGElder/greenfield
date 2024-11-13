use std::collections::HashMap;

use commons::geometry::{xy, XYRectangle, XY};
use commons::grid::{Grid, CORNERS_INVERSE};
use commons::map::ContainsKeyValue;
use commons::origin_grid::OriginGrid;

use crate::controllers::Result::{self, Action, NoAction};

use crate::model::door::Door;
use crate::model::gate::Gate;
use crate::model::lift::Lift;
use crate::model::piste::Piste;
use crate::model::selection::Selection;
use crate::systems::{messenger, terrain_artist, tree_artist};

pub struct Controller {
    enabled: bool,
}

pub struct Parameters<'a> {
    pub open: &'a HashMap<usize, bool>,
    pub locations: &'a HashMap<usize, usize>,
    pub lifts: &'a HashMap<usize, Lift>,
    pub gates: &'a HashMap<usize, Gate>,
    pub doors: &'a HashMap<usize, Door>,
    pub pistes: &'a mut HashMap<usize, Piste>,
    pub piste_map: &'a mut Grid<Option<usize>>,
    pub selection: &'a mut Selection,
    pub terrain_artist: &'a mut terrain_artist::System,
    pub tree_artist: &'a mut tree_artist::System,
    pub messenger: &'a mut messenger::System,
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
            open,
            locations,
            lifts,
            gates,
            doors,
            pistes,
            piste_map,
            selection,
            terrain_artist,
            tree_artist,
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

        let Some(piste_id) = piste_map[origin] else {
            return NoAction;
        };

        let Some(piste) = pistes.get_mut(&piste_id) else {
            return NoAction;
        };

        if open.contains_key_value(piste_id, true) {
            messenger.send("Cannot erase piste: Piste is still open");
            return NoAction;
        }

        if let Some((entity_id, _)) = locations
            .iter()
            .find(|(_, &location_id)| location_id == piste_id)
        {
            messenger.send(format!(
                "Cannot erase piste: Skier {} is still on piste",
                entity_id
            ));
            return NoAction;
        }

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

        let is_in_selection_on_piste = |position: XY<u32>| {
            point_grid.in_bounds(position)
                && point_grid[position]
                && piste.grid.in_bounds(position)
                && piste.grid[position]
        };

        for (lift_id, lift) in lifts.iter() {
            let drop_off = lift.drop_off.state.position;
            let pick_up = lift.pick_up.state.position;
            if is_in_selection_on_piste(drop_off) || is_in_selection_on_piste(pick_up) {
                messenger.send(format!(
                    "Cannot erase piste: selection contains Lift {}",
                    lift_id
                ));
                return NoAction;
            }
        }

        for (gate_id, gate) in gates.iter() {
            if gate.footprint.iter().any(is_in_selection_on_piste) {
                messenger.send(format!(
                    "Cannot erase piste: selection contains Gate {}",
                    gate_id
                ));
                return NoAction;
            }
        }

        for (door_id, door) in doors.iter() {
            if door
                .aperture
                .iter()
                .any(|&position| is_in_selection_on_piste(position))
            {
                messenger.send(format!(
                    "Cannot erase piste: selection contains Door {}",
                    door_id
                ));
                return NoAction;
            }
        }

        // updating piste map

        for cell in grid.iter().filter(|cell| grid[cell]) {
            if piste_map[cell] == Some(piste_id) {
                piste_map[cell] = None
            }
        }

        // updating piste

        piste.grid = piste.grid.paste(&point_grid);

        // updating art

        terrain_artist.update_overlay(rectangle);
        tree_artist.update();

        // clearing selection

        selection.cells.clear();

        Action
    }
}
