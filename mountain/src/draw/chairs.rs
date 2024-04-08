use std::collections::HashMap;

use engine::graphics::utils::{transformation_matrix, triangles_from_quads, Transformation};
use engine::graphics::Graphics;

use crate::draw::model::{self};
use crate::model::frame::{Frame, Model};

pub struct Drawing {
    pub index: usize,
}

impl Drawing {
    pub fn init(graphics: &mut dyn Graphics) -> Drawing {
        let quads = model::chair::model().quads;
        let triangles = triangles_from_quads(&quads);
        let index = graphics.create_instanced_triangles(&triangles, &0).unwrap();
        Drawing { index }
    }

    pub fn draw(&self, graphics: &mut dyn Graphics, frames: &HashMap<usize, Option<Frame>>) {
        let world_matrices = frames
            .values()
            .flatten()
            .filter(|frame| matches!(frame.model, Model::Chair))
            .map(|frame| Transformation {
                translation: Some(frame.position),
                yaw: Some(frame.yaw),
                pitch: Some(frame.pitch),
                ..Transformation::default()
            })
            .map(transformation_matrix)
            .map(Some)
            .collect::<Vec<_>>();
        graphics
            .update_instanced_triangles(&self.index, &world_matrices)
            .unwrap();
    }
}
