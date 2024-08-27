use std::collections::HashMap;
use std::vec;

use commons::geometry::{xy, xyz, XYRectangle, XY};
use commons::grid::Grid;
use commons::unsafe_ordering::unsafe_ordering;
use engine::binding::Binding;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::controllers::Result::{self, Action, NoAction};
use crate::handlers::selection;
use crate::model::ability::Ability;
use crate::model::building::{Building, Roof, Window};
use crate::model::direction::Direction;
use crate::model::skier::{Clothes, Color, Skier};
use crate::services::id_allocator;
use crate::systems::{building_artist, tree_artist, window_artist};

pub const HEIGHT_MIN: u32 = 3;
pub const HEIGHT_MAX: u32 = 36;
pub const HEIGHT_INTERVAL: u32 = 3;

pub const WINDOW_INTERVAL: f32 = 3.0;
pub const WINDOW_FREE_LENGTH: f32 = 2.0;

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

pub struct Controller {
    pub state: State,
}

pub struct Bindings {
    pub decrease_height: Binding,
    pub increase_height: Binding,
    pub toggle_roof: Binding,
}

#[derive(Clone, Copy, PartialEq)]
pub enum State {
    Selecting,
    Editing { building_id: usize },
}

pub struct Parameters<'a> {
    pub action_binding: &'a Binding,
    pub bindings: &'a Bindings,
    pub event: &'a engine::events::Event,
    pub terrain: &'a Grid<f32>,
    pub selection: &'a mut selection::Handler,
    pub id_allocator: &'a mut id_allocator::Service,
    pub buildings: &'a mut HashMap<usize, Building>,
    pub locations: &'a mut HashMap<usize, usize>,
    pub skiers: &'a mut HashMap<usize, Skier>,
    pub building_artist: &'a mut building_artist::System,
    pub tree_artist: &'a mut tree_artist::System,
    pub window_artist: &'a mut window_artist::System,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            state: State::Selecting,
        }
    }

    pub fn trigger(&mut self, parameters: Parameters<'_>) -> Result {
        let old_state = self.state;
        self.state = match self.state {
            State::Selecting => self.select(parameters),
            State::Editing { building_id } => self.edit(building_id, parameters),
        };

        if old_state == self.state {
            NoAction
        } else {
            Action
        }
    }

    pub fn select(
        &mut self,
        Parameters {
            action_binding,
            event,
            selection,
            id_allocator,
            buildings,
            tree_artist,
            ..
        }: Parameters<'_>,
    ) -> State {
        if !action_binding.binds_event(event) {
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
            roof: Roof::Peaked,
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
            action_binding,
            bindings,
            event,
            terrain,
            id_allocator,
            buildings,
            locations,
            skiers,
            building_artist,
            window_artist,
            ..
        }: Parameters<'_>,
    ) -> State {
        let Some(building) = buildings.get_mut(&building_id) else {
            return State::Selecting;
        };

        if action_binding.binds_event(event) {
            // creating skiers

            building.windows = windows(terrain, &building.footprint, building.height);

            let capacity = building.windows.len();
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
            window_artist.update();
            State::Selecting
        } else if bindings.decrease_height.binds_event(event) {
            building.height = (building.height.saturating_sub(HEIGHT_INTERVAL)).max(HEIGHT_MIN);
            building_artist.redraw(building_id);
            self.state
        } else if bindings.increase_height.binds_event(event) {
            building.height = (building.height.saturating_add(HEIGHT_INTERVAL)).min(HEIGHT_MAX);
            building_artist.redraw(building_id);
            self.state
        } else if bindings.toggle_roof.binds_event(event) {
            building.roof = match building.roof {
                Roof::Peaked => Roof::PeakedRotated,
                Roof::PeakedRotated => Roof::Flat,
                Roof::Flat => Roof::Peaked,
            };
            building_artist.redraw(building_id);
            self.state
        } else {
            self.state
        }
    }
}

pub fn windows(terrain: &Grid<f32>, footprint: &XYRectangle<u32>, height: u32) -> Vec<Window> {
    let corners = [
        footprint.from,
        xy(footprint.from.x, footprint.to.y),
        footprint.to,
        xy(footprint.to.x, footprint.from.y),
    ];

    let Some(base_z) = corners
        .iter()
        .filter(|&corner| terrain.in_bounds(corner))
        .map(|corner| terrain[corner])
        .max_by(unsafe_ordering)
    else {
        return vec![];
    };

    let floor_count = height / HEIGHT_INTERVAL;

    (0..4)
        .flat_map(|i| window_wall(&corners[i], &corners[(i + 1) % 4], base_z, floor_count))
        .collect()
}

pub fn window_wall<'a>(
    from: &'a XY<u32>,
    to: &'a XY<u32>,
    base_z: f32,
    floor_count: u32,
) -> impl Iterator<Item = Window> + 'a {
    (0..floor_count)
        .map(|floor| floor * HEIGHT_INTERVAL)
        .flat_map(move |floor_height| window_row(from, to, base_z, floor_height))
}

pub fn window_row(
    from: &XY<u32>,
    to: &XY<u32>,
    base_z: f32,
    floor_height: u32,
) -> impl Iterator<Item = Window> {
    let from = xy(from.x as f32, from.y as f32);
    let to = xy(to.x as f32, to.y as f32);
    let vector = from - to;
    let length = vector.magnitude();
    let angle = vector.angle();
    let available_length = (length - WINDOW_FREE_LENGTH).max(0.0);
    let window_count = (available_length / WINDOW_INTERVAL).floor() as u32;
    let margin = length - (window_count as f32 * WINDOW_INTERVAL);
    let offset = (WINDOW_INTERVAL + margin) / 2.0;

    let z = base_z + HEIGHT_INTERVAL as f32 / 2.0 + floor_height as f32;

    (0..window_count)
        .map(move |w| offset + (w as f32 * WINDOW_INTERVAL))
        .map(move |distance| distance / length)
        .map(move |p| from * (1.0 - p) + to * p)
        .map(move |XY { x, y }| xyz(x, y, z))
        .map(move |position| Window {
            position,
            direction: Direction::snap_to_direction(angle),
        })
}
