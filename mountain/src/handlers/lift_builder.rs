use std::collections::HashMap;
use std::f32::consts::PI;
use std::iter::once;

use commons::curves::approximate_curve;
use commons::geometry::{xy, xyz, XYRectangle, XY, XYZ};
use commons::grid::Grid;
use engine::binding::Binding;

use crate::model::carousel::{Car, Carousel};
use crate::model::direction::Direction;
use crate::model::lift::{self, Lift, Segment};
use crate::services::id_allocator;
use crate::systems::overlay;
use crate::utils;

pub const LIFT_VELOCITY: f32 = 2.0;
pub const CAR_INTERVAL_METERS: f32 = 10.0;
pub const CIRCLE_SEGMENTS: u8 = 16;
pub const CURVE_INCREMENT: f32 = (2.0 * PI) / CIRCLE_SEGMENTS as f32;
pub const CURVE_RADIUS: f32 = 2.0;
pub const WIRE_HEIGHT: f32 = 3.0;

pub struct Handler {
    pub bindings: Bindings,
    from: Option<XY<u32>>,
}

pub struct Bindings {
    pub teleporter: Binding,
    pub carousel: Binding,
}

pub struct Parameters<'a> {
    pub event: &'a engine::events::Event,
    pub mouse_xy: &'a Option<XY<u32>>,
    pub terrain: &'a Grid<f32>,
    pub lifts: &'a mut HashMap<usize, Lift>,
    pub overlay: &'a mut overlay::System,
    pub id_allocator: &'a mut id_allocator::Service,
    pub carousels: &'a mut HashMap<usize, Carousel>,
    pub cars: &'a mut HashMap<usize, Car>,
    pub graphics: &'a mut dyn engine::graphics::Graphics,
}

impl Handler {
    pub fn new(bindings: Bindings) -> Handler {
        Handler {
            bindings,
            from: None,
        }
    }

    pub fn handle(
        &mut self,
        Parameters {
            event,
            mouse_xy,
            terrain,
            lifts,
            overlay,
            id_allocator,
            carousels,
            cars,
            graphics,
        }: Parameters<'_>,
    ) {
        if !(self.bindings.carousel.binds_event(event)
            || self.bindings.teleporter.binds_event(event))
        {
            return;
        }

        let Some(mouse_xy) = mouse_xy else { return };
        let Ok(XYZ { x, y, .. }) = graphics.world_xyz_at(mouse_xy) else {
            return;
        };
        let position = xy(x.round() as u32, y.round() as u32);

        // handle case where from position is not set

        let Some(from) = self.from else {
            self.from = Some(position);
            return;
        };

        // create lift

        let to = position;
        let lift_id = id_allocator.next_id();

        let points = get_points(terrain, &from, &to);
        let direction = get_direction(&from, &to);

        let lift = Lift {
            segments: Segment::segments(&points),
            pick_up: lift::Portal {
                segment: 0,
                position: from,
                direction,
            },
            drop_off: lift::Portal {
                segment: 1,
                position: to,
                direction,
            },
        };

        // setup carousel

        if self.bindings.carousel.binds_event(event) {
            let new_cars = utils::carousel::create_cars(&lift.segments, &CAR_INTERVAL_METERS);

            let car_ids = (0..new_cars.len())
                .map(|_| id_allocator.next_id())
                .collect::<Vec<_>>();

            car_ids.iter().zip(new_cars).for_each(|(id, car)| {
                cars.insert(*id, car);
            });

            carousels.insert(
                id_allocator.next_id(),
                Carousel {
                    lift_id,
                    velocity: LIFT_VELOCITY,
                    car_ids,
                },
            );
        }

        // register lift

        lifts.insert(lift_id, lift);

        // update overlay

        overlay.update(XYRectangle { from, to: from });
        overlay.update(XYRectangle { from: to, to });

        // clear from position

        self.from = None;
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
    let buttom_curve_first = *top_curve_last - (to_2d - from_2d);
    let bottom_curve = approximate_curve(
        &buttom_curve_first,
        angle + PI,
        CURVE_INCREMENT,
        CURVE_RADIUS,
        CIRCLE_SEGMENTS / 2 + 1,
    );

    once(from_3d)
        .chain(top_curve.into_iter().map(|XY { x, y }| xyz(x, y, to_3d.z)))
        .chain(
            bottom_curve
                .into_iter()
                .map(|XY { x, y }| xyz(x, y, from_3d.z)),
        )
        .collect()
}
