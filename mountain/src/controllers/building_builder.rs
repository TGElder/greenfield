use std::collections::HashMap;
use std::vec;

use commons::geometry::{xy, xyz, XYRectangle, XY};
use commons::grid::Grid;
use commons::unsafe_ordering::unsafe_ordering;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::controllers::Result::{self, Action, NoAction};
use crate::model::ability::Ability;
use crate::model::building::{Building, Roof, Window};
use crate::model::direction::Direction;
use crate::model::selection::Selection;
use crate::model::skier::{Clothes, Color, Skier};
use crate::services::id_allocator;
use crate::systems::{building_artist, messenger, tree_artist, window_artist};

pub const HEIGHT_MIN: u32 = 3;
pub const HEIGHT_MAX: u32 = 60;
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
    state: State,
}

#[derive(Clone, Copy, PartialEq)]
pub enum State {
    Selecting,
    Editing { building_id: usize },
}

pub struct SelectParameters<'a> {
    pub selection: &'a mut Selection,
    pub id_allocator: &'a mut id_allocator::Service,
    pub buildings: &'a mut HashMap<usize, Building>,
    pub tree_artist: &'a mut tree_artist::System,
}

pub struct FinalizeParameters<'a> {
    pub terrain: &'a Grid<f32>,
    pub id_allocator: &'a mut id_allocator::Service,
    pub buildings: &'a mut HashMap<usize, Building>,
    pub locations: &'a mut HashMap<usize, usize>,
    pub skiers: &'a mut HashMap<usize, Skier>,
    pub building_artist: &'a mut building_artist::System,
    pub window_artist: &'a mut window_artist::System,
    pub messenger: &'a mut messenger::System,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            state: State::Selecting,
        }
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn select(
        &mut self,
        SelectParameters {
            selection,
            id_allocator,
            buildings,
            tree_artist,
            ..
        }: SelectParameters<'_>,
    ) -> Result {
        let State::Selecting = self.state else {
            return NoAction;
        };

        let Some(grid) = &selection.grid else {
            return NoAction;
        };

        for position in grid.iter() {
            if !grid[position] {
                // can only build rectangle buildings
                return NoAction;
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

        selection.cells.clear();

        self.state = State::Editing { building_id };
        Action
    }

    pub fn finalize(
        &mut self,
        FinalizeParameters {
            terrain,
            id_allocator,
            buildings,
            locations,
            skiers,
            building_artist,
            window_artist,
            messenger,
        }: FinalizeParameters<'_>,
    ) -> Result {
        let State::Editing { building_id } = self.state else {
            return NoAction;
        };

        let Some(building) = buildings.get_mut(&building_id) else {
            self.state = State::Selecting;
            return NoAction;
        };

        if building.under_construction {
            return NoAction;
        }

        // creating skiers

        building.windows = windows(terrain, &building.footprint, building.height);

        let capacity = building.windows.len();
        messenger.send(format!("Spawing {} skiers", capacity));

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

        messenger.send(format!("{} total skiers", skiers.len()));

        building.under_construction = false;
        building_artist.redraw(building_id);
        window_artist.update();

        self.state = State::Selecting;
        Action
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
