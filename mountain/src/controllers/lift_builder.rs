use std::collections::{HashMap, HashSet};

use commons::geometry::{xy, XY, XYZ};
use commons::grid::Grid;

use crate::controllers::Result::{self, Action, NoAction};
use crate::model::carousel::{Car, Carousel};
use crate::model::direction::Direction;
use crate::model::entrance::Entrance;
use crate::model::exit::Exit;
use crate::model::lift::{self, Lift, Segment};
use crate::model::lift_building::{LiftBuilding, LiftBuildingClass, LiftBuildings};
use crate::model::open;
use crate::model::reservation::Reservation;
use crate::model::skiing::State;
use crate::network::velocity_encoding::{encode_velocity, VELOCITY_LEVELS};
use crate::services::id_allocator;
use crate::systems::{lift_building_artist, messenger, piste_computer};
use crate::utils;

pub const LIFT_VELOCITY: f32 = 2.0;
pub const CAR_INTERVAL_METERS: f32 = 10.0;

pub struct Controller {
    lift_building_id: Option<usize>,
}

pub struct TriggerParameters<'a> {
    pub mouse_xy: &'a Option<XY<u32>>,
    pub terrain: &'a Grid<f32>,
    pub piste_map: &'a Grid<Option<usize>>,
    pub lift_buildings: &'a mut HashMap<usize, LiftBuildings>,
    pub lifts: &'a mut HashMap<usize, Lift>,
    pub open: &'a mut HashMap<usize, open::Status>,
    pub id_allocator: &'a mut id_allocator::Service,
    pub carousels: &'a mut HashMap<usize, Carousel>,
    pub cars: &'a mut HashMap<usize, Car>,
    pub exits: &'a mut HashMap<usize, Exit>,
    pub entrances: &'a mut HashMap<usize, Entrance>,
    pub reservations: &'a mut Grid<HashMap<usize, Reservation>>,
    pub parents: &'a mut HashMap<usize, usize>,
    pub children: &'a mut HashMap<usize, Vec<usize>>,
    pub piste_computer: &'a mut piste_computer::System,
    pub messenger: &'a mut messenger::System,
    pub lift_building_artist: &'a mut lift_building_artist::System,
    pub graphics: &'a mut dyn engine::graphics::Graphics,
}

