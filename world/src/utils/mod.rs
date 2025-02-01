use commons::geometry::XY;
use commons::grid::{Grid, CORNERS};
use network::model::{Edge, InNetwork};

pub fn tile_heights(terrain: &Grid<f32>) -> Grid<f32> {
    Grid::from_fn(terrain.width() - 1, terrain.height() - 1, |xy| {
        terrain
            .offsets(xy, &CORNERS)
            .map(|corner| terrain[corner])
            .sum::<f32>()
            / 4.0
    })
}

pub fn is_cliff(position: &XY<u32>, tile_heights: &Grid<f32>, cliff_rise: f32) -> bool {
    tile_heights
        .neighbours_4(position)
        .any(|neighbour| (tile_heights[position] - tile_heights[neighbour]).abs() >= cliff_rise)
}

pub fn cost(
    from: &XY<u32>,
    to: &XY<u32>,
    tile_heights: &Grid<f32>,
    sea_level: f32,
    cliff_rise: f32,
) -> Option<u32> {
    if tile_heights[from] < sea_level || tile_heights[to] < sea_level {
        return None;
    }
    let rise = (tile_heights[from] - tile_heights[to]).abs();
    if rise > cliff_rise {
        return None;
    }
    Some(((rise + 1.0) * 1000.0).round() as u32)
}

pub struct Network<'a> {
    pub sea_level: f32,
    pub cliff_rise: f32,
    pub tile_heights: &'a Grid<f32>,
}

impl InNetwork<XY<u32>> for Network<'_> {
    fn edges_in<'a>(
        &'a self,
        to: &'a XY<u32>,
    ) -> Box<dyn Iterator<Item = network::model::Edge<XY<u32>>> + 'a> {
        Box::new(self.tile_heights.neighbours_4(to).filter_map(|from| {
            let cost = cost(
                &from,
                to,
                self.tile_heights,
                self.sea_level,
                self.cliff_rise,
            )?;
            Some(Edge {
                from,
                to: *to,
                cost,
            })
        }))
    }
}
