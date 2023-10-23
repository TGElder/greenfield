use std::collections::HashMap;

use commons::geometry::{xy, xyz, XYRectangle, XY, XYZ};
use commons::grid::Grid;
use engine::binding::Binding;

use crate::model::carousel::{Car, Carousel};
use crate::model::lift::{self, Lift, Segment};
use crate::services::id_allocator;
use crate::systems::overlay;
use crate::utils;

pub const LIFT_VELOCITY: f32 = 2.0;
pub const CAR_INTERVAL_METERS: f32 = 10.0;

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
        let lift = Lift {
            segments: Segment::segments(&[from_3d, to_3d, from_3d]),
            pick_up: lift::Portal {
                segment: 0,
                position: from,
            },
            drop_off: lift::Portal {
                segment: 1,
                position: to,
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
