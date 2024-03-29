use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Tree {
    pub yaw: f32,
    pub height: f32,
}
