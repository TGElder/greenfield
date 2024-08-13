use crate::{handlers, Game};

#[derive(Clone, Copy)]
pub enum Mode {
    Piste,
    _Path,
    _Lift,
    _Gates,
    _Building,
    _Door,
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
    let update_selection = match mode {
        Mode::Piste => !game
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
        Mode::None => false,
        _ => panic!("Unsupported mode!"),
    };

    if update_selection {
        game.handlers.selection.handle(
            event,
            &game.mouse_xy,
            &game.components.terrain,
            graphics,
            &mut game.systems.terrain_artist,
        );
    }
}
