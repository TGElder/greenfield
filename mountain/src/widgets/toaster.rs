use engine::egui;

use crate::systems::log;
use crate::widgets::ContextWidget;

pub struct Widget {
    toast: Option<String>,
}

pub struct Input<'a> {
    pub log: &'a log::System,
}

impl<'a> ContextWidget<Input<'a>, ()> for Widget {
    fn init(input: Input) -> Self {
        let toast = input
            .log
            .messages()
            .first()
            .map(|message| message.text.clone());
        Widget { toast }
    }

    fn draw(&mut self, ctx: &engine::egui::Context) {
        let Some(ref toast) = self.toast else {
            return;
        };
        egui::Window::new("Toast")
            .interactable(false)
            .resizable(false)
            .movable(false)
            .title_bar(false)
            .frame(egui::Frame::none())
            .anchor(egui::Align2::CENTER_TOP, egui::vec2(0.0, 16.0))
            .show(ctx, |ui| {
                ui.label(
                    egui::RichText::new(toast)
                        .color(egui::Color32::from_rgb(0, 0, 0))
                        .background_color(egui::Color32::from_rgb(255, 0, 0)),
                )
            });
    }

    fn _update(&self, _: ()) {}
}
