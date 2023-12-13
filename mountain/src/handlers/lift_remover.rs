use std::collections::{HashMap, HashSet};

use commons::geometry::{xy, XY, XYZ};
use engine::binding::Binding;

use crate::model::carousel::{Car, Carousel};
use crate::model::exit::Exit;
use crate::model::frame::Frame;
use crate::model::lift::Lift;
use crate::model::piste::PisteCosts;

pub struct Handler {
    pub binding: Binding,
}

impl Handler {
    pub fn handle(
        &self,
        event: &engine::events::Event,
        mouse_xy: &Option<XY<u32>>,
        open: &HashSet<usize>,
        locations: &HashMap<usize, usize>,
        targets: &HashMap<usize, usize>,
        lifts: &mut HashMap<usize, Lift>,
        carousels: &mut HashMap<usize, Carousel>,
        cars: &mut HashMap<usize, Car>,
        distance_costs: &mut HashMap<usize, PisteCosts>,
        skiing_costs: &mut HashMap<usize, PisteCosts>,
        frames: &mut HashMap<usize, Option<Frame>>,
        drawings: &mut HashMap<usize, usize>,
        exits: &mut HashMap<usize, Vec<Exit>>,
        graphics: &mut dyn engine::graphics::Graphics,
    ) {
        if !self.binding.binds_event(event) {
            return;
        }

        let Some(mouse_xy) = mouse_xy else { return };
        let Ok(XYZ { x, y, .. }) = graphics.world_xyz_at(mouse_xy) else {
            return;
        };
        let position = xy(x.round() as u32, y.round() as u32);

        let to_remove = lifts
            .iter()
            .filter(|(_, lift)| {
                lift.pick_up.position == position || lift.drop_off.position == position
            })
            .map(|(lift_id, _)| *lift_id)
            .collect::<Vec<_>>();

        for lift_id in to_remove {
            let carousel_ids = carousels
                .iter()
                .filter(|(_, carousel)| carousel.lift_id == lift_id)
                .map(|(carousel_id, _)| *carousel_id)
                .collect::<Vec<_>>();

            let car_ids = carousels
                .iter()
                .filter(|(_, carousel)| carousel.lift_id == lift_id)
                .flat_map(|(_, carousel)| carousel.car_ids.iter().copied())
                .collect::<Vec<_>>();

            if open.contains(&lift_id) {
                println!("Close lift {} before removing it!", lift_id);
                continue;
            }

            if locations
                .values()
                .any(|location_id| car_ids.contains(location_id))
            {
                println!("Cannot remove lift {} while people are riding it!", lift_id);
                continue;
            }

            if targets.values().any(|target_id| *target_id == lift_id) {
                println!(
                    "Cannot remove lift {} while people are targeting it!",
                    lift_id
                );
                continue;
            }

            lifts.remove(&lift_id);

            if let Some(drawing_id) = drawings.get(&lift_id) {
                let _ = graphics.draw_quads(drawing_id, &[]);
            }

            for carousel_id in carousel_ids {
                carousels.remove(&carousel_id);
            }

            for car_id in car_ids {
                cars.remove(&car_id);
                frames.remove(&car_id);
                if let Some(drawing_id) = drawings.get(&car_id) {
                    let _ = graphics.draw_quads(drawing_id, &[]);
                }
            }

            for (_, exits) in exits.iter_mut() {
                exits.retain(|exit| exit.id != lift_id);
            }

            for (_, costs) in distance_costs.iter_mut() {
                costs.remove_costs(lift_id);
            }

            for (_, costs) in skiing_costs.iter_mut() {
                costs.remove_costs(lift_id);
            }
        }
    }
}
