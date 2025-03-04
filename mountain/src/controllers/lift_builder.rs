use std::collections::HashMap;

use commons::geometry::{xy, XY, XYZ};
use commons::grid::Grid;

use crate::controllers::Result::{self, Action, NoAction};
use crate::model::lift_building::{LiftBuilding, LiftBuildingClass, LiftBuildings};
use crate::services::id_allocator;

pub struct Controller {
    editing: Option<usize>,
    brush: LiftBuildingClass,
}

pub struct TriggerParameters<'a> {
    pub mouse_xy: &'a Option<XY<u32>>,
    pub terrain: &'a Grid<f32>,
    pub id_allocator: &'a mut id_allocator::Service,
    pub lift_buildings: &'a mut HashMap<usize, LiftBuildings>,
    pub graphics: &'a mut dyn engine::graphics::Graphics,
}

pub struct MouseMoveParameters<'a> {
    pub mouse_xy: &'a Option<XY<u32>>,
    pub terrain: &'a Grid<f32>,
    pub lift_buildings: &'a mut HashMap<usize, LiftBuildings>,
    pub graphics: &'a mut dyn engine::graphics::Graphics,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            editing: None,
            brush: LiftBuildingClass::Pylon,
        }
    }

    pub fn lift_building_id(&self) -> &Option<usize> {
        &self.editing
    }

    pub fn trigger(
        &mut self,
        TriggerParameters {
            mouse_xy,
            terrain,
            id_allocator,
            graphics,
            lift_buildings,
            ..
        }: TriggerParameters<'_>,
    ) -> Result {
        let editing = self.editing.get_or_insert_with(|| {
            let id = id_allocator.next_id();
            lift_buildings.insert(id, LiftBuildings { buildings: vec![] });
            id
        });
        let Some(lift_buildings) = lift_buildings.get_mut(editing) else {
            return NoAction;
        };
        let last_lift_building = lift_buildings.buildings.last();

        if last_lift_building.map(|building| building.class)
            == Some(LiftBuildingClass::DropOffStation)
        {
            self.editing = None;
            return Action;
        }

        let Some(position) = get_position(mouse_xy, terrain, graphics) else {
            return NoAction;
        };

        lift_buildings.buildings.push(LiftBuilding {
            class: next_class(last_lift_building.map(|building| building.class)),
            position,
            yaw: 0.0,
        });

        Action
    }

    pub fn on_mouse_move(
        &mut self,

        MouseMoveParameters {
            mouse_xy,
            terrain,
            lift_buildings,
            graphics,
        }: MouseMoveParameters<'_>,
    ) {
        let Some(editing) = self.editing else {
            return;
        };
        let Some(lift_buildings) = lift_buildings.get_mut(&editing) else {
            return;
        };
        let Some(lift_building) = lift_buildings.buildings.last_mut() else {
            return;
        };

        let Some(position) = get_position(mouse_xy, terrain, graphics) else {
            return;
        };

        lift_building.position = position;
    }
}

fn next_class(class: Option<LiftBuildingClass>) -> LiftBuildingClass {
    match class {
        None => LiftBuildingClass::PickUpStation,
        Some(_) => LiftBuildingClass::Pylon,
    }
}

fn get_position(
    mouse_xy: &Option<XY<u32>>,
    terrain: &Grid<f32>,
    graphics: &mut dyn engine::graphics::Graphics,
) -> Option<XY<u32>> {
    let Some(mouse_xy) = mouse_xy else {
        return None;
    };
    let Ok(XYZ { x, y, .. }) = graphics.world_xyz_at(mouse_xy) else {
        return None;
    };
    let position = xy(x.round() as u32, y.round() as u32);
    if !terrain.in_bounds(position) {
        return None;
    }

    Some(position)
}
