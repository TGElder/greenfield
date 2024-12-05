use commons::geometry::{xy, XYRectangle};

use crate::model::open;
use crate::{Components, Systems};

pub fn set_open_status(
    id: &usize,
    status: open::Status,
    components: &mut Components,
    systems: &mut Systems,
) {
    let clock = &mut components.services.clock;
    let current_speed = clock.speed();
    clock.set_speed(0.0);

    set_open_status_internal(id, status, components, systems);

    let clock = &mut components.services.clock;
    clock.set_speed(current_speed);
}

fn set_open_status_internal(
    id: &usize,
    status: open::Status,
    components: &mut Components,
    systems: &mut Systems,
) {
    let Components {
        pistes,
        open,
        children,
        ..
    } = components;

    if !open.contains_key(id) {
        // This is not something with an open status
        return;
    }

    open.insert(*id, status);

    if let open::Status::Open = status {
        systems.piste_computer.compute(*id);
    }

    if let Some(piste) = pistes.get(id) {
        let grid = &piste.grid;
        systems.terrain_artist.update_overlay(XYRectangle {
            from: *grid.origin(),
            to: *grid.origin() + xy(grid.width() - 2, grid.height() - 2),
        });
    }

    if let Some(children) = children.get(id).cloned() {
        for child_id in children {
            set_open_status_internal(&child_id, status, components, systems);
        }
    }
}
