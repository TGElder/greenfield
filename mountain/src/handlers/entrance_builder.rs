use std::collections::HashMap;

use commons::geometry::{xy, XYRectangle};
use commons::grid::Grid;
use engine::binding::Binding;

use crate::handlers::selection;
use crate::model::entrance::Entrance;
use crate::services::id_allocator;
use crate::systems::overlay;

pub struct Handler {
    pub binding: Binding,
}

pub struct Parameters<'a> {
    pub event: &'a engine::events::Event,
    pub piste_map: &'a Grid<Option<usize>>,
    pub selection: &'a mut selection::Handler,
    pub overlay: &'a mut overlay::System,
    pub id_allocator: &'a mut id_allocator::Service,
    pub entrances: &'a mut HashMap<usize, Entrance>,
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
            overlay,
            piste_map,
            id_allocator,
            entrances,
        }: Parameters<'_>,
    ) {
        if !self.binding.binds_event(event) {
            return;
        }
        let (Some(origin), Some(rectangle)) = (selection.origin, selection.rectangle) else {
            return;
        };

        // clearing selection

        selection.clear_selection();
        overlay.update(rectangle);

        // create entrance

        if rectangle.width() == 0 || rectangle.height() == 0 {
            println!("WARN: Entrance must not be zero length");
            return;
        }

        if let Some([a, b]) = get_pistes_if_valid_vertical_entrance(rectangle, piste_map) {
            let piste = (a + b) - piste_map[origin].unwrap();

            entrances.insert(
                id_allocator.next_id(),
                Entrance {
                    from: xy(rectangle.to.x, rectangle.from.y),
                    to: xy(rectangle.to.x, rectangle.to.y + 1),
                    piste,
                },
            );

            dbg!(entrances);
        } else if let Some([a, b]) = get_pites_if_valid_horizontal_entrance(rectangle, piste_map) {
            let piste = (a + b) - piste_map[origin].unwrap();

            entrances.insert(
                id_allocator.next_id(),
                Entrance {
                    from: xy(rectangle.from.x, rectangle.to.y),
                    to: xy(rectangle.to.x + 1, rectangle.to.y),
                    piste,
                },
            );

            dbg!(entrances);
        }
    }
}

fn get_pistes_if_valid_vertical_entrance(
    rectangle: XYRectangle<u32>,
    piste_map: &Grid<Option<usize>>,
) -> Option<[usize; 2]> {
    if rectangle.width() != 2 {
        println!("INFO: Not vertical entrance - selection must be 2 wide");
        return None;
    }

    let mut out = [0; 2];

    for (index, x) in (rectangle.from.x..=rectangle.to.x).enumerate() {
        let value = piste_map[xy(x, rectangle.from.y)]?;
        for y in rectangle.from.y..=rectangle.to.y {
            if piste_map[xy(x, y)]? != value {
                println!("INFO: Not vertical entrance - column does not contain piste");
                return None;
            }
        }
        out[index] = value;
    }

    if out[0] == out[1] {
        println!("INFO: Not vertical entrance - same piste on both sides");
        return None;
    }

    Some(out)
}

fn get_pites_if_valid_horizontal_entrance(
    rectangle: XYRectangle<u32>,
    piste_map: &Grid<Option<usize>>,
) -> Option<[usize; 2]> {
    if rectangle.height() != 2 {
        println!("INFO: Not horizontal entrance - selection must be 2 high");
        return None;
    }

    let mut out = [0; 2];

    for (index, y) in (rectangle.from.y..=rectangle.to.y).enumerate() {
        let value = piste_map[xy(rectangle.from.x, y)]?;
        for x in rectangle.from.x..=rectangle.to.x {
            if piste_map[xy(x, y)]? != value {
                println!("INFO: Not horizontal entrance - row does not contain piste");
                return None;
            }
        }
        out[index] = value;
    }

    if out[0] == out[1] {
        println!("INFO: Not horizontal entrance - same piste on both sides");
        return None;
    }

    Some(out)
}
