use std::f32::consts::PI;

use commons::color::Rgb;
use commons::geometry::{xy, xyz, XYZ};
use commons::grid::Grid;

use engine::graphics::elements::Quad;

use engine::graphics::Graphics;

use nalgebra::Vector3;

pub fn draw_terrain(terrain: &Grid<f32>, graphics: &mut dyn Graphics) {
    let slab_size = 256;
    let slabs = xy(
        (terrain.width() / slab_size) + 1,
        (terrain.height() / slab_size) + 1,
    );
    for x in 0..slabs.x {
        for y in 0..slabs.y {
            let slab = xy(x, y);
            let mut quads = Vec::with_capacity(
                (terrain.width() - 1) as usize * (terrain.height() - 1) as usize,
            );
            for x in 0..slab_size {
                let x = slab.x * slab_size + x;
                if x >= terrain.width() - 1 {
                    break;
                }
                for y in 0..slab_size {
                    let y = slab.y * slab_size + y;
                    if y >= terrain.height() - 1 {
                        break;
                    }
                    let corners = [xy(0, 0), xy(1, 0), xy(1, 1), xy(0, 1)]
                        .iter()
                        .map(|d| {
                            xyz(
                                (x + d.x) as f32,
                                (y + d.y) as f32,
                                terrain[xy(x + d.x, y + d.y)],
                            )
                        })
                        .collect::<Vec<_>>();

                    quads.push(Quad {
                        color: color(&corners),
                        corners: [corners[0], corners[1], corners[2], corners[3]],
                    });
                }
            }

            let index = graphics.create_quads().unwrap();
            graphics.draw_quads(&index, &quads).unwrap();
        }
    }
}

fn color(corners: &[XYZ<f32>]) -> Rgb<f32> {
    let light_direction: Vector3<f32> = Vector3::new(1.0, 0.0, 0.0);
    let base_color: Rgb<f32> = Rgb::new(1.0, 1.0, 1.0);

    let corners = corners
        .iter()
        .map(|XYZ { x, y, z }| Vector3::new(*x, *y, *z))
        .collect::<Vec<_>>();

    let u = corners[0] - corners[2];
    let v = corners[1] - corners[3];
    let normal = u.cross(&v);
    let angle = normal.angle(&light_direction);
    let shade = angle / PI;
    Rgb::new(
        base_color.r * shade,
        base_color.g * shade,
        base_color.b * shade,
    )
}
