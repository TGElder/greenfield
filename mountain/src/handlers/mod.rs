use engine::events::Event;
use engine::graphics::Graphics;

pub mod building_builder;
pub mod building_remover;
pub mod clock;
pub mod door_builder;
pub mod gate_builder;
pub mod gate_opener;
pub mod gate_remover;
pub mod lift_builder;
pub mod lift_opener;
pub mod lift_remover;
pub mod lift_targeter;
pub mod mode;
pub mod piste_builder;
pub mod piste_computer;
pub mod piste_eraser;
pub mod piste_highlighter;
pub mod save;
pub mod selection;
pub mod skier_debugger;

#[derive(PartialEq)]
pub enum HandlerResult {
    EventPersists,
    EventConsumed,
}
