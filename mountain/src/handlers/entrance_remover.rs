use commons::geometry::{xy, XY, XYZ};
use engine::binding::Binding;
use engine::graphics::Graphics;

use crate::model::entrance::Entrance;
use crate::Components;

pub struct Handler {
    pub binding: Binding,
}

impl Handler {
    pub fn handle(
        &self,
        event: &engine::events::Event,
        mouse_xy: &Option<XY<u32>>,
        graphics: &mut dyn engine::graphics::Graphics,
        components: &mut Components,
    ) {
        if !self.binding.binds_event(event) {
            return;
        }

        let Some(mouse_xy) = mouse_xy else { return };
        let Ok(XYZ { x, y, .. }) = graphics.world_xyz_at(mouse_xy) else {
            return;
        };
        let position = xy(x.round() as u32, y.round() as u32);

        let entrance_ids = components
            .entrances
            .iter()
            .filter(|(_, Entrance { footprint, .. })| footprint.contains(position))
            .map(|(entrance_id, _)| *entrance_id)
            .collect::<Vec<_>>();

        for entrance_id in entrance_ids {
            remove_entrance(graphics, components, &entrance_id);
        }
    }
}

pub fn remove_entrance(
    graphics: &mut dyn Graphics,
    components: &mut Components,
    entrance_id: &usize,
) {
    // Validate

    if components.open.contains(entrance_id) {
        println!("Close entrance {} before removing it!", entrance_id);
        return;
    }

    if components
        .targets
        .values()
        .any(|target_id| *target_id == *entrance_id)
    {
        println!(
            "Cannot remove entrance {} while people are targeting it!",
            entrance_id
        );
        return;
    }

    // Remove

    let entrance = components.entrances.remove(entrance_id);

    if let Some(entrance) = entrance {
        entrance.footprint.iter().for_each(|position| {
            components.reservations[position].remove(entrance_id);
        });
    }

    remove_drawing(graphics, components, entrance_id);

    for (_, exits) in components.exits.iter_mut() {
        exits.retain(|exit| exit.id != *entrance_id);
    }

    for (_, costs) in components.costs.iter_mut() {
        costs.remove_costs(entrance_id);
    }
}

fn remove_drawing(graphics: &mut dyn Graphics, components: &mut Components, drawing_id: &usize) {
    if let Some(drawing_id) = components.drawings.get(drawing_id) {
        let _ = graphics.draw_quads(drawing_id, &[]);
    }
}
