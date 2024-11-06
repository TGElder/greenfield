pub mod building_builder;
pub mod building_remover;
pub mod door_builder;
pub mod entity_window;
pub mod gate_builder;
pub mod gate_opener;
pub mod gate_remover;
pub mod lift_builder;
pub mod lift_opener;
pub mod lift_remover;
pub mod piste_builder;
pub mod piste_eraser;
pub mod save;

#[derive(Debug, PartialEq)]
pub enum Result {
    NoAction,
    Action,
}

impl Result {
    pub fn then_try<T>(&self, mut function: T) -> Result
    where
        T: FnMut() -> Result,
    {
        match self {
            Result::NoAction => function(),
            Result::Action => Result::Action,
        }
    }
}
