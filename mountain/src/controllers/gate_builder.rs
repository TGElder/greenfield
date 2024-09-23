use std::collections::{HashMap, HashSet};

use commons::geometry::{xy, XYRectangle};
use commons::grid::Grid;

use crate::controllers::Result::{self, Action, NoAction};
use crate::model::direction::DIRECTIONS;
use crate::model::entrance::Entrance;
use crate::model::exit::Exit;
use crate::model::gate::Gate;
use crate::model::reservation::Reservation;
use crate::model::selection::Selection;
use crate::model::skiing::State;
use crate::services::id_allocator;
use crate::systems::{messenger, terrain_artist};

pub struct Parameters<'a> {
    pub terrain: &'a Grid<f32>,
    pub piste_map: &'a Grid<Option<usize>>,
    pub selection: &'a mut Selection,
    pub terrain_artist: &'a mut terrain_artist::System,
    pub id_allocator: &'a mut id_allocator::Service,
    pub gates: &'a mut HashMap<usize, Gate>,
    pub entrances: &'a mut HashMap<usize, Entrance>,
    pub exits: &'a mut HashMap<usize, Exit>,
    pub open: &'a mut HashSet<usize>,
    pub reservations: &'a mut Grid<HashMap<usize, Reservation>>,
    pub messenger: &'a mut messenger::System,
}

pub fn trigger(
    Parameters {
        terrain,
        piste_map,
        selection,
        terrain_artist,
        id_allocator,
        gates,
        entrances,
        exits,
        open,
        reservations,
        messenger,
    }: Parameters<'_>,
) -> Result {
    let (Some(&origin), Some(grid)) = (selection.cells.first(), &selection.grid) else {
        return NoAction;
    };

    let Ok(rectangle) = grid.rectangle() else {
        return NoAction;
    };

    // clearing selection
    selection.cells.clear();
    terrain_artist.update_overlay(rectangle);

    // create gate

    if rectangle.width() == 0 || rectangle.height() == 0 {
        messenger.send("Entrance must not be zero length");
        return NoAction;
    }
    let maybe_configuration = try_get_vertical_configuration(rectangle, piste_map, messenger)
        .or_else(|| try_get_horizontal_configuration(rectangle, piste_map, messenger));

    let Some(configuration) = maybe_configuration else {
        return NoAction;
    };

    let gate = match configuration.orientation {
        Orientation::Vertical => Gate {
            footprint: XYRectangle {
                from: xy(rectangle.to.x, rectangle.from.y),
                to: xy(rectangle.to.x, rectangle.to.y + 1),
            },
        },
        Orientation::Horizontal => Gate {
            footprint: XYRectangle {
                from: xy(rectangle.from.x, rectangle.to.y),
                to: xy(rectangle.to.x + 1, rectangle.to.y),
            },
        },
    };

    // creating gate

    let gate_id = id_allocator.next_id();

    // reserving footprint

    gate.footprint.iter().for_each(|position| {
        reservations[position].insert(gate_id, Reservation::Structure);
    });

    // creating entrance and exit

    let origin_piste_id = piste_map[origin].unwrap();
    let destination_piste_id =
        (configuration.pistes[0] + configuration.pistes[1]) - origin_piste_id;

    let stationary_states = gate
        .footprint
        .iter()
        .flat_map(|position| {
            DIRECTIONS.iter().map(move |&travel_direction| State {
                position,
                velocity: 0,
                travel_direction,
            })
        })
        .collect::<HashSet<_>>();
    entrances.insert(
        gate_id,
        Entrance {
            destination_piste_id,
            stationary_states: stationary_states.clone(),
            altitude_meters: gate
                .footprint
                .iter()
                .map(|position| terrain[position])
                .sum::<f32>()
                / gate.footprint.iter().count() as f32,
        },
    );
    exits.insert(
        gate_id,
        Exit {
            origin_piste_id,
            stationary_states,
        },
    );

    // opening gate

    open.insert(gate_id);

    // inserting gate

    gates.insert(gate_id, gate);

    Action
}

fn try_get_vertical_configuration(
    rectangle: XYRectangle<u32>,
    piste_map: &Grid<Option<usize>>,
    messenger: &mut messenger::System,
) -> Option<Configuration> {
    if rectangle.width() != 2 {
        messenger.send("Not vertical gate - selection must be 2 wide");
        return None;
    }

    let mut pistes = [0; 2];

    for (index, x) in (rectangle.from.x..=rectangle.to.x).enumerate() {
        let value = piste_map[xy(x, rectangle.from.y)]?;
        for y in rectangle.from.y..=rectangle.to.y {
            if piste_map[xy(x, y)]? != value {
                messenger.send("Not vertical gate - column does not contain piste");
                return None;
            }
        }
        pistes[index] = value;
    }

    if pistes[0] == pistes[1] {
        messenger.send("Not vertical gate - same piste on both sides");
        return None;
    }

    Some(Configuration {
        orientation: Orientation::Vertical,
        pistes,
    })
}

fn try_get_horizontal_configuration(
    rectangle: XYRectangle<u32>,
    piste_map: &Grid<Option<usize>>,
    messenger: &mut messenger::System,
) -> Option<Configuration> {
    if rectangle.height() != 2 {
        messenger.send("Not horizontal gate - selection must be 2 high");
        return None;
    }

    let mut pistes = [0; 2];

    for (index, y) in (rectangle.from.y..=rectangle.to.y).enumerate() {
        let value = piste_map[xy(rectangle.from.x, y)]?;
        for x in rectangle.from.x..=rectangle.to.x {
            if piste_map[xy(x, y)]? != value {
                messenger.send("Not horizontal gate - row does not contain piste");
                return None;
            }
        }
        pistes[index] = value;
    }

    if pistes[0] == pistes[1] {
        messenger.send("Not horizontal gate - same piste on both sides");
        return None;
    }

    Some(Configuration {
        orientation: Orientation::Horizontal,
        pistes,
    })
}

struct Configuration {
    orientation: Orientation,
    pistes: [usize; 2],
}

enum Orientation {
    Vertical,
    Horizontal,
}
