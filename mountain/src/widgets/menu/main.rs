use engine::egui;

use crate::widgets::ContextWidget;

use engine::engine::Engine;

use crate::controllers::save;
use crate::systems::messenger;
use crate::Components;

#[derive(Default)]
pub struct Widget {
    pub save_file: Option<String>,
    pub save_as: bool,
    pub save: bool,
    pub load: bool,
    pub quit: bool,
}

pub struct Input<'a> {
    pub save_file: &'a Option<String>,
}

pub struct Output<'a> {
    pub components: &'a mut Components,
    pub engine: &'a mut dyn Engine,
    pub save_directory: &'a str,
    pub save_extension: &'a str,
    pub messenger: &'a mut messenger::System,
}

impl<'a> ContextWidget<Input<'a>, Output<'a>> for Widget {
    fn init(&mut self, input: Input) {
        self.save_file = input.save_file.clone();
    }

    fn draw(&mut self, ctx: &engine::egui::Context) {
        egui::Window::new("Main Menu")
            .resizable(false)
            .movable(false)
            .collapsible(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    self.save_as = ui.button("Save As").clicked();
                    if let Some(save_file) = &self.save_file {
                        self.save = ui.button(format!("Save to \"{}\"", save_file)).clicked();
                    }
                    self.load = ui.button("Load").clicked();
                    self.quit = ui.button("Quit").clicked();
                });
            });
    }

    fn update(&mut self, output: Output<'a>) {
        if self.save {
            if let Some(save_file) = &self.save_file {
                save::trigger(
                    output.components,
                    save_file,
                    output.save_directory,
                    output.save_extension,
                    output.messenger,
                );
            }
        }

        if self.quit {
            output.engine.shutdown();
        }
    }
}
