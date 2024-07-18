use commons::color::Rgb;
use serde::{Deserialize, Serialize};

use crate::model::ability::Ability;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Skier {
    pub clothes: Clothes<Color>,
    pub ability: Ability,
    pub hotel_id: usize,
}

#[derive(Clone, Copy, Default, Debug, Serialize, Deserialize)]
pub struct Clothes<T> {
    pub skis: T,
    pub trousers: T,
    pub jacket: T,
    pub helmet: T,
}

#[derive(Clone, Copy, Default, Serialize, Deserialize)]
pub enum Color {
    #[default]
    Black,
    Grey,
    White,
    Color1,
    Color2,
    Color3,
    Color4,
    Color5,
}

impl Color {
    pub fn rgb(&self) -> Rgb<f32> {
        match self {
            Color::Black => Rgb::new(0.0, 0.0, 0.0),
            Color::Grey => Rgb::new(0.502, 0.502, 0.502),
            Color::White => Rgb::new(1.0, 1.0, 1.0),
            Color::Color1 => Rgb::new(1.0, 0.0, 0.756),
            Color::Color2 => Rgb::new(0.0, 0.0, 1.0),
            Color::Color3 => Rgb::new(0.447, 0.888, 0.0),
            Color::Color4 => Rgb::new(0.0, 0.696, 0.944),
            Color::Color5 => Rgb::new(0.977, 0.409, 0.781),
        }
    }
}
