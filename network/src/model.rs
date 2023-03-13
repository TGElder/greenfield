#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
    pub cost: u32,
}

pub trait Network {
    fn edges<'a>(&'a self, from: &'a usize) -> Box<dyn Iterator<Item = Edge> + 'a>;
}
