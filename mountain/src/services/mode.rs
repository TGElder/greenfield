use crate::controllers::Result::{self, Action, NoAction};
use crate::systems::messenger;
use crate::{controllers, Game};

#[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
pub enum Mode {
    #[default]
    None,
    Open,
    Query,
    Piste,
    PisteEraser,
    Path,
    Lift,
    Gate,
    Building,
    Door,
    Demolish,
}

impl Mode {
    fn has_selection(&self) -> bool {
        matches!(
            self,
            Mode::Piste | Mode::PisteEraser | Mode::Path | Mode::Gate | Mode::Building | Mode::Door
        )
    }
}

#[derive(Default)]
pub struct Service {
    mode: Mode,
}

impl Service {
    pub fn mode(&self) -> Mode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    pub fn get_handler(
        &self,
    ) -> impl FnOnce(&engine::events::Event, &mut Game, &mut dyn engine::graphics::Graphics) {
        let mode = self.mode;
        move |event, game, graphics| handle(mode, event, game, graphics)
    }
}

fn handle(
    mode: Mode,
    event: &engine::events::Event,
    game: &mut Game,
    graphics: &mut dyn engine::graphics::Graphics,
) {
    let handler_result = try_to_handle(mode, event, game, graphics);

    if handler_result == Action {
        return;
    }

    if mode.has_selection() {
        game.handlers.selection.handle(
            &game.bindings.selection,
            event,
            &game.mouse_xy,
            &game.components.terrain,
            graphics,
            &mut game.systems.terrain_artist,
        );
    }
}

fn try_to_handle(
    mode: Mode,
    event: &engine::events::Event,
    game: &mut Game,
    graphics: &mut dyn engine::graphics::Graphics,
) -> controllers::Result {
    if !game.bindings.action.binds_event(event) {
        return NoAction;
    }

    match mode {
        Mode::Open => try_to_open(game, graphics),
        Mode::Query => {
            controllers::skier_debugger::trigger(controllers::skier_debugger::Parameters {
                mouse_xy: &game.mouse_xy,
                reservations: &game.components.reservations,
                plans: &game.components.plans,
                locations: &game.components.locations,
                targets: &game.components.targets,
                global_targets: &game.components.global_targets,
                messenger: &mut game.systems.messenger,
                graphics,
            })
        }
        Mode::Piste => game
            .controllers
            .piste_builder
            .trigger(controllers::piste_builder::Parameters {
                pistes: &mut game.components.pistes,
                piste_map: &mut game.components.piste_map,
                selection: &mut game.handlers.selection,
                terrain_artist: &mut game.systems.terrain_artist,
                tree_artist: &mut game.systems.tree_artist,
                id_allocator: &mut game.components.services.id_allocator,
            })
            .then_try(|| {
                game.controllers
                    .piste_eraser
                    .trigger(controllers::piste_eraser::Parameters {
                        pistes: &mut game.components.pistes,
                        piste_map: &mut game.components.piste_map,
                        selection: &mut game.handlers.selection,
                        terrain_artist: &mut game.systems.terrain_artist,
                        tree_artist: &mut game.systems.tree_artist,
                    })
            }),
        Mode::Path => game
            .controllers
            .path_builder
            .trigger(controllers::piste_builder::Parameters {
                pistes: &mut game.components.pistes,
                piste_map: &mut game.components.piste_map,
                selection: &mut game.handlers.selection,
                terrain_artist: &mut game.systems.terrain_artist,
                tree_artist: &mut game.systems.tree_artist,
                id_allocator: &mut game.components.services.id_allocator,
            })
            .then_try(|| {
                game.controllers
                    .piste_eraser
                    .trigger(controllers::piste_eraser::Parameters {
                        pistes: &mut game.components.pistes,
                        piste_map: &mut game.components.piste_map,
                        selection: &mut game.handlers.selection,
                        terrain_artist: &mut game.systems.terrain_artist,
                        tree_artist: &mut game.systems.tree_artist,
                    })
            }),
        Mode::Lift => {
            game.controllers
                .lift_builder
                .trigger(controllers::lift_builder::Parameters {
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
                    messenger: &mut game.systems.messenger,
                    graphics,
                })
        }
        Mode::Building => game.controllers.building_builder.select(
            controllers::building_builder::SelectParameters {
                selection: &mut game.handlers.selection,
                id_allocator: &mut game.components.services.id_allocator,
                buildings: &mut game.components.buildings,
                tree_artist: &mut game.systems.tree_artist,
            },
        ),
        Mode::Gate => controllers::gate_builder::trigger(controllers::gate_builder::Parameters {
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
            messenger: &mut game.systems.messenger,
        }),
        Mode::Door => controllers::door_builder::trigger(controllers::door_builder::Parameters {
            pistes: &game.components.pistes,
            buildings: &game.components.buildings,
            terrain: &game.components.terrain,
            selection: &mut game.handlers.selection,
            id_allocator: &mut game.components.services.id_allocator,
            doors: &mut game.components.doors,
            entrances: &mut game.components.entrances,
            messenger: &mut game.systems.messenger,
        }),
        Mode::Demolish => try_to_demolish(game, graphics),
        _ => NoAction,
    }
}

fn try_to_open(game: &mut Game, graphics: &mut dyn engine::graphics::Graphics) -> Result {
    controllers::lift_opener::trigger(
        &game.mouse_xy,
        &game.components.lifts,
        &mut game.components.open,
        &mut game.systems.global_computer,
        &mut game.systems.messenger,
        graphics,
    )
    .then_try(|| {
        controllers::gate_opener::trigger(
            &game.mouse_xy,
            &game.components.gates,
            &mut game.components.open,
            &mut game.systems.global_computer,
            &mut game.systems.messenger,
            graphics,
        )
    })
}

fn try_to_demolish(game: &mut Game, graphics: &mut dyn engine::graphics::Graphics) -> Result {
    controllers::building_remover::trigger(
        &game.mouse_xy,
        graphics,
        &mut game.components,
        &mut game.systems,
    )
    .then_try(|| {
        controllers::gate_remover::trigger(
            &game.mouse_xy,
            &mut game.components,
            &mut game.systems.messenger,
            graphics,
        )
    })
    .then_try(|| {
        controllers::lift_remover::trigger(
            &game.mouse_xy,
            &mut game.components,
            &mut game.systems.messenger,
            graphics,
        )
    })
}
