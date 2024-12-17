use commons::geometry::{XY, XYZ};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum StructureClass {
    ChairliftBaseStation,
}

#[derive(Serialize, Deserialize)]
pub struct Structure {
    pub class: StructureClass,
    pub position: XY<u32>,
    pub footprint: XYZ<u32>,
    pub rotation: f32,
    pub wire_path_out: Vec<[XYZ<f32>; 2]>,
    pub wire_path_back: Vec<[XYZ<f32>; 2]>,
    pub under_construction: bool,
}
