use commons::geometry::{xy, xyz, XY, XYZ};
use commons::grid::Grid;
use engine::graphics::transform::Transform;
use engine::graphics::utils::{transformation_matrix, Transformation};
use nalgebra::Matrix4;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LiftBuildings {
    pub buildings: Vec<LiftBuilding>,
}

pub struct GlobalTransfer {
    pub position: XY<u32>,
    pub global_segment: usize,
}

impl LiftBuildings {
    pub fn wire_path(&self, terrain: &Grid<f32>) -> Vec<[XYZ<f32>; 2]> {
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

    pub fn get_pick_up_and_drop_off(
        &self,
        terrain: &Grid<f32>,
    ) -> (Option<GlobalTransfer>, Option<GlobalTransfer>) {
        let mut pick_up = None;
        let mut drop_off = None;
        let mut index = 0;
        let mut global_segment = 0;
        while pick_up.is_none() || drop_off.is_none() {
            if index >= self.buildings.len() {
                break;
            }
            let building = &self.buildings[index];

            if let Some(LocalTransfer { segment, class }) = building.class.transfer() {
                let position = building.wire_path_out(terrain)[segment][0];
                let position = xy(position.x.round() as u32, position.y.round() as u32);
                let transfer = GlobalTransfer {
                    position,
                    global_segment: global_segment + segment,
                };

                match class {
                    LocalTransferClass::PickUp => pick_up = Some(transfer),
                    LocalTransferClass::DropOff => drop_off = Some(transfer),
                }
            }

            index += 1;
            global_segment += building.class.wire_path_out().len() + 1;
        }
        (pick_up, drop_off)
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
            translation: Some(
                xyz(
                    self.position.x as f32,
                    self.position.y as f32,
                    terrain[self.position],
                ) + self.class.offset(),
            ),
            scale: Some(self.class.scale()),
            yaw: Some(self.yaw),
            ..Transformation::default()
        })
    }

    pub fn wire_path_out(&self, terrain: &Grid<f32>) -> Vec<[XYZ<f32>; 2]> {
        self.wire_path_over_terrain(&self.class.wire_path_out(), terrain)
    }

    pub fn wire_path_back(&self, terrain: &Grid<f32>) -> Vec<[XYZ<f32>; 2]> {
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

pub enum LocalTransferClass {
    PickUp,
    DropOff,
}

pub struct LocalTransfer {
    pub class: LocalTransferClass,
    pub segment: usize,
}

impl LiftBuildingClass {
    pub fn wire_path_out(&self) -> Vec<[XYZ<f32>; 2]> {
        match self {
            LiftBuildingClass::PickUpStation => {
                vec![[xyz(-0.5, -0.5, 0.0), xyz(0.5, -0.5, 0.0)]]
            }
            LiftBuildingClass::Pylon => {
                vec![[xyz(-0.125, -0.5, 1.0), xyz(0.125, -0.5, 1.0)]]
            }
            LiftBuildingClass::DropOffStation => {
                vec![[xyz(-0.5, -0.5, 0.0), xyz(0.5, -0.5, 0.0)]]
            }
        }
    }

    pub fn wire_path_back(&self) -> Vec<[XYZ<f32>; 2]> {
        match self {
            LiftBuildingClass::PickUpStation => vec![[xyz(0.5, 0.5, 0.0), xyz(-0.5, 0.5, 0.0)]],
            LiftBuildingClass::Pylon => {
                vec![[xyz(0.125, 0.5, 1.0), xyz(-0.125, 0.5, 1.0)]]
            }
            LiftBuildingClass::DropOffStation => vec![[xyz(0.5, 0.5, 0.0), xyz(-0.5, 0.5, 0.0)]],
        }
    }

    pub fn offset(&self) -> XYZ<f32> {
        match self {
            LiftBuildingClass::PickUpStation => xyz(4.0, 2.0, 3.0),
            LiftBuildingClass::Pylon => xyz(0.0, 0.0, 6.0),
            LiftBuildingClass::DropOffStation => xyz(-4.0, 2.0, 3.0),
        }
    }

    pub fn scale(&self) -> XYZ<f32> {
        match self {
            LiftBuildingClass::PickUpStation => xyz(8.0, 4.0, 1.0),
            LiftBuildingClass::Pylon => xyz(4.0, 3.0, 12.0),
            LiftBuildingClass::DropOffStation => xyz(8.0, 4.0, 1.0),
        }
    }

    pub fn transfer(&self) -> Option<LocalTransfer> {
        match self {
            LiftBuildingClass::PickUpStation => Some(LocalTransfer {
                class: LocalTransferClass::PickUp,
                segment: 0,
            }),
            LiftBuildingClass::DropOffStation => Some(LocalTransfer {
                class: LocalTransferClass::DropOff,
                segment: 0,
            }),
            _ => None,
        }
    }
}
