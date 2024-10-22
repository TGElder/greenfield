use commons::geometry::{xy, XY, XYZ};
use engine::graphics::Graphics;

use crate::controllers::Result::{self, Action, NoAction};
use crate::model::building::Building;
use crate::{Components, Systems};

pub fn trigger(
    mouse_xy: &Option<XY<u32>>,
    graphics: &mut dyn engine::graphics::Graphics,
    components: &mut Components,
    systems: &mut Systems,
) -> Result {
    let Some(mouse_xy) = mouse_xy else {
        return NoAction;
    };
    let Ok(XYZ { x, y, .. }) = graphics.world_xyz_at(mouse_xy) else {
        return NoAction;
    };
    let position = xy(x.round() as u32, y.round() as u32);

    let building_ids = components
        .buildings
        .iter()
        .filter(|(_, Building { footprint, .. })| footprint.contains(position))
        .map(|(building_id, _)| *building_id)
        .collect::<Vec<_>>();

    if building_ids.is_empty() {
        return NoAction;
    }

    for building_id in building_ids {
        remove_building(graphics, components, systems, &building_id);
    }

    Action
}

pub fn remove_building(
    graphics: &mut dyn Graphics,
    components: &mut Components,
    systems: &mut Systems,
    building_id: &usize,
) {
    // removing skiers

    let skiers_to_remove = components
        .skiers
        .iter()
        .filter(|(_, skier)| skier.hotel_id == *building_id)
        .map(|(skier_id, _)| *skier_id)
        .collect::<Vec<_>>();

    for skier_id in skiers_to_remove.iter() {
        components.skiers.remove(skier_id);
        components.plans.remove(skier_id);
        components.locations.remove(skier_id);
        components.targets.remove(skier_id);
        components.global_targets.remove(skier_id);
        components.frames.remove(skier_id);
        components.clothes.remove(skier_id);
        remove_dynamic_drawing(graphics, components, skier_id);
    }

    components
        .planning_queue
        .retain(|skier_id| !skiers_to_remove.contains(skier_id));

    for position in components.reservations.iter() {
        let reservations = &mut components.reservations[position];
        reservations.retain(|id, _| !skiers_to_remove.contains(id));
    }

    // removing doors

    let doors_to_remove = components
        .doors
        .iter()
        .filter(|(_, door)| door.building_id == *building_id)
        .map(|(door_id, _)| *door_id)
        .collect::<Vec<_>>();

    for door_id in doors_to_remove.iter() {
        components.doors.remove(door_id);
        components.entrances.remove(door_id);
        components.exits.remove(door_id);
        components.open.remove(door_id);
        remove_drawing(graphics, components, door_id);
    }

    // removing building

    components.buildings.remove(building_id);
    remove_drawing(graphics, components, building_id);

    // updating art

    systems.tree_artist.update();
}

fn remove_drawing(graphics: &mut dyn Graphics, components: &mut Components, id: &usize) {
    if let Some(drawing_id) = components.drawings.get(id) {
        let _ = graphics.draw_triangles(drawing_id, &[]);
    }
    components.drawings.remove(id);
}

fn remove_dynamic_drawing(graphics: &mut dyn Graphics, components: &mut Components, id: &usize) {
    if let Some(drawing_id) = components.drawings.get(id) {
        let _ = graphics.update_dynamic_triangles(drawing_id, None);
    }
    components.drawings.remove(id);
}
