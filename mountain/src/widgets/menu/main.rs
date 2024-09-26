use engine::egui;

use crate::widgets::ContextWidget;

use engine::engine::Engine;

use crate::controllers::save;
use crate::systems::messenger;
use crate::Components;

#[derive(Default)]
pub struct Widget {
    pub quit: bool,
    pub load: bool,
    pub save: bool,
}

pub struct Output<'a> {
    pub components: &'a mut Components,
    pub engine: &'a mut dyn Engine,
    pub messenger: &'a mut messenger::System,
    pub save_directory: &'a str,
    pub save_extension: &'a str,
}

impl<'a> ContextWidget<(), Output<'a>> for Widget {
    fn init(&mut self, _: ()) {}

    fn draw(&mut self, ctx: &engine::egui::Context) {
        egui::Window::new("Main Menu")
            .resizable(false)
            .movable(false)
            .collapsible(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    self.load = ui.button("Load").clicked();
                    self.save = ui.button("Save").clicked();
                    self.quit = ui.button("Quit").clicked();
                });
            });
    }

    fn update(&mut self, output: Output<'a>) {
        if self.save {
            output.messenger.send("Saving...");
            save::trigger(
                output.components,
                output.save_directory,
                output.save_extension,
            );
            output.messenger.send("Saved game");
        }

        if self.quit {
            output.engine.shutdown();
        }
    }
}
