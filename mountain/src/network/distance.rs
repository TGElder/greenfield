use commons::geometry::{xy, XY};
use network::model::{Edge, InNetwork};

use crate::model::piste::Piste;

const OFFSETS_8: [XY<i32>; 8] = [
    xy(-1, -1),
    xy(0, -1),
    xy(1, -1),
    xy(-1, 0),
    xy(1, 0),
    xy(-1, 1),
    xy(0, 1),
    xy(1, 1),
];

pub struct DistanceNetwork<'a> {
    piste: &'a Piste,
}

impl<'a> DistanceNetwork<'a> {
    pub fn new(piste: &'a Piste) -> DistanceNetwork<'a> {
        DistanceNetwork { piste }
    }
}

impl<'a> InNetwork<XY<u32>> for DistanceNetwork<'a> {
    fn edges_in<'b>(
        &'b self,
        to: &'b XY<u32>,
    ) -> Box<dyn Iterator<Item = network::model::Edge<XY<u32>>> + 'b> {
        let iter = OFFSETS_8
            .iter()
            .flat_map(move |offset| {
                self.piste
                    .grid
                    .offset(to, offset)
                    .map(|from| (offset, from))
            })
            .filter(|(_, from)| self.piste.grid.in_bounds(from))
            .filter(|(_, from)| self.piste.grid[from])
            .map(|(offset, from)| Edge {
                from,
                to: *to,
                cost: distance(offset),
            });

        Box::new(iter)
    }
}

fn distance(XY { x, y }: &XY<i32>) -> u32 {
    match x.abs() + y.abs() {
        1 => 1000,
        2 => 1414,
        value => panic!(
            "{} is not a valid key for values precomputed to cover offsets in OFFSETS_8",
            value
        ),
    }
}
