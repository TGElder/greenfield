use commons::color::Rgb;
use commons::geometry::xyz;

use engine::graphics::transform::{Recolor, Transform};
use engine::graphics::utils::{transformation_matrix, triangles_from_quads};
use engine::graphics::Graphics;

use crate::draw::model::{self, skier};
use crate::model::clothes::Clothes;
use crate::model::frame::{Frame, Model};

pub fn draw(graphics: &mut dyn Graphics, index: &usize, frame: &Frame) {
    let transformation = transformation_matrix(
        frame.position,
        frame.yaw,
        frame.pitch,
        0.0,
        xyz(1.0, 1.0, 1.0),
    );

    let quads = match frame.model {
        Model::Standing {
            skis: false,
            clothes,
        } => model::skier_standing::model()
            .quads
            .recolor(&|color| get_rgb(&clothes, color)),
        Model::Standing {
            skis: true,
            clothes,
        } => model::skier_standing::model()
            .quads
            .recolor(&|color| get_rgb(&clothes, color)),
        Model::Sitting { clothes } => model::skier_sitting::model()
            .quads
            .recolor(&|color| get_rgb(&clothes, color)),
        Model::Chair => model::chair::model().quads,
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
