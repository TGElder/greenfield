use std::collections::HashMap;

use commons::geometry::{xy, XYRectangle, XY, XYZ, xyz};
use commons::grid::Grid;
use engine::binding::Binding;
use nalgebra::Point3;

use crate::model::car::Car;
use crate::model::carousel::Carousel;
use crate::model::lift::{Lift, self};
use crate::services::id_allocator;
use crate::systems::overlay;

pub const LIFT_VELOCITY: f32 = 2.0;
pub const CAR_INTERVAL_METRES: f32 = 10.0;

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
        let from_3d = xyz(from.x as f32, from.y as f32, terrain[from]);
        let to_3d = xyz(to.x as f32, to.y as f32, terrain[to]);
        let length = nalgebra::distance(
            &Point3::new(from_3d.x, from_3d.y, from_3d.z),
            &Point3::new(to_3d.x, to_3d.y, to_3d.z)
        );
        let nodes = vec![
            lift::Node{ 
                from: from_3d, 
                to: to_3d, 
                distance_metres: length,
                from_action: Some(lift::Action::PickUp(from)),
            },
            lift::Node{ 
                    from: from_3d, 
                    to: to_3d, 
                    distance_metres: length,
                    from_action: Some(lift::Action::DropOff(to)),
            }
                    ];
        lifts.insert(lift_id, Lift { nodes  });
        self.from = None;

        // update overlay

        overlay.update(XYRectangle { from, to: from });
        overlay.update(XYRectangle { from: to, to });

        // setup carousel

        if self.bindings.carousel.binds_event(event) {
            carousels.insert(
                lift_id,
                Carousel {
                    velocity: LIFT_VELOCITY,
                },
            );

            let mut from_position = 0.0;
            let mut car_position = 0.0;
            for node in nodes {

                while car_position < from_position + node.distance_metres {
                    cars.insert(
                        id_allocator.next_id(),
                        Car { lift: lift_id, position: car_position });
                    car_position += CAR_INTERVAL_METRES;
                }

                from_position = from_position + node.distance_metres;
            }
        }
    }
}
