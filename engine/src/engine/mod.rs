pub mod errors;

pub trait Engine {
    fn shutdown(&mut self);
}
