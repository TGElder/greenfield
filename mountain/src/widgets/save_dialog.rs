use engine::egui;

use crate::widgets::{save_picker, ContextWidget, UiWidget};

#[derive(Default)]
pub struct Widget {
    save_picker: save_picker::Widget,
}

impl ContextWidget<(), ()> for Widget {
    fn init(&mut self, value: ()) {
        self.save_picker.init(value);
    }

    fn draw(&mut self, ctx: &engine::egui::Context) {
        egui::Window::new("Main Menu")
            .resizable(false)
            .movable(false)
            .collapsible(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.text_edit_singleline(&mut self.save_picker.focus);
                    self.save_picker.draw(ui);
                    ui.horizontal(|ui| {
                        ui.button("Save");
                        ui.button("Back");
                    });
                });
            });
    }

    fn update(&self, value: ()) {
        self.save_picker.update(value);
    }
}
