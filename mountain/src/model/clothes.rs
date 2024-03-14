use commons::color::Rgb;
use serde::{Deserialize, Serialize};

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
            Color::Grey => Rgb::new(0.22, 0.22, 0.22),
            Color::White => Rgb::new(1.0, 1.0, 1.0),
            Color::Color1 => Rgb::new(1.0, 0.0, 0.54),
            Color::Color2 => Rgb::new(0.0, 0.0, 1.0),
            Color::Color3 => Rgb::new(0.17, 0.77, 0.0),
            Color::Color4 => Rgb::new(0.0, 0.45, 0.88),
            Color::Color5 => Rgb::new(0.95, 0.14, 0.58),
        }
    }
}

#[derive(Clone, Copy, Default, Serialize, Deserialize)]
pub struct Clothes<T> {
    pub skis: T,
    pub trousers: T,
    pub jacket: T,
    pub helmet: T,
}

impl From<&Clothes<Color>> for Clothes<Rgb<f32>> {
    fn from(
        Clothes {
            skis,
            trousers,
            jacket,
            helmet,
        }: &Clothes<Color>,
    ) -> Self {
        Clothes {
            skis: skis.rgb(),
            trousers: trousers.rgb(),
            jacket: jacket.rgb(),
            helmet: helmet.rgb(),
        }
    }
}
