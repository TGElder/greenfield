#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Edge<T> {
    pub from: T,
    pub to: T,
    pub cost: u32,
}

pub trait Network<T> {
    fn edges<'a>(&'a self, from: &'a T) -> Box<dyn Iterator<Item = Edge<T>> + 'a>;
}
