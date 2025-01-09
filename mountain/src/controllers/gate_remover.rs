use commons::geometry::{xy, XY, XYZ};
use commons::map::ContainsKeyValue;
use engine::graphics::{DrawMode, Graphics};

use crate::controllers::Result::{self, Action, NoAction};
use crate::model::ability::ABILITIES;
use crate::model::entrance::Entrance;
use crate::model::exit::Exit;
use crate::model::gate::Gate;
use crate::model::open;
use crate::systems::{messenger, piste_computer};
use crate::Components;

pub fn trigger(
    mouse_xy: &Option<XY<u32>>,
    components: &mut Components,
    piste_computer: &mut piste_computer::System,
    messenger: &mut messenger::System,
    graphics: &mut dyn engine::graphics::Graphics,
) -> Result {
    let Some(mouse_xy) = mouse_xy else {
        return NoAction;
    };
    let Ok(XYZ { x, y, .. }) = graphics.world_xyz_at(mouse_xy) else {
        return NoAction;
    };
    let position = xy(x.round() as u32, y.round() as u32);

    let gate_ids = components
        .gates
        .iter()
        .filter(|(_, Gate { footprint, .. })| footprint.contains(position))
        .map(|(gate_id, _)| *gate_id)
        .collect::<Vec<_>>();

    if gate_ids.is_empty() {
        return NoAction;
    }

    for gate_id in gate_ids {
        remove_gate(components, &gate_id, piste_computer, messenger, graphics);
    }

    Action
}

pub fn remove_gate(
    components: &mut Components,
    gate_id: &usize,
    piste_computer: &mut piste_computer::System,
    messenger: &mut messenger::System,
    graphics: &mut dyn Graphics,
) {
    // Validate

    if !components
        .open
        .contains_key_value(gate_id, open::Status::Closed)
    {
        messenger.send(format!(
            "Gate {} must be closed before it can be removed",
            gate_id
        ));
        return;
    }

    // Remove

    let gate = components.gates.remove(gate_id);
    components.open.remove(gate_id);
    if let Some(Entrance {
        destination_piste_id,
        ..
    }) = components.entrances.remove(gate_id)
    {
        piste_computer.compute(destination_piste_id);
    }
    if let Some(Exit {
        origin_piste_id, ..
    }) = components.exits.remove(gate_id)
    {
        piste_computer.compute(origin_piste_id);
    }

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
        let _ = graphics.draw_triangles(drawing_id, DrawMode::Invisible, &[]);
    }
    components.drawings.remove(id);
}
