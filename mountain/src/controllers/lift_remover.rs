use commons::geometry::{xy, XY, XYZ};
use commons::map::ContainsKeyValue;
use engine::graphics::{DrawMode, Graphics};

use crate::controllers::Result::{self, Action, NoAction};
use crate::model::entrance::Entrance;
use crate::model::exit::Exit;
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

    let lift_ids = components
        .lifts
        .iter()
        .filter(|(_, lift)| {
            lift.pick_up.state.position == position || lift.drop_off.state.position == position
        })
        .map(|(lift_id, _)| *lift_id)
        .collect::<Vec<_>>();

    if lift_ids.is_empty() {
        return NoAction;
    }

    for lift_id in lift_ids {
        remove_lift(components, &lift_id, piste_computer, messenger, graphics);
    }

    Action
}

pub fn remove_lift(
    components: &mut Components,
    lift_id: &usize,
    piste_computer: &mut piste_computer::System,
    messenger: &mut messenger::System,
    graphics: &mut dyn Graphics,
) {
    // Validate

    if !components
        .open
        .contains_key_value(lift_id, open::Status::Closed)
    {
        messenger.send(format!(
            "Lift {} must be closed before it can be removed",
            lift_id
        ));
        return;
    }

    // Remove

    let lift = components.lifts.remove(lift_id);
    components.open.remove(lift_id);

    if let Some(lift) = lift {
        if let Some(Exit {
            origin_piste_id, ..
        }) = components.exits.remove(&lift.pick_up.id)
        {
            piste_computer.compute(origin_piste_id);
        }
        components.open.remove(&lift.pick_up.id);
        components.reservations[lift.pick_up.state.position].remove(&lift.pick_up.id);

        if let Some(Entrance {
            destination_piste_id,
            ..
        }) = components.entrances.remove(&lift.drop_off.id)
        {
            piste_computer.compute(destination_piste_id);
        }
        components.open.remove(&lift.drop_off.id);

        remove_carousel(graphics, components, &lift.carousel_id);

        remove_lift_buildings(graphics, components, &lift.buildings_id);
    }

    remove_drawing(graphics, components, lift_id);
}

fn remove_carousel(graphics: &mut dyn Graphics, components: &mut Components, carousel_id: &usize) {
    let car_ids = components
        .carousels
        .get(carousel_id)
        .iter()
        .flat_map(|carousel| carousel.car_ids.iter().copied())
        .collect::<Vec<_>>();

    components.carousels.remove(carousel_id);
    for car_id in car_ids {
        remove_car(graphics, components, &car_id);
    }
}

fn remove_car(graphics: &mut dyn Graphics, components: &mut Components, car_id: &usize) {
    components.cars.remove(car_id);
    components.frames.remove(car_id);
    remove_dynamic_drawing(graphics, components, car_id);
}

fn remove_lift_buildings(graphics: &mut dyn Graphics, components: &mut Components, id: &usize) {
    components.lift_buildings.remove(id);
    remove_drawing(graphics, components, id);
}

fn remove_drawing(graphics: &mut dyn Graphics, components: &mut Components, id: &usize) {
    if let Some(drawing_id) = components.drawings.get(id) {
        let _ = graphics.draw_triangles(drawing_id, DrawMode::Invisible, &[]);
    }
    components.drawings.remove(id);
}

fn remove_dynamic_drawing(graphics: &mut dyn Graphics, components: &mut Components, id: &usize) {
    if let Some(drawing_id) = components.drawings.get(id) {
        let _ = graphics.update_dynamic_triangles(drawing_id, DrawMode::Invisible, &[]);
    }
    components.drawings.remove(id);
}
