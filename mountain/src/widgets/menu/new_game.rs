use engine::egui;

use crate::widgets::ContextWidget;
use crate::{init, Command, NewGameParameters};

pub struct Widget {
    pub create: bool,
    pub cancel: bool,
    pub power: u32,
    pub seed: i32,
    pub tree_line_elevation: f32,
}

pub struct Output<'a> {
    pub command: &'a mut Command,
}

impl Widget {
    pub fn new() -> Widget {
        Widget {
            create: false,
            cancel: false,
            power: 11,
            seed: 0,
            tree_line_elevation: 512.0,
        }
    }
}

impl<'a> ContextWidget<(), Output<'a>> for Widget {
    fn init(&mut self, _: ()) {}

    fn draw(&mut self, ctx: &engine::egui::Context) {
        egui::Window::new("New Game")
            .resizable(false)
            .movable(false)
            .collapsible(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Power");
                        ui.add(egui::Slider::new(&mut self.power, 8..=13).step_by(1.0));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Seed");
                        ui.add(egui::Slider::new(&mut self.seed, 0..=7).step_by(1.0));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Tree Line Height");
                        ui.add(
                            egui::Slider::new(&mut self.tree_line_elevation, 0.0..=512.0)
                                .step_by(1.0),
                        );
                    });
                    ui.horizontal(|ui| {
                        self.create = ui.button("Create").clicked();
                        self.cancel = ui.button("Cancel").clicked();
                    });
                });
            });
    }

    fn update(&mut self, output: Output<'a>) {
        if self.create {
            *output.command = Command::NewGame(NewGameParameters {
                terrain: init::terrain::Parameters {
                    power: self.power,
                    seed: self.seed,
                },
                trees: init::trees::Parameters {
                    power: self.power,
                    tree_line_elevation: self.tree_line_elevation,
                },
            });
        }
    }
}
