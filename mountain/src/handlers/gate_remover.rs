use commons::geometry::{xy, XY, XYZ};
use engine::binding::Binding;
use engine::graphics::Graphics;

use crate::handlers::HandlerResult::{self, EventConsumed, EventPersists};
use crate::model::ability::ABILITIES;
use crate::model::gate::Gate;
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

        let gate_ids = components
            .gates
            .iter()
            .filter(|(_, Gate { footprint, .. })| footprint.contains(position))
            .map(|(gate_id, _)| *gate_id)
            .collect::<Vec<_>>();

        if gate_ids.is_empty() {
            return EventPersists;
        }

        for gate_id in gate_ids {
            remove_gate(graphics, components, &gate_id);
        }

        EventConsumed
    }
}

pub fn remove_gate(graphics: &mut dyn Graphics, components: &mut Components, gate_id: &usize) {
    // Validate

    if components.open.contains(gate_id) {
        println!("Close gate {} before removing it!", gate_id);
        return;
    }

    if components
        .targets
        .values()
        .any(|target_id| *target_id == *gate_id)
    {
        println!(
            "Cannot remove gate {} while people are targeting it!",
            gate_id
        );
        return;
    }

    // Remove

    let gate = components.gates.remove(gate_id);
    components.open.remove(gate_id);
    components.entrances.remove(gate_id);
    components.exits.remove(gate_id);

    if let Some(gate) = gate {
        gate.footprint.iter().for_each(|position| {
            components.reservations[position].remove(gate_id);
        });
    }

    remove_drawing(graphics, components, gate_id);
    for (_, costs) in components.costs.iter_mut() {
        for ability in ABILITIES {
            costs.remove_costs(*gate_id, ability);
        }
    }
}

fn remove_drawing(graphics: &mut dyn Graphics, components: &mut Components, id: &usize) {
    if let Some(drawing_id) = components.drawings.get(id) {
        let _ = graphics.draw_triangles(drawing_id, &[]);
    }
    components.drawings.remove(id);
}
