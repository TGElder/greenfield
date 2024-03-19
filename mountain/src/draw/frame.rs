use commons::color::Rgb;

use engine::graphics::transform::{Recolor, Transform};
use engine::graphics::utils::{transformation_matrix, triangles_from_quads, Transformation};
use engine::graphics::Graphics;

use crate::draw;
use crate::draw::model::{self, chair, skier};
use crate::model::frame::{Frame, Model};
use crate::model::skier::Clothes;

lazy_static! {
    static ref SKIER_STANDING_MODEL: draw::model::Model<skier::Color, skier::AttachmentPoints> =
        model::skier_standing::model();
}
lazy_static! {
    static ref SKIER_SITTING_MODEL: draw::model::Model<skier::Color, ()> =
        model::skier_sitting::model();
}
lazy_static! {
    static ref CHAIR_MODEL: draw::model::Model<Rgb<f32>, chair::AttachmentPoints> =
        model::chair::model();
}

pub fn draw(graphics: &mut dyn Graphics, index: &usize, frame: &Frame) {
    let transformation = transformation_matrix(Transformation {
        translation: Some(frame.position),
        yaw: Some(frame.yaw),
        pitch: Some(frame.pitch),
        ..Transformation::default()
    });

    let quads = match frame.model {
        Model::Standing {
            skis: false,
            clothes,
        } => SKIER_STANDING_MODEL
            .quads
            .recolor(&|color| get_rgb(&clothes, color)),
        Model::Standing {
            skis: true,
            clothes,
        } => SKIER_STANDING_MODEL
            .quads
            .recolor(&|color| get_rgb(&clothes, color)),
        Model::Sitting { clothes } => SKIER_SITTING_MODEL
            .quads
            .recolor(&|color| get_rgb(&clothes, color)),
        Model::Chair => CHAIR_MODEL.quads.clone(),
    };
    let transformed_quads = quads.transform(&transformation);
    let triangles = triangles_from_quads(&transformed_quads);
    graphics.draw_triangles(index, &triangles).unwrap();
}

fn get_rgb(clothes: &Clothes<Rgb<f32>>, color: &model::skier::Color) -> Rgb<f32> {
    match color {
        skier::Color::Skis => clothes.skis,
        skier::Color::Trousers => clothes.trousers,
        skier::Color::Jacket => clothes.jacket,
        skier::Color::Helmet => clothes.helmet,
    }
}
