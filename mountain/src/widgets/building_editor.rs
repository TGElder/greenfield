use std::collections::HashMap;

use engine::egui;

use crate::controllers::building_builder;
use crate::model::building::Building;
use crate::services;
use crate::systems::building_artist;
use crate::widgets::Widget;

#[derive(Default)]
pub struct ControllerView {
    building_id: Option<usize>,
    height: Option<u32>,
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

impl<'a> Widget<Input<'a>, Output<'a>> for ControllerView {
    fn init(&mut self, input: Input) {
        if input.mode != services::mode::Mode::Building {
            return;
        }
        if let &building_builder::State::Editing { building_id } = input.builder.state() {
            self.building_id = Some(building_id);
            self.height = input
                .buildings
                .get(&building_id)
                .map(|building| building.height);
        }
    }

    fn draw(&mut self, ui: &mut engine::egui::Ui) {
        if let Some(height) = self.height.as_mut() {
            ui.vertical(|ui| {
                ui.label("Building");
                ui.horizontal(|ui| {
                    ui.add(egui::Slider::new(height, 0..=32));
                });
            });
        }
    }

    fn update(&self, output: Output) {
        let Some(building_id) = self.building_id else {
            return;
        };
        let Some(height) = self.height else {
            return;
        };
        if let Some(building) = output.buildings.get_mut(&building_id) {
            building.height = height;
            output.artist.redraw(building_id);
        }
    }
}
