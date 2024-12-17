use commons::geometry::{xyz, XY, XYZ};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum StructureClass {
    ChairliftBaseStation,
}

impl StructureClass {
    pub fn wire_path_out(&self) -> Vec<[XYZ<f32>; 2]> {
        match self {
            StructureClass::ChairliftBaseStation => {
                vec![[xyz(-0.5, -0.5, 0.5), xyz(0.5, -0.5, 0.5)]]
            }
        }
    }

    pub fn wire_path_back(&self) -> Vec<[XYZ<f32>; 2]> {
        match self {
            StructureClass::ChairliftBaseStation => vec![[xyz(0.5, 0.5, 0.5), xyz(-0.5, 0.5, 0.5)]],
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Structure {
    pub class: StructureClass,
    pub position: XY<u32>,
    pub footprint: XYZ<u32>,
    pub rotation: f32,
    pub under_construction: bool,
}
