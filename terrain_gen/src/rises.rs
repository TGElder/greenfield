use std::cmp::Ordering;
use std::collections::BinaryHeap;

use commons::grid::Grid;
use commons::unsafe_float_ordering;

use crate::Heightmap;

pub struct Rises {
    rises: Grid<f32>,
}

pub trait AsRises {
    fn as_rises(self) -> Rises;
}

impl AsRises for Heightmap {
    fn as_rises(self) -> Rises {
        Rises { rises: self }
    }
}

pub trait ToHeightmap {
    fn to_heightmap(self) -> Heightmap;
}

impl ToHeightmap for Rises {
    fn to_heightmap(self) -> Heightmap {
        let rises = self.rises;

        let mut visited = Grid::<bool>::default(rises.width(), rises.height());
        let mut out = Grid::from_fn(rises.width(), rises.height(), |xy| {
            if visited.is_border(xy) {
                0.0
            } else {
                f32::MAX
            }
        });

        let mut heap: BinaryHeap<HeapElement> = out
            .iter()
            .filter(|xy| out.is_border(xy))
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
                let neighbour_z_through_xy = z + rises[neighbour];
                out[neighbour] = out[neighbour].min(neighbour_z_through_xy);
                heap.push(HeapElement {
                    xy: neighbour,
                    z: neighbour_z_through_xy,
                });
            }
        }

        out.normalize()
    }
}

struct HeapElement {
    xy: (u32, u32),
    z: f32,
}

impl Eq for HeapElement {}

impl PartialEq for HeapElement {
    fn eq(&self, other: &Self) -> bool {
        self.xy == other.xy
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
        let noise = simplex_noise(power, 1987, &weights)
            .normalize()
            .map(|_, z| (0.5 - z).abs() / 0.5);

        // when
        let heightmap = noise.as_rises().to_heightmap();

        // then
        let temp_path = temp_dir().join("test.png");
        let temp_path = temp_path.to_str().unwrap();
        heightmap.to_image(temp_path).unwrap();

        let actual = image::open(temp_path).unwrap();
        let expected = image::open("test_resources/rises_to_heightmap/test.png").unwrap();
        assert_eq!(actual, expected);
    }
}
