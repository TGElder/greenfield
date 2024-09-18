use engine::egui;

use crate::systems::log;
use crate::widgets::ContextWidget;

#[derive(Default)]
pub struct Widget {
    messages: Vec<String>,
}

pub struct Input<'a> {
    pub log: &'a log::System,
}

impl<'a> ContextWidget<Input<'a>, ()> for Widget {
    fn init(&mut self, input: Input) {
        self.messages = input
            .log
            .messages()
            .iter()
            .map(|message| message.text.clone())
            .collect();
    }

    fn draw(&mut self, ctx: &engine::egui::Context) {
        egui::Window::new("Toaster")
            .interactable(false)
            .resizable(false)
            .movable(false)
            .title_bar(false)
            .frame(egui::Frame::none())
            .anchor(egui::Align2::CENTER_TOP, egui::vec2(0.0, 16.0))
            .min_width(ctx.screen_rect().width())
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    for message in &self.messages {
                        ui.label(
                            egui::RichText::new(message)
                                .color(egui::Color32::from_rgb(0, 0, 0))
                                .background_color(egui::Color32::from_rgb(255, 255, 255)),
                        );
                    }
                })
            });
    }

    fn _update(&self, _: ()) {}
}
