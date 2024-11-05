use engine::egui;

use crate::widgets::ContextWidget;
use crate::Components;

pub struct EntityWindow {
    entity: usize,
    open: Option<bool>,
}

impl ContextWidget<&Components, &mut Components> for EntityWindow {
    fn init(&mut self, components: &Components) {
        self.open = components.open.get(&self.entity).copied();
    }

    fn draw(&mut self, ctx: &engine::egui::Context) {
        egui::Window::new(format!("Entity {}", self.entity))
            .movable(false)
            .collapsible(false)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    if let Some(open) = self.open.as_mut() {
                        ui.horizontal(|ui| {
                            ui.checkbox(open, "Open");
                        });
                    }
                });
            });
    }

    fn update(&mut self, components: &mut Components) {
        if let Some(open) = self.open {
            components.open.insert(self.entity, open);
        }
    }
}
