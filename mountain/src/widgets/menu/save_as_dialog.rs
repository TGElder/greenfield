use engine::egui;

use crate::controllers::save;
use crate::systems::messenger;
use crate::widgets::ContextWidget;
use crate::Components;

pub struct Widget {
    pub save_file: String,
    pub save: bool,
    pub cancel: bool,
}

pub struct Output<'a> {
    pub components: &'a mut Components,
    pub save_file: &'a mut Option<String>,
    pub save_directory: &'a str,
    pub save_extension: &'a str,
    pub messenger: &'a mut messenger::System,
}

impl Widget {
    pub fn new(save_file: String) -> Widget {
        Widget {
            save_file,
            save: false,
            cancel: false,
        }
    }
}

impl<'a> ContextWidget<(), Output<'a>> for Widget {
    fn init(&mut self, _: ()) {}

    fn draw(&mut self, ctx: &engine::egui::Context) {
        egui::Window::new("Save As")
            .resizable(false)
            .movable(false)
            .collapsible(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.text_edit_singleline(&mut self.save_file).request_focus();
                    ui.horizontal(|ui| {
                        self.save = ui.button("Save").clicked();
                        self.cancel = ui.button("Cancel").clicked();
                    });
                });
            });
    }

    fn update(&mut self, output: Output<'a>) {
        if self.save {
            *output.save_file = Some(self.save_file.clone());
            save::trigger(
                output.components,
                &self.save_file,
                output.save_directory,
                output.save_extension,
                output.messenger,
            );
        }
    }
}
