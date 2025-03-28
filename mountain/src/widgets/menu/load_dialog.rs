use engine::egui;

use crate::widgets::{save_picker, ContextWidget, UiWidget};
use crate::Command;

#[derive(Default)]
pub struct Widget {
    pub save_picker: save_picker::Widget,
    pub load: bool,
    pub cancel: bool,
}

pub struct Input<'a> {
    pub save_directory: &'a str,
    pub save_extension: &'a str,
}

pub struct Output<'a> {
    pub command: &'a mut Command,
}

impl<'a> ContextWidget<Input<'a>, Output<'a>> for Widget {
    fn init(&mut self, input: Input) {
        self.save_picker.init(save_picker::Input {
            directory: input.save_directory,
            extension: input.save_extension,
        });
    }

    fn draw(&mut self, ctx: &engine::egui::Context) {
        egui::Window::new("Load")
            .resizable(false)
            .movable(false)
            .collapsible(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    self.save_picker.draw(ui);
                    ui.horizontal(|ui| {
                        self.load = ui.button("Load").clicked();
                        self.cancel = ui.button("Cancel").clicked();
                    });
                });
            });
    }

    fn update(&mut self, output: Output<'a>) {
        self.save_picker.update(());
        if self.load {
            *output.command = Command::LoadGame(self.save_picker.focus().to_string());
        }
    }
}
