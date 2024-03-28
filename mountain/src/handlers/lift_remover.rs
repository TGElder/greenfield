use commons::geometry::{xy, XY, XYZ};
use engine::binding::Binding;
use engine::graphics::Graphics;

use crate::model::ability::ABILITIES;
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

        let lift_ids = components
            .lifts
            .iter()
            .filter(|(_, lift)| {
                lift.pick_up.position == position || lift.drop_off.position == position
            })
            .map(|(lift_id, _)| *lift_id)
            .collect::<Vec<_>>();

        for lift_id in lift_ids {
            remove_lift(graphics, components, &lift_id);
        }
    }
}

pub fn remove_lift(graphics: &mut dyn Graphics, components: &mut Components, lift_id: &usize) {
    // Fetch entities

    let carousel_ids = components
        .carousels
        .iter()
        .filter(|(_, carousel)| carousel.lift_id == *lift_id)
        .map(|(carousel_id, _)| *carousel_id)
        .collect::<Vec<_>>();

    let car_ids = components
        .carousels
        .iter()
        .filter(|(_, carousel)| carousel.lift_id == *lift_id)
        .flat_map(|(_, carousel)| carousel.car_ids.iter().copied())
        .collect::<Vec<_>>();

    // Validate

    if components.open.contains(lift_id) {
        println!("Close lift {} before removing it!", lift_id);
        return;
    }

    if components
        .locations
        .values()
        .any(|location_id| car_ids.contains(location_id))
    {
        println!("Cannot remove lift {} while people are riding it!", lift_id);
        return;
    }

    if components
        .targets
        .values()
        .any(|target_id| *target_id == *lift_id)
    {
        println!(
            "Cannot remove lift {} while people are targeting it!",
            lift_id
        );
        return;
    }

    // Remove

    let lift = components.lifts.remove(lift_id);

    if let Some(lift) = lift {
        components.reservations[lift.pick_up.position].remove(lift_id);
    }

    for carousel_id in carousel_ids {
        remove_carousel(graphics, components, &carousel_id);
    }
    remove_drawing(graphics, components, lift_id);

    for (_, exits) in components.exits.iter_mut() {
        exits.retain(|exit| exit.id != *lift_id);
    }

    for (_, costs) in components.costs.iter_mut() {
        for ability in ABILITIES {
            costs.remove_costs(*lift_id, ability);
        }
    }
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

fn remove_drawing(graphics: &mut dyn Graphics, components: &mut Components, drawing_id: &usize) {
    if let Some(drawing_id) = components.drawings.get(drawing_id) {
        let _ = graphics.draw_triangles(drawing_id, &[]);
    }
}

fn remove_dynamic_drawing(
    graphics: &mut dyn Graphics,
    components: &mut Components,
    drawing_id: &usize,
) {
    if let Some(drawing_id) = components.drawings.get(drawing_id) {
        let _ = graphics.update_dynamic_triangles(drawing_id, None);
    }
}
