use std::collections::HashMap;

use engine::egui;

use crate::controllers::building_builder;
use crate::model::building::Building;
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
        });
    }

    fn draw(&mut self, ui: &mut engine::egui::Ui) {
        if let Some(State { height, .. }) = self.state.as_mut() {
            ui.vertical(|ui| {
                ui.label("Building");
                ui.horizontal(|ui| {
                    ui.add(egui::Slider::new(height, 1..=32));
                });
            });
        }
    }

    fn update(&self, output: Output) {
        let Some(State {
            building_id,
            height,
        }) = self.state
        else {
            return;
        };

        let Some(building) = output.buildings.get_mut(&building_id) else {
            return;
        };

        building.height = height;
        output.artist.redraw(building_id);
    }
}
