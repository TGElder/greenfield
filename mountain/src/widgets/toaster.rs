use std::time::Instant;

use engine::egui;

use crate::gui::{self, Toast};
use crate::widgets::ContextWidget;

pub struct Widget {
    toast: Option<gui::Toast>,
}

pub struct Input {
    pub toast: Option<gui::Toast>,
}

impl ContextWidget<Input, ()> for Widget {
    fn init(value: Input) -> Self {
        Widget { toast: value.toast }
    }

    fn draw(&mut self, ctx: &engine::egui::Context) {
        let toast_message = self
            .toast
            .as_ref()
            .filter(|Toast { expiry, .. }| *expiry > Instant::now())
            .map(|Toast { message, .. }| message);
        let Some(toast_message) = toast_message else {
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
                    egui::RichText::new(toast_message)
                        .color(egui::Color32::from_rgb(0, 0, 0))
                        .background_color(egui::Color32::from_rgb(255, 0, 0)),
                )
            });
    }

    fn _update(&self, _: ()) {}
}
