use std::collections::HashMap;

use engine::egui;

use crate::controllers::lift_builder;
use crate::model::lift_building::{LiftBuildingClass, LiftBuildings};
use crate::services;
use crate::systems::lift_building_artist;
use crate::widgets;

#[derive(Default)]
pub struct Widget {
    state: Option<State>,
}

struct State {
    pub lift_building_id: usize,
    pub class: LiftBuildingClass,
    pub new_class: LiftBuildingClass,
    pub show_undo_button: bool,
    pub undo: bool,
}

pub struct Input<'a> {
    pub mode: services::mode::Mode,
    pub lift_builder: &'a lift_builder::Controller,
    pub lift_buildings: &'a HashMap<usize, LiftBuildings>,
}

pub struct Output<'a> {
    pub lift_buildings: &'a mut HashMap<usize, LiftBuildings>,
    pub lift_building_artist: &'a mut lift_building_artist::System,
}

impl<'a> widgets::UiWidget<Input<'a>, Output<'a>> for Widget {
    fn init(&mut self, input: Input) {
        self.state = None;

        if input.mode != services::mode::Mode::Lift {
            return;
        }

        let Some(lift_building_id) = *input.lift_builder.lift_building_id() else {
            return;
        };
        let Some(lift_buildings) = input.lift_buildings.get(&lift_building_id) else {
            return;
        };
        let Some(lift_building) = lift_buildings.buildings.last() else {
            return;
        };

        self.state = Some(State {
            lift_building_id,
            class: lift_building.class,
            new_class: lift_building.class,
            show_undo_button: !lift_buildings.buildings.is_empty(),
            undo: false,
        });
    }

    fn draw(&mut self, ui: &mut engine::egui::Ui) {
        let Some(State {
            new_class,
            show_undo_button,
            undo,
            ..
        }) = self.state.as_mut()
        else {
            return;
        };
        ui.vertical(|ui| {
            ui.label("Lift");
            ui.horizontal(|ui| {
                ui.label("Building");
                egui::ComboBox::from_id_source(0)
                    .selected_text(describe_class(new_class))
                    .show_ui(ui, |ui| {
                        for option in CLASS_OPTIONS {
                            ui.selectable_value(new_class, option, describe_class(&option));
                        }
                    });
                if *show_undo_button {
                    *undo = ui.button("Undo").clicked();
                }
            });
        });
    }

    fn update(&mut self, output: Output) {
        let Some(State {
            lift_building_id,
            class,
            new_class,
            undo,
            ..
        }) = self.state
        else {
            return;
        };

        let Some(lift_buildings) = output.lift_buildings.get_mut(&lift_building_id) else {
            return;
        };

        if undo {
            lift_buildings.buildings.pop();
            output.lift_building_artist.redraw(lift_building_id);
        }

        if class != new_class {
            let Some(lift_building) = lift_buildings.buildings.last_mut() else {
                return;
            };
            lift_building.class = new_class;
            output.lift_building_artist.redraw(lift_building_id);
        }
    }
}

const CLASS_OPTIONS: [LiftBuildingClass; 3] = [
    LiftBuildingClass::PickUpStation,
    LiftBuildingClass::Pylon,
    LiftBuildingClass::DropOffStation,
];

fn describe_class(class: &LiftBuildingClass) -> &str {
    match class {
        LiftBuildingClass::PickUpStation => "Pick Up Station",
        LiftBuildingClass::Pylon => "Pylon",
        LiftBuildingClass::DropOffStation => "Drop Off Station",
    }
}
