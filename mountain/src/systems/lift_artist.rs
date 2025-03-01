use std::collections::hash_map::Entry;
use std::collections::HashMap;

use engine::graphics::{DrawMode, Graphics};

use crate::draw::model::line;
use crate::model::lift::{Lift, Segment};

pub fn run(
    graphics: &mut dyn Graphics,
    lifts: &HashMap<usize, Lift>,
    drawings: &mut HashMap<usize, usize>,
) {
    for (id, lift) in lifts {
        let segments = lift
            .segments
            .iter()
            .map(|Segment { from, to, .. }| [*from, *to])
            .collect::<Vec<_>>();

        match drawings.entry(*id) {
            Entry::Occupied(_) => (),
            Entry::Vacant(cell) => {
                if let Ok(index) = graphics.create_triangles() {
                    let triangles = line::model(&segments, 0.5);
                    graphics
                        .draw_triangles(&index, DrawMode::Hologram, &triangles)
                        .unwrap();
                    cell.insert(index);
                }
            }
        };
    }
}
