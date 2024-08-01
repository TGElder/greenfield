use crate::{handlers, Game};

#[derive(Clone, Copy)]
pub enum Mode {
    Piste,
    Path,
    Lift,
    Gates,
    Building,
    Door,
    None,
}

pub struct Handler {
    mode: Mode,
}

impl Handler {
    pub fn new() -> Handler {
        Handler { mode: Mode::None }
    }

    pub fn mode(&self) -> Mode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }
}

pub fn handle(
    mode: Mode,
    event: &engine::events::Event,
    game: &mut Game,
    graphics: &mut dyn engine::graphics::Graphics,
) {
    match mode {
        Mode::Piste => game
            .handlers
            .piste_builder
            .handle(handlers::piste_builder::Parameters {
                event,
                pistes: &mut game.components.pistes,
                piste_map: &mut game.components.piste_map,
                selection: &mut game.handlers.selection,
                terrain_artist: &mut game.systems.terrain_artist,
                tree_artist: &mut game.systems.tree_artist,
                id_allocator: &mut game.components.services.id_allocator,
            }),
        Mode::Path => game
            .handlers
            .path_builder
            .handle(handlers::piste_builder::Parameters {
                event,
                pistes: &mut game.components.pistes,
                piste_map: &mut game.components.piste_map,
                selection: &mut game.handlers.selection,
                terrain_artist: &mut game.systems.terrain_artist,
                tree_artist: &mut game.systems.tree_artist,
                id_allocator: &mut game.components.services.id_allocator,
            }),
        Mode::Lift => game
            .handlers
            .lift_builder
            .handle(handlers::lift_builder::Parameters {
                event,
                mouse_xy: &game.mouse_xy,
                terrain: &game.components.terrain,
                piste_map: &game.components.piste_map,
                lifts: &mut game.components.lifts,
                open: &mut game.components.open,
                id_allocator: &mut game.components.services.id_allocator,
                carousels: &mut game.components.carousels,
                cars: &mut game.components.cars,
                exits: &mut game.components.exits,
                entrances: &mut game.components.entrances,
                reservations: &mut game.components.reservations,
                graphics,
            }),
        Mode::Gates => game
            .handlers
            .gate_builder
            .handle(handlers::gate_builder::Parameters {
                event,
                piste_map: &game.components.piste_map,
                terrain: &game.components.terrain,
                selection: &mut game.handlers.selection,
                terrain_artist: &mut game.systems.terrain_artist,
                id_allocator: &mut game.components.services.id_allocator,
                gates: &mut game.components.gates,
                entrances: &mut game.components.entrances,
                exits: &mut game.components.exits,
                open: &mut game.components.open,
                reservations: &mut game.components.reservations,
            }),
        Mode::Building => {
            game.handlers
                .building_builder
                .handle(handlers::building_builder::Parameters {
                    event,
                    terrain: &game.components.terrain,
                    selection: &mut game.handlers.selection,
                    id_allocator: &mut game.components.services.id_allocator,
                    buildings: &mut game.components.buildings,
                    locations: &mut game.components.locations,
                    skiers: &mut game.components.skiers,
                    building_artist: &mut game.systems.building_artist,
                    tree_artist: &mut game.systems.tree_artist,
                    window_artist: &mut game.systems.window_artist,
                })
        }
        Mode::Door => game
            .handlers
            .door_builder
            .handle(handlers::door_builder::Parameters {
                event,
                pistes: &game.components.pistes,
                buildings: &game.components.buildings,
                terrain: &game.components.terrain,
                selection: &mut game.handlers.selection,
                id_allocator: &mut game.components.services.id_allocator,
                doors: &mut game.components.doors,
                entrances: &mut game.components.entrances,
            }),
        _ => (),
    }
}
