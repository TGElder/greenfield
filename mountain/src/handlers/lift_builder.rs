use std::collections::{HashMap, HashSet};
use std::f32::consts::PI;
use std::iter::once;

use commons::curves::approximate_curve;
use commons::geometry::{xy, xyz, XY, XYZ};
use commons::grid::{Grid, CORNERS};
use engine::binding::Binding;

use crate::handlers::HandlerResult::{self, EventConsumed, EventPersists};
use crate::model::carousel::{Car, Carousel};
use crate::model::direction::Direction;
use crate::model::entrance::Entrance;
use crate::model::exit::Exit;
use crate::model::lift::{self, Lift, Segment};
use crate::model::reservation::Reservation;
use crate::model::skiing::State;
use crate::network::velocity_encoding::{encode_velocity, VELOCITY_LEVELS};
use crate::services::id_allocator;
use crate::utils;

pub const LIFT_VELOCITY: f32 = 2.0;
pub const CAR_INTERVAL_METERS: f32 = 10.0;
pub const CIRCLE_SEGMENTS: u8 = 16;
pub const CURVE_INCREMENT: f32 = (2.0 * PI) / CIRCLE_SEGMENTS as f32;
pub const CURVE_RADIUS: f32 = 2.0;
pub const WIRE_HEIGHT: f32 = 2.5;

pub struct Handler {
    binding: Binding,
    from: Option<XY<u32>>,
}

pub struct Parameters<'a> {
    pub event: &'a engine::events::Event,
    pub mouse_xy: &'a Option<XY<u32>>,
    pub terrain: &'a Grid<f32>,
    pub piste_map: &'a Grid<Option<usize>>,
    pub lifts: &'a mut HashMap<usize, Lift>,
    pub open: &'a mut HashSet<usize>,
    pub id_allocator: &'a mut id_allocator::Service,
    pub carousels: &'a mut HashMap<usize, Carousel>,
    pub cars: &'a mut HashMap<usize, Car>,
    pub exits: &'a mut HashMap<usize, Exit>,
    pub entrances: &'a mut HashMap<usize, Entrance>,
    pub reservations: &'a mut Grid<HashMap<usize, Reservation>>,
    pub graphics: &'a mut dyn engine::graphics::Graphics,
}

impl Handler {
    pub fn new(binding: Binding) -> Handler {
        Handler {
            binding,
            from: None,
        }
    }

    pub fn handle(
        &mut self,
        Parameters {
            event,
            mouse_xy,
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
            graphics,
        }: Parameters<'_>,
    ) -> HandlerResult {
        if !self.binding.binds_event(event) {
            return EventPersists;
        }

        let Some(mouse_xy) = mouse_xy else {
            return EventPersists;
        };
        let Ok(XYZ { x, y, .. }) = graphics.world_xyz_at(mouse_xy) else {
            return EventPersists;
        };
        let position = xy(x.round() as u32, y.round() as u32);
        if !terrain.in_bounds(position) {
            return EventPersists;
        }

        // handle case where from position is not set

        let Some(from) = self.from else {
            self.from = Some(position);
            return EventConsumed;
        };

        // create lift

        let to = position;

        let Some(from_piste) = piste_map[from] else {
            println!("INFO: No piste at from position");
            self.from = None;
            return EventPersists;
        };
        let Some(to_piste) = piste_map[to] else {
            self.from = None;
            println!("INFO: No piste at to position");
            return EventPersists;
        };

        let lift_id = id_allocator.next_id();
        let carousel_id = id_allocator.next_id();

        let points = get_points(terrain, &from, &to);
        let travel_direction = get_direction(&from, &to);

        let lift = Lift {
            segments: Segment::segments(&points),
            pick_up: lift::Portal {
                id: id_allocator.next_id(),
                segment: 0,
                state: State {
                    position: from,
                    travel_direction,
                    velocity: 0,
                },
            },
            drop_off: lift::Portal {
                id: id_allocator.next_id(),
                segment: 1,
                state: State {
                    position: to,
                    travel_direction,
                    velocity: encode_velocity(&LIFT_VELOCITY).unwrap_or(VELOCITY_LEVELS - 1),
                },
            },
            carousel_id,
        };

        // opening lift

        open.insert(lift_id);
        open.insert(lift.pick_up.id);
        open.insert(lift.drop_off.id);

        // setup carousel

        let new_cars =
            utils::carousel::create_cars(carousel_id, &lift.segments, &CAR_INTERVAL_METERS);

        let car_ids = (0..new_cars.len())
            .map(|_| id_allocator.next_id())
            .collect::<Vec<_>>();

        car_ids.iter().zip(new_cars).for_each(|(id, car)| {
            cars.insert(*id, car);
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
                origin_piste_id: from_piste,
                stationary_states: HashSet::from([lift.pick_up.state.stationary()]),
            },
        );

        // setup entrance

        entrances.insert(
            lift.drop_off.id,
            Entrance {
                destination_piste_id: to_piste,
                stationary_states: HashSet::from([lift.drop_off.state.stationary()]),
                altitude_meters: terrain
                    .offsets(to, &CORNERS)
                    .map(|corner| terrain[corner])
                    .sum::<f32>()
                    / terrain.offsets(to, &CORNERS).count() as f32,
            },
        );

        // reserve pick up position

        reservations[lift.pick_up.state.position].insert(lift.pick_up.id, Reservation::Structure);

        // clear from position

        self.from = None;

        // register lift

        lifts.insert(lift_id, lift);
        EventConsumed
    }
}

fn get_direction(from: &XY<u32>, to: &XY<u32>) -> Direction {
    let vector = xy(to.x as f32 - from.x as f32, to.y as f32 - from.y as f32);
    Direction::snap_to_direction(vector.angle())
}

fn get_points(terrain: &Grid<f32>, from: &XY<u32>, to: &XY<u32>) -> Vec<XYZ<f32>> {
    let from_3d = xyz(from.x as f32, from.y as f32, terrain[from] + WIRE_HEIGHT);
    let to_3d = xyz(to.x as f32, to.y as f32, terrain[to] + WIRE_HEIGHT);
    let from_2d = from_3d.xy();
    let to_2d = to_3d.xy();

    let angle = (to_2d - from_2d).angle();

    let top_curve = approximate_curve(
        &to_2d,
        angle,
        CURVE_INCREMENT,
        CURVE_RADIUS,
        CIRCLE_SEGMENTS / 2 + 1,
    );
    let top_curve_last = top_curve.last().unwrap();
    let bottom_curve_first = *top_curve_last - (to_2d - from_2d);
    let bottom_curve = approximate_curve(
        &bottom_curve_first,
        angle + PI,
        CURVE_INCREMENT,
        CURVE_RADIUS,
        CIRCLE_SEGMENTS / 2 + 1,
    );

    once(from_3d)
        .chain(once(to_3d))
        .chain(top_curve.into_iter().map(|XY { x, y }| xyz(x, y, to_3d.z)))
        .chain(
            once(bottom_curve_first)
                .chain(bottom_curve)
                .map(|XY { x, y }| xyz(x, y, from_3d.z)),
        )
        .collect()
}
