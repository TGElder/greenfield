use std::collections::HashMap;

use commons::geometry::{xy, XYRectangle, XY, XYZ};
use commons::grid::Grid;
use engine::binding::Binding;
use nalgebra::Point3;

use crate::model::car::Car;
use crate::model::carousel::Carousel;
use crate::model::lift::{self, Lift};
use crate::services::id_allocator;
use crate::systems::overlay;

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

        let Some(from) = self.from else {
            self.from = Some(position);
            return;
        };

        let to = position;
        let lift_id = id_allocator.next_id();
        lifts.insert(
            lift_id,
            Lift {
                pick_up: lift::Portal {
                    segment: 0,
                    position: from,
                },
                drop_off: lift::Portal {
                    segment: 0,
                    position: to,
                },
                segments: vec![],
            },
        );
        self.from = None;

        // update overlay

        overlay.update(XYRectangle { from, to: from });
        overlay.update(XYRectangle { from: to, to });

        // setup carousel

        if self.bindings.carousel.binds_event(event) {
            let length = nalgebra::distance(
                &Point3::new(from.x as f32, from.y as f32, terrain[from]),
                &Point3::new(to.x as f32, to.y as f32, terrain[to]),
            );

            let mut position = 0.0;
            let mut car_vec = vec![];
            while position < length * 2.0 {
                position += CAR_INTERVAL_METERS;
                let car_id = id_allocator.next_id();
                cars.insert(
                    car_id,
                    Car {
                        lift_id,
                        segment: 0,
                        meters_from_start_of_segment: position,
                    },
                );
                car_vec.push(car_id);
            }

            carousels.insert(
                lift_id,
                Carousel {
                    velocity: LIFT_VELOCITY,
                    cars: car_vec,
                },
            );
        }
    }
}
