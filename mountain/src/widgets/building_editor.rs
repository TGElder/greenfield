use std::collections::HashMap;

use engine::egui;

use crate::controllers::building_builder;
use crate::model::building::{Building, Roof, ROOFS};
use crate::services;
use crate::systems::building_artist;
use crate::widgets;

#[derive(Default)]
pub struct Widget {
    state: Option<State>,
}

struct State {
    building_id: usize,
    height: u32,
    roof: Roof,
}

pub struct Input<'a> {
    pub mode: services::mode::Mode,
    pub builder: &'a building_builder::Controller,
    pub buildings: &'a HashMap<usize, Building>,
}

pub struct Output<'a> {
    pub buildings: &'a mut HashMap<usize, Building>,
    pub artist: &'a mut building_artist::System,
}

impl<'a> widgets::Widget<Input<'a>, Output<'a>> for Widget {
    fn init(&mut self, input: Input) {
        if input.mode != services::mode::Mode::Building {
            return;
        }

        let &building_builder::State::Editing { building_id } = input.builder.state() else {
            return;
        };

        let Some(building) = input.buildings.get(&building_id) else {
            return;
        };

        self.state = Some(State {
            building_id,
            height: building.height,
            roof: building.roof,
        });
    }

    fn draw(&mut self, ui: &mut engine::egui::Ui) {
        let Some(State { height, roof, .. }) = self.state.as_mut() else {
            return;
        };
        ui.vertical(|ui| {
            ui.label("Hotel");
            ui.horizontal(|ui| {
                ui.label("Floors:");
                ui.add(egui::Slider::new(height, 1..=32));
                ui.label("Roof Style:");
                egui::ComboBox::from_id_source(0)
                    .selected_text(describe_roof(roof))
                    .show_ui(ui, |ui| {
                        for option in ROOFS {
                            ui.selectable_value(roof, option, describe_roof(&option));
                        }
                    });
            });
        });
    }

    fn update(&self, output: Output) {
        let Some(State {
            building_id,
            height,
            roof,
        }) = self.state
        else {
            return;
        };

        let Some(building) = output.buildings.get_mut(&building_id) else {
            return;
        };

        building.height = height;
        building.roof = roof;
        output.artist.redraw(building_id);
    }
}

fn describe_roof(roof: &Roof) -> &str {
    match roof {
        Roof::Peaked => "Peaked",
        Roof::PeakedRotated => "Peaked Rotated",
        Roof::Flat => "Flat",
    }
}
