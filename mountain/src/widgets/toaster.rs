use engine::egui;

use crate::systems::log;
use crate::widgets::ContextWidget;

pub struct Widget {
    log: log::System,
}

impl Widget {
    pub fn new(log: log::System) -> Widget {
        Widget { log }
    }
}

impl ContextWidget<(), ()> for Widget {
    fn init(&mut self, _: ()) {
        self.log.run();
    }

    fn draw(&mut self, ctx: &engine::egui::Context) {
        let messages = self
            .log
            .messages()
            .iter()
            .map(|message| message.text.clone())
            .collect::<Vec<_>>();

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
                    for message in messages.iter() {
                        ui.label(
                            egui::RichText::new(message)
                                .color(egui::Color32::from_rgb(0, 0, 0))
                                .background_color(egui::Color32::from_rgb(255, 255, 255)),
                        );
                    }
                })
            });
    }

    fn update(&self, _: ()) {}
}
