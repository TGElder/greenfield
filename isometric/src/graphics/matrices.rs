use nalgebra::Matrix4;

pub fn isometric(yaw: &f32, pitch: &f32) -> Matrix4<f32> {
    let yc = yaw.cos();
    let ys = yaw.sin();
    let pc = pitch.cos();
    let ps = pitch.sin();
    [
        [yc, -ys * pc, 0.0, 0.0],
        [ys, yc * pc, 0.0, 0.0],
        [0.0, ps, -1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
    .into()
}

pub fn scale(scale: &f32) -> Matrix4<f32> {
    [
        [*scale, 0.0, 0.0, 0.0],
        [0.0, *scale, 0.0, 0.0],
        [0.0, 0.0, *scale, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
    .into()
}
