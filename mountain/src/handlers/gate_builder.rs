use std::collections::{HashMap, HashSet};

use commons::geometry::{xy, XYRectangle};
use commons::grid::Grid;
use engine::binding::Binding;

use crate::handlers::selection;
use crate::model::gate::Gate;
use crate::model::reservation::Reservation;
use crate::services::id_allocator;
use crate::systems::terrain_artist;

pub struct Handler {
    pub binding: Binding,
}

pub struct Parameters<'a> {
    pub event: &'a engine::events::Event,
    pub piste_map: &'a Grid<Option<usize>>,
    pub selection: &'a mut selection::Handler,
    pub terrain_artist: &'a mut terrain_artist::System,
    pub id_allocator: &'a mut id_allocator::Service,
    pub gates: &'a mut HashMap<usize, Gate>,
    pub open: &'a mut HashSet<usize>,
    pub reservations: &'a mut Grid<HashMap<usize, Reservation>>,
}

impl Handler {
    pub fn new(binding: Binding) -> Handler {
        Handler { binding }
    }

    pub fn handle(
        &mut self,
        Parameters {
            event,
            selection,
            terrain_artist,
            piste_map,
            id_allocator,
            gates,
            open,
            reservations,
        }: Parameters<'_>,
    ) {
        if !self.binding.binds_event(event) {
            return;
        }
        let (Some(&origin), Some(grid)) = (selection.cells.first(), &selection.grid) else {
            return;
        };

        let Ok(rectangle) = grid.rectangle() else {
            return;
        };

        // clearing selection

        selection.clear_selection();
        terrain_artist.update(rectangle);

        // create gate

        if rectangle.width() == 0 || rectangle.height() == 0 {
            println!("WARN: Entrance must not be zero length");
            return;
        }

        let maybe_gate = get_pistes_if_valid_vertical_gate(rectangle, piste_map)
            .map(|[a, b]| {
                let origin_piste = piste_map[origin].unwrap();
                Gate {
                    footprint: XYRectangle {
                        from: xy(rectangle.to.x, rectangle.from.y),
                        to: xy(rectangle.to.x, rectangle.to.y + 1),
                    },
                    origin_piste,
                    destination_piste: (a + b) - origin_piste,
                }
            })
            .or_else(|| {
                get_pistes_if_valid_horizontal_gate(rectangle, piste_map).map(|[a, b]| {
                    let origin_piste = piste_map[origin].unwrap();
                    Gate {
                        footprint: XYRectangle {
                            from: xy(rectangle.from.x, rectangle.to.y),
                            to: xy(rectangle.to.x + 1, rectangle.to.y),
                        },
                        origin_piste,
                        destination_piste: (a + b) - origin_piste,
                    }
                })
            });

        let Some(gate) = maybe_gate else {
            return;
        };

        let gate_id = id_allocator.next_id();

        // reserving footprint

        gate.footprint.iter().for_each(|position| {
            reservations[position].insert(gate_id, Reservation::Structure);
        });

        // opening gate

        open.insert(gate_id);

        // register gate

        gates.insert(gate_id, gate);
    }
}

fn get_pistes_if_valid_vertical_gate(
    rectangle: XYRectangle<u32>,
    piste_map: &Grid<Option<usize>>,
) -> Option<[usize; 2]> {
    if rectangle.width() != 2 {
        println!("INFO: Not vertical gate - selection must be 2 wide");
        return None;
    }

    let mut out = [0; 2];

    for (index, x) in (rectangle.from.x..=rectangle.to.x).enumerate() {
        let value = piste_map[xy(x, rectangle.from.y)]?;
        for y in rectangle.from.y..=rectangle.to.y {
            if piste_map[xy(x, y)]? != value {
                println!("INFO: Not vertical gate - column does not contain piste");
                return None;
            }
        }
        out[index] = value;
    }

    if out[0] == out[1] {
        println!("INFO: Not vertical gate - same piste on both sides");
        return None;
    }

    Some(out)
}

fn get_pistes_if_valid_horizontal_gate(
    rectangle: XYRectangle<u32>,
    piste_map: &Grid<Option<usize>>,
) -> Option<[usize; 2]> {
    if rectangle.height() != 2 {
        println!("INFO: Not horizontal gate - selection must be 2 high");
        return None;
    }

    let mut out = [0; 2];

    for (index, y) in (rectangle.from.y..=rectangle.to.y).enumerate() {
        let value = piste_map[xy(rectangle.from.x, y)]?;
        for x in rectangle.from.x..=rectangle.to.x {
            if piste_map[xy(x, y)]? != value {
                println!("INFO: Not horizontal gate - row does not contain piste");
                return None;
            }
        }
        out[index] = value;
    }

    if out[0] == out[1] {
        println!("INFO: Not horizontal gate - same piste on both sides");
        return None;
    }

    Some(out)
}
