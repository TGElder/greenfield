use std::collections::HashMap;

use commons::geometry::{xy, XYRectangle};
use engine::binding::Binding;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::handlers::selection;
use crate::model::ability::Ability;
use crate::model::building::{Building, Roof};
use crate::model::skier::{Clothes, Color, Skier};
use crate::services::id_allocator;
use crate::systems::{building_artist, tree_artist};

pub const CUBIC_METERS_PER_SKIER: u32 = 27;

pub const HEIGHT_MIN: u32 = 3;
pub const HEIGHT_MAX: u32 = 36;
pub const HEIGHT_INTERVAL: u32 = 3;

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
    pub bindings: Bindings,
    pub state: State,
}

pub struct Bindings {
    pub start_building: Binding,
    pub finish_building: Binding,
    pub decrease_height: Binding,
    pub increase_height: Binding,
    pub toggle_roof: Binding,
}

#[derive(Clone, Copy)]
pub enum State {
    Selecting,
    Editing { building_id: usize },
}

pub struct Parameters<'a> {
    pub event: &'a engine::events::Event,
    pub selection: &'a mut selection::Handler,
    pub id_allocator: &'a mut id_allocator::Service,
    pub buildings: &'a mut HashMap<usize, Building>,
    pub locations: &'a mut HashMap<usize, usize>,
    pub skiers: &'a mut HashMap<usize, Skier>,
    pub building_artist: &'a mut building_artist::System,
    pub tree_artist: &'a mut tree_artist::System,
}

impl Handler {
    pub fn new(bindings: Bindings) -> Handler {
        Handler {
            bindings,
            state: State::Selecting,
        }
    }

    pub fn handle(&mut self, parameters: Parameters<'_>) {
        self.state = match self.state {
            State::Selecting => self.select(parameters),
            State::Editing { building_id } => self.edit(building_id, parameters),
        }
    }

    pub fn select(
        &mut self,
        Parameters {
            event,
            selection,
            id_allocator,
            buildings,
            tree_artist,
            ..
        }: Parameters<'_>,
    ) -> State {
        if !self.bindings.start_building.binds_event(event) {
            return self.state;
        }
        let Some(grid) = &selection.grid else {
            return self.state;
        };

        for position in grid.iter() {
            if !grid[position] {
                // can only build rectangle buildings
                return self.state;
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
            height: HEIGHT_MIN,
            roof: Roof::Default,
            under_construction: true,
            windows: vec![],
        };

        buildings.insert(building_id, building);

        // updating art

        tree_artist.update();

        // clearing selection

        selection.clear_selection();

        State::Editing { building_id }
    }

    pub fn edit(
        &mut self,
        building_id: usize,
        Parameters {
            event,
            id_allocator,
            buildings,
            locations,
            skiers,
            building_artist,
            ..
        }: Parameters<'_>,
    ) -> State {
        let Some(building) = buildings.get_mut(&building_id) else {
            return State::Selecting;
        };

        if self.bindings.finish_building.binds_event(event) {
            // creating skiers

            let volume_cubic_meters = ((building.footprint.width() - 1)
                * (building.footprint.height() - 1))
                * building.height;
            let capacity = volume_cubic_meters / CUBIC_METERS_PER_SKIER;

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
                        hotel_id: building_id,
                    },
                );
            }

            println!("INFO: {} total skiers", skiers.len());

            building.under_construction = false;
            building_artist.redraw(building_id);
            State::Selecting
        } else if self.bindings.decrease_height.binds_event(event) {
            building.height = (building.height.saturating_sub(HEIGHT_INTERVAL)).max(HEIGHT_MIN);
            building_artist.redraw(building_id);
            self.state
        } else if self.bindings.increase_height.binds_event(event) {
            building.height = (building.height.saturating_add(HEIGHT_INTERVAL)).min(HEIGHT_MAX);
            building_artist.redraw(building_id);
            self.state
        } else if self.bindings.toggle_roof.binds_event(event) {
            building.roof = match building.roof {
                Roof::Default => Roof::Rotated,
                Roof::Rotated => Roof::Default,
            };
            building_artist.redraw(building_id);
            self.state
        } else {
            self.state
        }
    }
}
