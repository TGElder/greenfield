use std::collections::HashMap;

use commons::geometry::{xy, XYRectangle};
use engine::binding::Binding;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::handlers::selection;
use crate::model::ability::Ability;
use crate::model::building::Building;
use crate::model::skier::{Clothes, Color, Skier};
use crate::services::id_allocator;

pub const CUBIC_METERS_PER_SKIER: u32 = 27;

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

pub struct Handler {
    pub binding: Binding,
}

pub struct Parameters<'a> {
    pub event: &'a engine::events::Event,
    pub selection: &'a mut selection::Handler,
    pub id_allocator: &'a mut id_allocator::Service,
    pub buildings: &'a mut HashMap<usize, Building>,
    pub locations: &'a mut HashMap<usize, usize>,
    pub skiers: &'a mut HashMap<usize, Skier>,
}

impl Handler {
    pub fn handle(
        &mut self,
        Parameters {
            event,
            selection,
            id_allocator,
            buildings,
            locations,
            skiers,
        }: Parameters<'_>,
    ) {
        if !self.binding.binds_event(event) {
            return;
        }

        let Some(grid) = &selection.grid else {
            return;
        };

        for position in grid.iter() {
            if !grid[position] {
                // can only build rectangle buildings
                return;
            }
        }

        let rectangle = XYRectangle {
            from: *grid.origin(),
            to: *grid.origin() + xy(grid.width(), grid.height()),
        };

        // creating building

        let building_id = id_allocator.next_id();
        let building = Building {
            footprint: rectangle,
            height: 6,
        };

        let volume_cubic_meters =
            ((rectangle.width() - 1) * (rectangle.height() - 1)) * building.height;
        let capacity = volume_cubic_meters / CUBIC_METERS_PER_SKIER;

        buildings.insert(building_id, building);

        // creating skiers

        println!("INFO: Spawing {} skiers", capacity);

        let mut rng = thread_rng();
        for _ in 0..capacity {
            let skier_id = id_allocator.next_id();

            locations.insert(skier_id, building_id);

            skiers.insert(
                skier_id,
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
        }

        // clearing selection

        selection.clear_selection();
    }
}
