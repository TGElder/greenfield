#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Edge<T> {
    pub from: T,
    pub to: T,
    pub cost: u32,
}

pub trait OutNetwork<T> {
    fn edges_out<'a>(&'a self, from: &'a T) -> Box<dyn Iterator<Item = Edge<T>> + 'a>;
}

pub trait InNetwork<T> {
    fn edges_in<'a>(&'a self, to: &'a T) -> Box<dyn Iterator<Item = Edge<T>> + 'a>;
}
