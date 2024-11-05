use std::collections::{HashMap, HashSet};
use std::f32::consts::PI;
use std::iter::once;

use commons::curves::approximate_curve;
use commons::geometry::{xy, xyz, XY, XYZ};
use commons::grid::Grid;

use crate::controllers::Result::{self, Action, NoAction};
use crate::model::carousel::{Car, Carousel};
use crate::model::direction::Direction;
use crate::model::entrance::Entrance;
use crate::model::exit::Exit;
use crate::model::lift::{self, Lift, Segment};
use crate::model::reservation::Reservation;
use crate::model::skiing::State;
use crate::network::velocity_encoding::{encode_velocity, VELOCITY_LEVELS};
use crate::services::id_allocator;
use crate::systems::messenger;
use crate::utils;

pub const LIFT_VELOCITY: f32 = 2.0;
pub const CAR_INTERVAL_METERS: f32 = 10.0;
pub const CIRCLE_SEGMENTS: u8 = 16;
pub const CURVE_INCREMENT: f32 = (2.0 * PI) / CIRCLE_SEGMENTS as f32;
pub const CURVE_RADIUS: f32 = 2.0;
pub const WIRE_HEIGHT: f32 = 2.5;

pub struct Controller {
    from: Option<XY<u32>>,
}

pub struct Parameters<'a> {
    pub mouse_xy: &'a Option<XY<u32>>,
    pub terrain: &'a Grid<f32>,
    pub piste_map: &'a Grid<Option<usize>>,
    pub lifts: &'a mut HashMap<usize, Lift>,
    pub open: &'a mut HashMap<usize, bool>,
    pub id_allocator: &'a mut id_allocator::Service,
    pub carousels: &'a mut HashMap<usize, Carousel>,
    pub cars: &'a mut HashMap<usize, Car>,
    pub exits: &'a mut HashMap<usize, Exit>,
    pub entrances: &'a mut HashMap<usize, Entrance>,
    pub reservations: &'a mut Grid<HashMap<usize, Reservation>>,
    pub messenger: &'a mut messenger::System,
    pub graphics: &'a mut dyn engine::graphics::Graphics,
}

impl Controller {
    pub fn new() -> Controller {
        Controller { from: None }
    }

    pub fn trigger(
        &mut self,
        Parameters {
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
            messenger,
            graphics,
        }: Parameters<'_>,
    ) -> Result {
        let Some(mouse_xy) = mouse_xy else {
            return NoAction;
        };
        let Ok(XYZ { x, y, .. }) = graphics.world_xyz_at(mouse_xy) else {
            return NoAction;
        };
        let position = xy(x.round() as u32, y.round() as u32);
        if !terrain.in_bounds(position) {
            return NoAction;
        }

        // handle case where from position is not set

        let Some(from) = self.from else {
            self.from = Some(position);
            return Action;
        };

        // create lift

        let to = position;

        let Some(from_piste) = piste_map[from] else {
            messenger.send("Lift needs piste at start position!");
            self.from = None;
            return NoAction;
        };
        let Some(to_piste) = piste_map[to] else {
            messenger.send("Lift needs piste at end position!");
            self.from = None;
            return NoAction;
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

        open.insert(lift_id, true);
        open.insert(lift.pick_up.id, true);
        open.insert(lift.drop_off.id, true);

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
            },
        );

        // reserve pick up position

        reservations[lift.pick_up.state.position].insert(lift.pick_up.id, Reservation::Structure);

        // clear from position

        self.from = None;

        // register lift

        lifts.insert(lift_id, lift);
        Action
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
