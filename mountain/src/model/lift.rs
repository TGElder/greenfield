use commons::geometry::XY;

pub struct Lift {
    pub from: XY<u32>,
    pub to: XY<u32>,
    pub max_entry_velocity: f32,
}
