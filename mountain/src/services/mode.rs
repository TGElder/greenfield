use crate::controllers::lift_builder::MouseMoveParameters;
use crate::controllers::Result::{self, Action, NoAction};
use crate::handlers::selection::Parameters;
use crate::model::selection::Selection;
use crate::{controllers, Game};

#[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
pub enum Mode {
    #[default]
    None,
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

    pub fn set_mode(&mut self, mode: Mode, selection: &mut Selection) {
        self.mode = mode;
        if !mode.has_selection() {
            selection.cells.clear();
        }
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
            event,
            Parameters {
                bindings: &game.bindings.selection,
                mouse_xy: &game.mouse_xy,
                terrain: &game.components.terrain,
                selection: &mut game.components.selection,
                graphics,
            },
        );
    }
}

fn try_to_handle(
    mode: Mode,
    event: &engine::events::Event,
    game: &mut Game,
    graphics: &mut dyn engine::graphics::Graphics,
) -> controllers::Result {
    if matches!(event, engine::events::Event::MouseMoved(..)) {
        if let Mode::Lift = mode {
            game.controllers
                .lift_builder
                .on_mouse_move(MouseMoveParameters {
                    mouse_xy: &game.mouse_xy,
                    terrain: &game.components.terrain,
                    lift_buildings: &mut game.components.lift_buildings,
                    lift_building_artist: &mut game.systems.lift_building_artist,
                    graphics,
                });
        }
    }

    if !game.bindings.action.binds_event(event) {
        return NoAction;
    }

    match mode {
        Mode::Query => {
            controllers::entity_window::trigger(controllers::entity_window::Parameters {
                mouse_xy: &game.mouse_xy,
                lifts: &game.components.lifts,
                gates: &game.components.gates,
                pistes: &game.components.pistes,
                windows: &mut game.widgets.windows,
                graphics,
            })
        }
        Mode::Piste => game
            .controllers
            .piste_builder
            .trigger(controllers::piste_builder::Parameters {
                pistes: &mut game.components.pistes,
                piste_map: &mut game.components.piste_map,
                open: &mut game.components.open,
                selection: &mut game.components.selection,
                terrain_artist: &mut game.systems.terrain_artist,
                tree_artist: &mut game.systems.tree_artist,
                id_allocator: &mut game.components.services.id_allocator,
                messenger: &mut game.systems.messenger,
            })
            .then_try(|| {
                game.controllers
                    .piste_eraser
                    .trigger(controllers::piste_eraser::Parameters {
                        open: &game.components.open,
                        lifts: &game.components.lifts,
                        gates: &game.components.gates,
                        doors: &game.components.doors,
                        pistes: &mut game.components.pistes,
                        piste_map: &mut game.components.piste_map,
                        selection: &mut game.components.selection,
                        terrain_artist: &mut game.systems.terrain_artist,
                        tree_artist: &mut game.systems.tree_artist,
                        messenger: &mut game.systems.messenger,
                    })
            }),
        Mode::Path => game
            .controllers
            .path_builder
            .trigger(controllers::piste_builder::Parameters {
                pistes: &mut game.components.pistes,
                piste_map: &mut game.components.piste_map,
                open: &mut game.components.open,
                selection: &mut game.components.selection,
                terrain_artist: &mut game.systems.terrain_artist,
                tree_artist: &mut game.systems.tree_artist,
                id_allocator: &mut game.components.services.id_allocator,
                messenger: &mut game.systems.messenger,
            })
            .then_try(|| {
                game.controllers
                    .piste_eraser
                    .trigger(controllers::piste_eraser::Parameters {
                        open: &game.components.open,
                        lifts: &game.components.lifts,
                        gates: &game.components.gates,
                        doors: &game.components.doors,
                        pistes: &mut game.components.pistes,
                        piste_map: &mut game.components.piste_map,
                        selection: &mut game.components.selection,
                        terrain_artist: &mut game.systems.terrain_artist,
                        tree_artist: &mut game.systems.tree_artist,
                        messenger: &mut game.systems.messenger,
                    })
            }),
        Mode::Lift => {
            game.controllers
                .lift_builder
                .trigger(controllers::lift_builder::TriggerParameters {
                    mouse_xy: &game.mouse_xy,
                    terrain: &game.components.terrain,
                    piste_map: &game.components.piste_map,
                    lift_buildings: &mut game.components.lift_buildings,
                    lifts: &mut game.components.lifts,
                    open: &mut game.components.open,
                    id_allocator: &mut game.components.services.id_allocator,
                    carousels: &mut game.components.carousels,
                    cars: &mut game.components.cars,
                    exits: &mut game.components.exits,
                    entrances: &mut game.components.entrances,
                    reservations: &mut game.components.reservations,
                    parents: &mut game.components.parents,
                    children: &mut game.components.children,
                    piste_computer: &mut game.systems.piste_computer,
                    messenger: &mut game.systems.messenger,
                    lift_building_artist: &mut game.systems.lift_building_artist,
                    graphics,
                })
        }
        Mode::Building => game.controllers.building_builder.select(
            controllers::building_builder::SelectParameters {
                selection: &mut game.components.selection,
                id_allocator: &mut game.components.services.id_allocator,
                buildings: &mut game.components.buildings,
                tree_artist: &mut game.systems.tree_artist,
            },
        ),
        Mode::Gate => controllers::gate_builder::trigger(controllers::gate_builder::Parameters {
            piste_map: &game.components.piste_map,
            selection: &mut game.components.selection,
            terrain_artist: &mut game.systems.terrain_artist,
            id_allocator: &mut game.components.services.id_allocator,
            gates: &mut game.components.gates,
            entrances: &mut game.components.entrances,
            exits: &mut game.components.exits,
            open: &mut game.components.open,
            reservations: &mut game.components.reservations,
            piste_computer: &mut game.systems.piste_computer,
            messenger: &mut game.systems.messenger,
        }),
        Mode::Door => controllers::door_builder::trigger(controllers::door_builder::Parameters {
            pistes: &game.components.pistes,
            buildings: &game.components.buildings,
            selection: &mut game.components.selection,
            id_allocator: &mut game.components.services.id_allocator,
            doors: &mut game.components.doors,
            entrances: &mut game.components.entrances,
            exits: &mut game.components.exits,
            open: &mut game.components.open,
            parents: &mut game.components.parents,
            children: &mut game.components.children,
            piste_computer: &mut game.systems.piste_computer,
            messenger: &mut game.systems.messenger,
        }),
        Mode::Demolish => try_to_demolish(game, graphics),
        _ => NoAction,
    }
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
            &mut game.systems.piste_computer,
            &mut game.systems.messenger,
            graphics,
        )
    })
    .then_try(|| {
        controllers::lift_remover::trigger(
            &game.mouse_xy,
            &mut game.components,
            &mut game.systems.piste_computer,
            &mut game.systems.messenger,
            graphics,
        )
    })
}
