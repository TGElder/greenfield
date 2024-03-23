use std::collections::HashMap;

use commons::geometry::{xy, XY, XYZ};
use engine::binding::Binding;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::model::ability::Ability;
use crate::model::direction::Direction;
use crate::model::skier::{Clothes, Color, Skier};
use crate::model::skiing;
use crate::services::id_allocator;

pub struct Handler {
    pub binding: Binding,
}

const ABILITIES: [Ability; 3] = [Ability::Intermediate, Ability::Advanced, Ability::Expert];

const SKI_COLORS: [Color; 5] = [
    Color::Color1,
    Color::Color2,
    Color::Color3,
    Color::Color4,
    Color::Color5,
];

const SUIT_COLORS: [Color; 8] = [
    Color::Black,
    Color::Grey,
    Color::White,
    Color::Color1,
    Color::Color2,
    Color::Color3,
    Color::Color4,
    Color::Color5,
];

const HELMET_COLORS: [Color; 2] = [Color::Black, Color::Grey];

impl Handler {
    pub fn handle(
        &self,
        event: &engine::events::Event,
        mouse_xy: &Option<XY<u32>>,
        plans: &mut HashMap<usize, skiing::Plan>,
        skiers: &mut HashMap<usize, Skier>,
        id_allocator: &mut id_allocator::Service,
        graphics: &mut dyn engine::graphics::Graphics,
    ) {
        if !self.binding.binds_event(event) {
            return;
        }

        let Some(mouse_xy) = mouse_xy else { return };
        let Ok(XYZ { x, y, .. }) = graphics.world_xyz_at(mouse_xy) else {
            return;
        };

        let id = id_allocator.next_id();

        plans.insert(
            id,
            skiing::Plan::Stationary(skiing::State {
                position: xy(x.round() as u32, y.round() as u32),
                velocity: 1,
                travel_direction: Direction::NorthEast,
            }),
        );

        let mut rng = thread_rng();
        skiers.insert(
            id,
            Skier {
                ability: *ABILITIES.choose(&mut rng).unwrap(),
                clothes: Clothes {
                    skis: *SKI_COLORS.choose(&mut rng).unwrap(),
                    trousers: *SUIT_COLORS.choose(&mut rng).unwrap(),
                    jacket: *SUIT_COLORS.choose(&mut rng).unwrap(),
                    helmet: *HELMET_COLORS.choose(&mut rng).unwrap(),
                },
            },
        );

        println!("{} skiers", skiers.len());
    }
}
