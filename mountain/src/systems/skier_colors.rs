use std::collections::HashMap;

use commons::color::Rgb;

use crate::model::skier::{Clothes, Color, Skier};

pub struct System {
    mode: Mode,
}

pub enum Mode {
    Clothes,
    _Ability,
}

impl System {
    pub fn new() -> System {
        System {
            mode: Mode::Clothes,
        }
    }

    pub fn run(
        &self,
        skiers: &HashMap<usize, Skier>,
        clothes: &mut HashMap<usize, Clothes<Rgb<f32>>>,
    ) {
        clothes.clear();
        for (id, skier) in skiers {
            if let Mode::Clothes = self.mode {
                clothes.insert(*id, (&skier.clothes).into());
            };
        }
    }
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
