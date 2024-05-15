use std::collections::{HashMap, HashSet};

use commons::geometry::{xy, XYRectangle};
use commons::grid::Grid;
use engine::binding::Binding;

use crate::handlers::selection;
use crate::model::direction::DIRECTIONS;
use crate::model::entrance::Entrance;
use crate::model::exit::Exit;
use crate::model::gate::Gate;
use crate::model::reservation::Reservation;
use crate::model::skiing::State;
use crate::services::id_allocator;
use crate::systems::terrain_artist;

pub struct Handler {
    pub binding: Binding,
}

pub struct Parameters<'a> {
    pub event: &'a engine::events::Event,
    pub terrain: &'a Grid<f32>,
    pub piste_map: &'a Grid<Option<usize>>,
    pub selection: &'a mut selection::Handler,
    pub terrain_artist: &'a mut terrain_artist::System,
    pub id_allocator: &'a mut id_allocator::Service,
    pub gates: &'a mut HashMap<usize, Gate>,
    pub entrances: &'a mut HashMap<usize, Entrance>,
    pub exits: &'a mut HashMap<usize, Exit>,
    pub open: &'a mut HashSet<usize>,
    pub reservations: &'a mut Grid<HashMap<usize, Reservation>>,
}

impl Handler {
    pub fn new(binding: Binding) -> Handler {
        Handler { binding }
    }

    pub fn handle(
        &mut self,
        Parameters {
            event,
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
        }: Parameters<'_>,
    ) {
        if !self.binding.binds_event(event) {
            return;
        }
        let (Some(&origin), Some(grid)) = (selection.cells.first(), &selection.grid) else {
            return;
        };

        let Ok(rectangle) = grid.rectangle() else {
            return;
        };

        // clearing selection

        selection.clear_selection();
        terrain_artist.update(rectangle);

        // create gate

        if rectangle.width() == 0 || rectangle.height() == 0 {
            println!("INFO: Entrance must not be zero length");
            return;
        }
        let maybe_configuration = try_get_vertical_configuration(rectangle, piste_map)
            .or_else(|| try_get_horizontal_configuration(rectangle, piste_map));

        let Some(configuration) = maybe_configuration else {
            return;
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
    }
}

fn try_get_vertical_configuration(
    rectangle: XYRectangle<u32>,
    piste_map: &Grid<Option<usize>>,
) -> Option<Configuration> {
    if rectangle.width() != 2 {
        println!("INFO: Not vertical gate - selection must be 2 wide");
        return None;
    }

    let mut pistes = [0; 2];

    for (index, x) in (rectangle.from.x..=rectangle.to.x).enumerate() {
        let value = piste_map[xy(x, rectangle.from.y)]?;
        for y in rectangle.from.y..=rectangle.to.y {
            if piste_map[xy(x, y)]? != value {
                println!("INFO: Not vertical gate - column does not contain piste");
                return None;
            }
        }
        pistes[index] = value;
    }

    if pistes[0] == pistes[1] {
        println!("INFO: Not vertical gate - same piste on both sides");
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
) -> Option<Configuration> {
    if rectangle.height() != 2 {
        println!("INFO: Not horizontal gate - selection must be 2 high");
        return None;
    }

    let mut pistes = [0; 2];

    for (index, y) in (rectangle.from.y..=rectangle.to.y).enumerate() {
        let value = piste_map[xy(rectangle.from.x, y)]?;
        for x in rectangle.from.x..=rectangle.to.x {
            if piste_map[xy(x, y)]? != value {
                println!("INFO: Not horizontal gate - row does not contain piste");
                return None;
            }
        }
        pistes[index] = value;
    }

    if pistes[0] == pistes[1] {
        println!("INFO: Not horizontal gate - same piste on both sides");
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
