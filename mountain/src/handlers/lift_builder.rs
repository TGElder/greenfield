use std::collections::HashMap;
use std::f32::consts::PI;

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
pub const CURVE_INCREMENTS: u8 = 8;
pub const CURVE_RADIUS: f32 = 2.0;

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
        let from_3d = xyz(from.x as f32, from.y as f32, terrain[from]);
        let to_3d = xyz(to.x as f32, to.y as f32, terrain[to]);
        let lift_id = id_allocator.next_id();

        let vector = xy(to_3d.x - from_3d.x, to_3d.y - from_3d.y);
        let direction = Direction::snap_to_direction(vector.angle());

        let mut points = Vec::with_capacity(2 + CURVE_INCREMENTS as usize * 2);
        points.push(from_3d);
        points.push(to_3d);
        for point in curve(from_3d, to_3d, CURVE_INCREMENTS) {
            points.push(point);
        }
        points.push(from_3d);

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

fn curve(from: XYZ<f32>, to: XYZ<f32>, segments: u8) -> Vec<XYZ<f32>> {
    let right_angle_vector = xy(to.y - from.y, from.x - to.x).normalize();
    let curve_center = xy(to.x, to.y) + right_angle_vector * CURVE_RADIUS;
    let mut current_angle = curve_center.angle();
    let increment = PI / segments as f32;

    let mut out = Vec::with_capacity(segments as usize);

    for i in 0..segments {
        current_angle += increment;
        let vector = xy(current_angle.cos(), current_angle.sin()) * CURVE_RADIUS;
        let XY { x, y } = curve_center + vector;
        out.push(xyz(x, y, to.z));
    }

    out
}
