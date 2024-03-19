use std::collections::HashMap;

use commons::color::Rgb;

use crate::model::ability::Ability;
use crate::model::skier::{Clothes, Color, Skier};

pub struct System {
    colors: AbilityColors,
    pub mode: Mode,
}

pub struct AbilityColors {
    pub intermedite: Rgb<f32>,
    pub advanced: Rgb<f32>,
    pub expert: Rgb<f32>,
}

pub enum Mode {
    Clothes,
    Ability,
}

impl System {
    pub fn new(colors: AbilityColors) -> System {
        System {
            colors,
            mode: Mode::Clothes,
        }
    }

    pub fn run(
        &self,
        skiers: &HashMap<usize, Skier>,
        clothes: &mut HashMap<usize, Clothes<Rgb<f32>>>,
    ) {
        for (id, skier) in skiers {
            let skier_clothes = match self.mode {
                Mode::Clothes => Some((&skier.clothes).into()),
                Mode::Ability => self.ability_clothes(&skier.ability),
            };
            if let Some(skier_clothes) = skier_clothes {
                clothes.insert(*id, skier_clothes);
            }
        }
    }

    fn ability_clothes(&self, ability: &Ability) -> Option<Clothes<Rgb<f32>>> {
        let color = self.ability_color(ability)?;
        Some(Clothes {
            skis: color,
            trousers: color,
            jacket: color,
            helmet: color,
        })
    }

    fn ability_color(&self, ability: &Ability) -> Option<Rgb<f32>> {
        match ability {
            Ability::Intermediate => Some(self.colors.intermedite),
            Ability::Advanced => Some(self.colors.advanced),
            Ability::Expert => Some(self.colors.expert),
            _ => None,
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
