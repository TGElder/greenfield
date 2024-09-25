use engine::egui;

use crate::widgets::{save_picker, ContextWidget, UiWidget};

#[derive(Default)]
pub struct Widget {
    save_picker: save_picker::Widget,
    load: bool,
    pub cancel: bool,
}

impl Widget {
    pub fn new() -> Self {
        Widget {
            save_picker: save_picker::Widget::default(),
            load: false,
            cancel: false,
        }
    }
}

pub struct Output<'a> {
    pub load: &'a mut Option<String>,
}

impl<'a> ContextWidget<(), Output<'a>> for Widget {
    fn init(&mut self, value: ()) {
        self.save_picker.init(value);
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

    fn update(&mut self, value: Output<'a>) {
        self.save_picker.update(());
        if self.load {
            *value.load = Some(format!("saves/{}.save", self.save_picker.focus));
        }
    }
}
