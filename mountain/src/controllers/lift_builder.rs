use std::collections::{HashMap, HashSet};

use commons::geometry::{xy, XY};
use commons::grid::Grid;

use crate::controllers::Result::{self, Action, NoAction};
use crate::handlers::structure_builder;
use crate::model::carousel::{Car, Carousel};
use crate::model::direction::Direction;
use crate::model::entrance::Entrance;
use crate::model::exit::Exit;
use crate::model::lift::{self, Lift, Segment};
use crate::model::open;
use crate::model::reservation::Reservation;
use crate::model::skiing::State;
use crate::model::structure::{get_wire_path, get_wire_path_out, Structure};
use crate::network::velocity_encoding::{encode_velocity, VELOCITY_LEVELS};
use crate::services::id_allocator;
use crate::systems::{messenger, piste_computer};
use crate::utils;

pub const LIFT_VELOCITY: f32 = 2.0;
pub const CAR_INTERVAL_METERS: f32 = 10.0;

pub struct Controller {
    from: Option<XY<u32>>,
}

pub struct Parameters<'a> {
    pub terrain: &'a Grid<f32>,
    pub piste_map: &'a Grid<Option<usize>>,
    pub structures: &'a HashMap<usize, Structure>,
    pub lifts: &'a mut HashMap<usize, Lift>,
    pub open: &'a mut HashMap<usize, open::Status>,
    pub id_allocator: &'a mut id_allocator::Service,
    pub carousels: &'a mut HashMap<usize, Carousel>,
    pub cars: &'a mut HashMap<usize, Car>,
    pub exits: &'a mut HashMap<usize, Exit>,
    pub entrances: &'a mut HashMap<usize, Entrance>,
    pub reservations: &'a mut Grid<HashMap<usize, Reservation>>,
    pub parents: &'a mut HashMap<usize, usize>,
    pub children: &'a mut HashMap<usize, Vec<usize>>,
    pub piste_computer: &'a mut piste_computer::System,
    pub structure_builder: &'a mut structure_builder::Handler,
    pub messenger: &'a mut messenger::System,
}

impl Controller {
    pub fn new() -> Controller {
        Controller { from: None }
    }

    pub fn trigger(
        &mut self,
        Parameters {
            terrain,
            piste_map,
            lifts,
            open,
            id_allocator,
            carousels,
            cars,
            exits,
            entrances,
            reservations,
            parents,
            children,
            piste_computer,
            structures,
            structure_builder,
            messenger,
        }: Parameters<'_>,
    ) -> Result {
        let lift_structures = &mut structure_builder.structures;

        if lift_structures.is_empty() || lift_structures.len() < 2 {
            return NoAction;
        }

        // create lift

        let from_structure = &structures[lift_structures.first().unwrap()];
        let from_position = get_wire_path_out(from_structure, terrain)[0][0];
        let from = xy(
            from_position.x.round() as u32,
            from_position.y.round() as u32,
        );

        let to_structure = &structures[lift_structures.last().unwrap()];
        let to_position = get_wire_path_out(to_structure, terrain)[0][0];
        let to = xy(to_position.x.round() as u32, to_position.y.round() as u32);

        let Some(origin_piste_id) = piste_map[from] else {
            messenger.send("Lift needs piste at start position!");
            self.from = None;
            return NoAction;
        };
        let Some(destination_piste_id) = piste_map[to] else {
            messenger.send("Lift needs piste at end position!");
            self.from = None;
            return NoAction;
        };

        let lift_id = id_allocator.next_id();
        let carousel_id = id_allocator.next_id();
        let pick_up_id = id_allocator.next_id();
        let drop_off_id = id_allocator.next_id();

        children.entry(lift_id).or_default().append(&mut vec![
            carousel_id,
            pick_up_id,
            drop_off_id,
        ]);
        parents.entry(carousel_id).insert_entry(lift_id);
        parents.entry(pick_up_id).insert_entry(lift_id);
        parents.entry(drop_off_id).insert_entry(lift_id);

        let segments = get_wire_path(
            &lift_structures
                .iter()
                .map(|id| &structures[id])
                .collect::<Vec<_>>(),
            terrain,
        )
        .drain(..)
        .map(|segment| Segment::new(segment[0], segment[1]))
        .collect();
        let travel_direction = get_direction(&from, &to);

        let lift = Lift {
            segments,
            pick_up: lift::Portal {
                id: pick_up_id,
                segment: 0,
                state: State {
                    position: from,
                    travel_direction,
                    velocity: 0,
                },
            },
            drop_off: lift::Portal {
                id: drop_off_id,
                segment: lift_structures.len(), // le haque
                state: State {
                    position: to,
                    travel_direction,
                    velocity: encode_velocity(&LIFT_VELOCITY).unwrap_or(VELOCITY_LEVELS - 1),
                },
            },
            carousel_id,
        };

        // opening lift

        open.insert(lift_id, open::Status::Open);
        open.insert(lift.pick_up.id, open::Status::Open);
        open.insert(lift.drop_off.id, open::Status::Open);

        // setup carousel

        let new_cars =
            utils::carousel::create_cars(carousel_id, &lift.segments, &CAR_INTERVAL_METERS);

        let car_ids = (0..new_cars.len())
            .map(|_| id_allocator.next_id())
            .collect::<Vec<_>>();

        children
            .entry(carousel_id)
            .or_default()
            .append(&mut car_ids.clone());

        car_ids.iter().zip(new_cars).for_each(|(id, car)| {
            cars.insert(*id, car);
            parents.insert(*id, carousel_id);
        });

        carousels.insert(
            carousel_id,
            Carousel {
                lift_id,
                velocity: LIFT_VELOCITY,
                car_ids,
            },
        );

        // setup exit

        exits.insert(
            lift.pick_up.id,
            Exit {
                origin_piste_id,
                stationary_states: HashSet::from([lift.pick_up.state.stationary()]),
            },
        );

        // setup entrance

        entrances.insert(
            lift.drop_off.id,
            Entrance {
                destination_piste_id,
                stationary_states: HashSet::from([lift.drop_off.state.stationary()]),
            },
        );

        // reserve pick up position

        reservations[lift.pick_up.state.position].insert(lift.pick_up.id, Reservation::Structure);

        // clear from position

        self.from = None;

        // register lift

        lifts.insert(lift_id, lift);

        // recomputing pistes
        piste_computer.compute(origin_piste_id);
        piste_computer.compute(destination_piste_id);

        structure_builder.reset();

        Action
    }
}

fn get_direction(from: &XY<u32>, to: &XY<u32>) -> Direction {
    let vector = xy(to.x as f32 - from.x as f32, to.y as f32 - from.y as f32);
    Direction::snap_to_direction(vector.angle())
}
