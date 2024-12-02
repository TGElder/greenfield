use commons::geometry::{xy, XYRectangle};

use crate::model::open;
use crate::utils::computer;
use crate::{Components, Services, Systems};

pub fn set_open_status(
    id: &usize,
    status: open::Status,
    Components {
        terrain,
        pistes,
        abilities,
        entrances,
        exits,
        open,
        reservations,
        costs,
        services: Services { clock, .. },
        ..
    }: &mut Components,
    Systems {
        global_computer,
        terrain_artist,
        ..
    }: &mut Systems,
) {
    let current_speed = clock.speed();
    clock.set_speed(0.0);

    open.insert(*id, status);

    if let open::Status::Open = status {
        computer::costs::compute_piste(id, pistes, terrain, exits, reservations, costs);
        computer::piste_ability::compute_piste(id, costs, entrances, exits, abilities);
    }

    global_computer.update();

    if let Some(piste) = pistes.get(id) {
        let grid = &piste.grid;
        terrain_artist.update_overlay(XYRectangle {
            from: *grid.origin(),
            to: *grid.origin() + xy(grid.width() - 2, grid.height() - 2),
        });
    }

    clock.set_speed(current_speed);
}
