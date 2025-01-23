use commons::geometry::{xy, XY};
use commons::grid::Grid;
use rand::{thread_rng, Rng};

use crate::utils::is_cliff;

pub fn generate_towns(
    tile_heights: &Grid<f32>,
    sea_level: f32,
    cliff_rise: f32,
    town_count: u32,
) -> Grid<bool> {
    let mut out = tile_heights.map(|_, _| false);
    let mut rng = thread_rng();

    for _ in 0..town_count {
        let xy = place_town(&mut rng, &out, tile_heights, sea_level, cliff_rise);
        out[xy] = true;
    }

    out
}

fn place_town<R>(
    rng: &mut R,
    towns: &Grid<bool>,
    tile_heights: &Grid<f32>,
    sea_level: f32,
    cliff_rise: f32,
) -> XY<u32>
where
    R: Rng,
{
    loop {
        let x = rng.gen_range(0..tile_heights.width());
        let y = rng.gen_range(0..tile_heights.height());

        let xy = xy(x, y);

        if tile_heights[xy] <= sea_level {
            continue;
        }
        if is_cliff(xy, tile_heights, cliff_rise) {
            continue;
        }
        if towns[xy] {
            continue;
        }
        return xy;
    }
}
