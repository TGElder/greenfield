use std::collections::HashMap;

use commons::geometry::{xy, XY, XYZ};
use commons::grid::Grid;
use engine::binding::Binding;
use engine::graphics::Graphics;

use crate::model::reservation::Reservation;
use crate::model::skiing::Plan;

pub struct Handler {
    pub binding: Binding,
    pub ids: Vec<usize>,
}

pub struct Parameters<'a> {
    pub mouse_xy: &'a Option<XY<u32>>,
    pub reservations: &'a Grid<HashMap<usize, Reservation>>,
    pub graphics: &'a mut dyn Graphics,
}

impl Handler {
    pub fn handle(
        &mut self,
        event: &engine::events::Event,
        Parameters {
            mouse_xy,
            reservations,
            graphics,
        }: Parameters<'_>,
    ) {
        if !self.binding.binds_event(event) {
            return;
        }

        let Some(mouse_xy) = mouse_xy else { return };
        let Ok(XYZ { x, y, .. }) = graphics.world_xyz_at(mouse_xy) else {
            return;
        };
        let mouse_position = xy(x.round() as u32, y.round() as u32);

        for (id, _) in reservations[mouse_position].iter() {
            self.ids.push(*id);
        }
    }

    pub fn update_gui(
        &mut self,
        plans: &HashMap<usize, Plan>,
        locations: &HashMap<usize, usize>,
        targets: &HashMap<usize, usize>,
        global_targets: &HashMap<usize, usize>,
        ctx: &engine::egui::Context,
    ) {
        self.ids.retain(|id| {
            let mut open = true;
            engine::egui::Window::new(format!("Skier {}", id))
                .open(&mut open)
                .show(ctx, |ui| {
                    ui.label(format!("Location = {:?}", locations.get(id)));
                    ui.label(format!("Target = {:?}", targets.get(id)));
                    ui.label(format!("Global target = {:?}", global_targets.get(id)));
                    ui.label(format!("Plan = {:?}", plans.get(id)));
                });
            open
        });
    }
}
