use commons::geometry::{xyz, XY, XYZ};
use commons::grid::Grid;
use engine::graphics::transform::Transform;
use engine::graphics::utils::{transformation_matrix, Transformation};
use nalgebra::Matrix4;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LiftBuildings {
    pub buildings: Vec<LiftBuilding>,
}

impl LiftBuildings {
    pub fn wire_path_over_terrain(&self, terrain: &Grid<f32>) -> Vec<[XYZ<f32>; 2]> {
        let unlinked = self
            .buildings
            .iter()
            .flat_map(|building| building.wire_path_out(terrain))
            .chain(
                self.buildings
                    .iter()
                    .rev()
                    .flat_map(|building| building.wire_path_back(terrain)),
            )
            .collect::<Vec<_>>();

        let mut linked = Vec::with_capacity(unlinked.len() * 2);
        for i in 0..unlinked.len() {
            let j = (i + 1) % unlinked.len();
            linked.push(unlinked[i]);
            linked.push([unlinked[i][1], unlinked[j][0]]);
        }
        linked
    }
}

#[derive(Serialize, Deserialize)]
pub struct LiftBuilding {
    pub class: LiftBuildingClass,
    pub position: XY<u32>,
    pub yaw: f32,
}

impl LiftBuilding {
    pub fn transformation_matrix(&self, terrain: &Grid<f32>) -> Matrix4<f32> {
        transformation_matrix(Transformation {
            translation: Some(xyz(
                self.position.x as f32,
                self.position.y as f32,
                terrain[self.position],
            )),
            scale: Some(self.class.footprint()),
            yaw: Some(self.yaw),
            ..Transformation::default()
        })
    }

    fn wire_path_out(&self, terrain: &Grid<f32>) -> Vec<[XYZ<f32>; 2]> {
        self.wire_path_over_terrain(&self.class.wire_path_out(), terrain)
    }

    fn wire_path_back(&self, terrain: &Grid<f32>) -> Vec<[XYZ<f32>; 2]> {
        self.wire_path_over_terrain(&self.class.wire_path_back(), terrain)
    }

    fn wire_path_over_terrain(
        &self,
        wire_path: &[[XYZ<f32>; 2]],
        terrain: &Grid<f32>,
    ) -> Vec<[XYZ<f32>; 2]> {
        let matrix = self.transformation_matrix(terrain);

        wire_path
            .iter()
            .map(|[a, b]| [a.transform(&matrix), b.transform(&matrix)])
            .collect()
    }
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum LiftBuildingClass {
    PickUpStation,
    Pylon,
    DropOffStation,
}

impl LiftBuildingClass {
    pub fn wire_path_out(&self) -> Vec<[XYZ<f32>; 2]> {
        match self {
            LiftBuildingClass::PickUpStation => {
                vec![[xyz(-0.5, -0.5, 1.0), xyz(0.5, -0.5, 1.0)]]
            }
            LiftBuildingClass::Pylon => {
                vec![[xyz(-0.125, -0.5, 1.0), xyz(0.125, -0.5, 1.0)]]
            }
            LiftBuildingClass::DropOffStation => {
                vec![[xyz(-0.5, -0.5, 1.0), xyz(0.5, -0.5, 1.0)]]
            }
        }
    }

    pub fn wire_path_back(&self) -> Vec<[XYZ<f32>; 2]> {
        match self {
            LiftBuildingClass::PickUpStation => vec![[xyz(0.5, 0.5, 1.0), xyz(-0.5, 0.5, 1.0)]],
            LiftBuildingClass::Pylon => {
                vec![[xyz(0.125, 0.5, 1.0), xyz(-0.125, 0.5, 1.0)]]
            }
            LiftBuildingClass::DropOffStation => vec![[xyz(0.5, 0.5, 1.0), xyz(-0.5, 0.5, 1.0)]],
        }
    }

    pub fn footprint(&self) -> XYZ<f32> {
        match self {
            LiftBuildingClass::PickUpStation => xyz(9.0, 3.0, 3.5),
            LiftBuildingClass::Pylon => xyz(4.0, 3.0, 12.0),
            LiftBuildingClass::DropOffStation => xyz(8.0, 4.0, 3.5),
        }
    }

    pub fn pick_up_segment(&self) -> Option<usize> {
        match self {
            LiftBuildingClass::PickUpStation => Some(0),
            _ => None,
        }
    }

    pub fn drop_off_segment(&self) -> Option<usize> {
        match self {
            LiftBuildingClass::DropOffStation => Some(0),
            _ => None,
        }
    }
}
