use nalgebra::{Matrix4, Vector4};

pub fn isometric_projection(yaw: &f32, pitch: &f32) -> Matrix4<f32> {
    let yc = yaw.cos();
    let ys = yaw.sin();
    let pc = pitch.cos();
    let ps = pitch.sin();
    Matrix4::new(
        yc,
        ys,
        0.0,
        0.0, //
        -ys * pc,
        yc * pc,
        ps,
        0.0, //
        0.0,
        0.0,
        -1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    )
}

fn look_at_translation(
    [a, b, c]: &[f32; 3],
    [x, y]: &[f32; 2],
    transform: &Matrix4<f32>,
) -> Matrix4<f32> {
    let point = Vector4::new(*a, *b, *c, 1.0);

    let offsets = transform * point;

    let out = Matrix4::new(
        1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
        0.0,
        -offsets.x + x,
        -offsets.y + y,
        1.0,
        1.0,
    )
    .transpose();

    out
}
