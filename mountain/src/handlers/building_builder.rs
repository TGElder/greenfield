use std::collections::HashMap;

use commons::geometry::{xy, XYRectangle, XY};
use engine::binding::Binding;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::handlers::selection;
use crate::model::ability::Ability;
use crate::model::building::Building;
use crate::model::skier::{Clothes, Color, Skier};
use crate::services::id_allocator;
use crate::systems::tree_artist;

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
    construction: Option<Construction>,
}

pub struct Construction {
    building_id: usize,
    reference_y: u32,
}

pub struct Parameters<'a> {
    pub event: &'a engine::events::Event,
    pub mouse_xy: &'a Option<XY<u32>>,
    pub selection: &'a mut selection::Handler,
    pub id_allocator: &'a mut id_allocator::Service,
    pub buildings: &'a mut HashMap<usize, Building>,
    pub locations: &'a mut HashMap<usize, usize>,
    pub skiers: &'a mut HashMap<usize, Skier>,
    pub tree_artist: &'a mut tree_artist::System,
}

impl Handler {
    pub fn new(binding: Binding) -> Handler {
        Handler {
            binding,
            construction: None,
        }
    }
    pub fn handle(
        &mut self,
        Parameters {
            event,
            mouse_xy,
            selection,
            id_allocator,
            buildings,
            locations,
            skiers,
            tree_artist,
        }: Parameters<'_>,
    ) {
        let Some(XY { y, .. }) = mouse_xy else { return };

        if !self.binding.binds_event(event) {
            if let Some(construction) = &self.construction {
                let Some(building) = buildings.get_mut(&construction.building_id) else {
                    // TODO clear construction
                    return;
                };
                building.height = ((construction.reference_y as f32 - *y as f32) / 10.0)
                    .max(1.0)
                    .round() as u32
                    * 3;
                dbg!(building.height);
            }
            return;
        }

        if let Some(construction) = &self.construction {
            let Some(building) = buildings.get_mut(&construction.building_id) else {
                // TODO clear construction
                return;
            };

            // creating skiers

            let volume_cubic_meters = ((building.footprint.width() - 1)
                * (building.footprint.height() - 1))
                * building.height;
            let capacity = volume_cubic_meters / CUBIC_METERS_PER_SKIER;

            println!("INFO: Spawing {} skiers", capacity);

            let mut rng = thread_rng();
            for _ in 0..capacity {
                let skier_id = id_allocator.next_id();

                locations.insert(skier_id, construction.building_id);

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
            self.construction = None;
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
            height: 1,
        };

        buildings.insert(building_id, building);

        // updating art

        tree_artist.update();

        // clearing selection

        selection.clear_selection();

        // setting construction

        self.construction = Some(Construction {
            building_id,
            reference_y: *y,
        });
    }
}
