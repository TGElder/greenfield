use nalgebra::Matrix4;

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