pub struct MouseMoveParameters<'a> {
    pub mouse_xy: &'a Option<XY<u32>>,
    pub terrain: &'a Grid<f32>,
    pub lift_buildings: &'a mut HashMap<usize, LiftBuildings>,
    pub lift_building_artist: &'a mut lift_building_artist::System,
    pub graphics: &'a mut dyn engine::graphics::Graphics,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            lift_building_id: None,
        }
    }

    pub fn lift_building_id(&self) -> &Option<usize> {
        &self.lift_building_id
    }

    pub fn trigger(&mut self, parameters: TriggerParameters) -> Result {
        let lift_building_id = self.lift_building_id.get_or_insert_with(|| {
            let id = parameters.id_allocator.next_id();
            parameters
                .lift_buildings
                .insert(id, LiftBuildings { buildings: vec![] });
            id
        });
        let Some(lift_buildings) = parameters.lift_buildings.get_mut(lift_building_id) else {
            return NoAction;
        };
        let last_lift_building = lift_buildings.buildings.last();

        // complete lift

        if last_lift_building.map(|building| building.class)
            == Some(LiftBuildingClass::DropOffStation)
        {
            self.create_lift(parameters);
            return Action;
        }

        // or add new building

        let Some(position) =
            get_position(parameters.mouse_xy, parameters.terrain, parameters.graphics)
        else {
            return NoAction;
        };
        lift_buildings.buildings.push(LiftBuilding {
            class: next_class(last_lift_building.map(|building| building.class)),
            position,
            yaw: 0.0,
        });
        parameters.lift_building_artist.redraw(*lift_building_id);

        Action
    }

    fn create_lift(
        &mut self,
        TriggerParameters {
            terrain,
            piste_map,
            lift_buildings,
            lifts,
            open,
            id_allocator,
            carousels,
            cars,
            exits,
            entrances,
            reservations,
            parents,
            children,
            piste_computer,
            messenger,
            ..
        }: TriggerParameters<'_>,
    ) {
        // create lift

        let Some(editing) = self.lift_building_id else {
            return;
        };
        let Some(lift_buildings) = lift_buildings.get(&editing) else {
            return;
        };

        let (pick_up, drop_off) = lift_buildings.get_pick_up_and_drop_off(terrain);
        if pick_up.is_none() {
            messenger.send("Lift needs pick up position!");
            return;
        }
        if drop_off.is_none() {
            messenger.send("Lift needs drop off position!");
            return;
        }
        let pick_up = pick_up.unwrap();
        let drop_off = drop_off.unwrap();

        let Some(origin_piste_id) = piste_map[pick_up.position] else {
            messenger.send("Lift needs piste at pick up position!");
            return;
        };
        let Some(destination_piste_id) = piste_map[drop_off.position] else {
            messenger.send("Lift needs piste at drop off position!");
            return;
        };

        let lift_id = id_allocator.next_id();
        let carousel_id = id_allocator.next_id();
        let pick_up_id = id_allocator.next_id();
        let drop_off_id = id_allocator.next_id();

        children.entry(lift_id).or_default().append(&mut vec![
            carousel_id,
            pick_up_id,
            drop_off_id,
        ]);
        parents.entry(carousel_id).insert_entry(lift_id);
        parents.entry(pick_up_id).insert_entry(lift_id);
        parents.entry(drop_off_id).insert_entry(lift_id);

        let segments = lift_buildings
            .wire_path(terrain)
            .drain(..)
            .map(|segment| Segment::new(segment[0], segment[1]))
            .collect();
        let travel_direction = get_direction(&pick_up.position, &drop_off.position);

        let lift = Lift {
            segments,
            pick_up: lift::Portal {
                id: pick_up_id,
                segment: pick_up.global_segment,
                state: State {
                    position: pick_up.position,
                    travel_direction,
                    velocity: 0,
                },
            },
            drop_off: lift::Portal {
                id: drop_off_id,
                segment: drop_off.global_segment,
                state: State {
                    position: drop_off.position,
                    travel_direction,
                    velocity: encode_velocity(&LIFT_VELOCITY).unwrap_or(VELOCITY_LEVELS - 1),
                },
            },
            carousel_id,
        };

        // opening lift

        open.insert(lift_id, open::Status::Open);
        open.insert(lift.pick_up.id, open::Status::Open);
        open.insert(lift.drop_off.id, open::Status::Open);

        // setup carousel

        let new_cars =
            utils::carousel::create_cars(carousel_id, &lift.segments, &CAR_INTERVAL_METERS);

        let car_ids = (0..new_cars.len())
            .map(|_| id_allocator.next_id())
            .collect::<Vec<_>>();

        children
            .entry(carousel_id)
            .or_default()
            .append(&mut car_ids.clone());

        car_ids.iter().zip(new_cars).for_each(|(id, car)| {
            cars.insert(*id, car);
            parents.insert(*id, carousel_id);
        });

        carousels.insert(
            carousel_id,
            Carousel {
                lift_id,
                velocity: LIFT_VELOCITY,
                car_ids,
            },
        );

        // setup exit

        exits.insert(
            lift.pick_up.id,
            Exit {
                origin_piste_id,
                stationary_states: HashSet::from([lift.pick_up.state.stationary()]),
            },
        );

        // setup entrance

        entrances.insert(
            lift.drop_off.id,
            Entrance {
                destination_piste_id,
                stationary_states: HashSet::from([lift.drop_off.state.stationary()]),
            },
        );

        // reserve pick up position

        reservations[lift.pick_up.state.position].insert(lift.pick_up.id, Reservation::Structure);

        // register lift

        lifts.insert(lift_id, lift);

        // recomputing pistes
        piste_computer.compute(origin_piste_id);
        piste_computer.compute(destination_piste_id);

        // clear editing

        self.lift_building_id = None;
    }

    pub fn on_mouse_move(
        &mut self,

        MouseMoveParameters {
            mouse_xy,
            terrain,
            lift_buildings,
            lift_building_artist,
            graphics,
        }: MouseMoveParameters<'_>,
    ) {
        let Some(lift_building_id) = self.lift_building_id else {
            return;
        };
        let Some(lift_buildings) = lift_buildings.get_mut(&lift_building_id) else {
            return;
        };
        let lift_buildings = &mut lift_buildings.buildings;

        if lift_buildings.is_empty() {
            return;
        }

        let Some(position) = get_position(mouse_xy, terrain, graphics) else {
            return;
        };

        let lift_building_count = lift_buildings.len();
        if lift_building_count >= 2 {
            let penultimate_building = &mut lift_buildings[lift_building_count - 2];
            let yaw = (xy(position.x as f32, position.y as f32)
                - xy(
                    penultimate_building.position.x as f32,
                    penultimate_building.position.y as f32,
                ))
            .angle();
            penultimate_building.yaw = yaw;
            lift_buildings.last_mut().unwrap().yaw = yaw;
        }

        lift_buildings.last_mut().unwrap().position = position;

        lift_building_artist.redraw(lift_building_id);
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

fn next_class(class: Option<LiftBuildingClass>) -> LiftBuildingClass {
    match class {
        None => LiftBuildingClass::PickUpStation,
        Some(_) => LiftBuildingClass::Pylon,
    }
}

fn get_direction(from: &XY<u32>, to: &XY<u32>) -> Direction {
    let vector = xy(to.x as f32 - from.x as f32, to.y as f32 - from.y as f32);
    Direction::snap_to_direction(vector.angle())
}
