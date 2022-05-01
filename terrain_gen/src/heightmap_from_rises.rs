use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

use commons::grid::Grid;
use commons::unsafe_float_ordering;

use crate::{Heightmap, Rises};

pub struct HeightmapParameters<F>
where
    F: Fn((u32, u32)) -> bool,
{
    pub min_rise: f32,
    pub origin_fn: F,
}

pub fn heightmap_from_rises<F, B>(rises: &Rises, parameters: B) -> Heightmap
where
    B: Borrow<HeightmapParameters<F>>,
    F: Fn((u32, u32)) -> bool,
{
    let parameters = parameters.borrow();

    let mut visited = Grid::<bool>::default(rises.width(), rises.height());
    let mut out = Grid::from_fn(rises.width(), rises.height(), |xy| {
        if (parameters.origin_fn)(xy) {
            parameters.min_rise
        } else {
            f32::MAX
        }
    });

    let mut heap: BinaryHeap<HeapElement> = out
        .iter()
        .filter(|xy| (parameters.origin_fn)(*xy))
        .map(|xy| HeapElement { xy, z: 0.0 })
        .collect();

    while let Some(HeapElement { xy, z }) = heap.pop() {
        if visited[xy] {
            continue;
        }

        visited[xy] = true;

        let unvisited_neighbours = out
            .neighbours_4(&xy)
            .filter(|neighbour| !visited[neighbour])
            .collect::<Vec<_>>();

        for neighbour in unvisited_neighbours {
            let neighbour_z_through_xy = z + rises[neighbour] + parameters.min_rise;
            out[neighbour] = out[neighbour].min(neighbour_z_through_xy);
            heap.push(HeapElement {
                xy: neighbour,
                z: neighbour_z_through_xy,
            });
        }
    }

    out.normalize()
}

struct HeapElement {
    xy: (u32, u32),
    z: f32,
}

impl Eq for HeapElement {}

impl PartialEq for HeapElement {
    fn eq(&self, other: &Self) -> bool {
        self.xy == other.xy && self.z == other.z
    }
}

impl Ord for HeapElement {
    fn cmp(&self, other: &Self) -> Ordering {
        unsafe_float_ordering(&other.z, &self.z)
    }
}

impl PartialOrd for HeapElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use std::env::temp_dir;

    use commons::noise::simplex_noise;

    use super::*;

    #[test]
    fn test() {
        // given
        let power = 8;
        let weights = (0..power + 1)
            .map(|i| 1.0f32 / 1.125f32.powf((power - i) as f32))
            .collect::<Vec<_>>();
        let rises = simplex_noise(power, 1987, &weights)
            .normalize()
            .map(|_, z| (0.5 - z).abs() / 0.5);

        // when
        let heightmap = heightmap_from_rises(
            &rises,
            HeightmapParameters {
                min_rise: 1.0 / 1024.0,
                origin_fn: |xy| rises.is_border(xy),
            },
        );

        // then
        let temp_path = temp_dir().join("test.png");
        let temp_path = temp_path.to_str().unwrap();
        heightmap.to_image(temp_path).unwrap();

        let actual = image::open(temp_path).unwrap();
        let expected = image::open("test_resources/heightmap_from_rises/test.png").unwrap();
        assert_eq!(actual, expected);
    }
}
