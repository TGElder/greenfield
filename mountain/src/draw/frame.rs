use commons::color::Rgb;

use engine::graphics::elements::Triangle;
use engine::graphics::transform::{Recolor, Transform};
use engine::graphics::utils::{transformation_matrix, triangles_from_quads, Transformation};

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

pub fn draw(frame: &Frame) -> Vec<Triangle<Rgb<f32>>> {
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
        unsupported => {
            panic!("Cannot draw model {:?}", unsupported);
        }
    };
    let transformed_quads = quads.transform(&transformation);
    triangles_from_quads(&transformed_quads)
}

fn get_rgb(clothes: &Clothes<Rgb<f32>>, color: &model::skier::Color) -> Rgb<f32> {
    match color {
        skier::Color::Skis => clothes.skis,
        skier::Color::Trousers => clothes.trousers,
        skier::Color::Jacket => clothes.jacket,
        skier::Color::Helmet => clothes.helmet,
    }
}
