use engine::egui;

use crate::widgets::ContextWidget;
use crate::Components;

pub struct EntityWindow {
    id: usize,
    open: Option<bool>,
}

impl EntityWindow {
    pub fn new(id: usize) -> EntityWindow {
        EntityWindow { id, open: None }
    }
}

impl ContextWidget<&Components, &mut Components> for EntityWindow {
    fn init(&mut self, components: &Components) {
        self.open = components.open.get(&self.id).copied();
    }

    fn draw(&mut self, ctx: &engine::egui::Context) {
        egui::Window::new(format!("Entity {}", self.id))
            .movable(true)
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
            components.open.insert(self.id, open);
        }
    }
}
