use std::collections::HashMap;

use engine::graphics::Graphics;

use crate::draw::frame;
use crate::model::frame::Frame;

pub fn run(
    graphics: &mut dyn Graphics,
    frames: &HashMap<usize, Option<Frame>>,
    drawings: &mut HashMap<usize, usize>,
) {
    for (id, frame) in frames {
        match frame {
            Some(frame) => {
                let triangles = frame::draw(frame);
                let index = drawings.entry(*id).or_insert_with(|| {
                    graphics.create_dynamic_triangles(&triangles.len()).unwrap()
                });
                graphics
                    .update_dynamic_triangles(index, Some(&triangles))
                    .unwrap()
            }
            None => {
                if let Some(index) = drawings.get(id) {
                    graphics.update_dynamic_triangles(index, None).unwrap()
                }
            }
        }
    }
}
