use commons::geometry::XY;
use engine::egui::{self, RichText};

use crate::gui;
use crate::widgets::ContextWidget;
use crate::Components;

pub struct EntityWindow {
    entity_id: usize,
    mouse_position: XY<u32>,
    is_entity_open: Option<bool>,
    is_window_open: bool,
}

impl EntityWindow {
    pub fn new(id: usize, mouse_pos: XY<u32>) -> EntityWindow {
        EntityWindow {
            entity_id: id,
            mouse_position: mouse_pos,
            is_entity_open: None,
            is_window_open: true,
        }
    }

    pub fn is_open(&self) -> bool {
        self.is_window_open
    }
}

impl ContextWidget<&Components, &mut Components> for EntityWindow {
    fn init(&mut self, components: &Components) {
        self.is_entity_open = components.open.get(&self.entity_id).copied();
    }

    fn draw(&mut self, ctx: &engine::egui::Context) {
        egui::Window::new(
            RichText::new(format!("Entity {}", self.entity_id)).text_style(egui::TextStyle::Body),
        )
        .movable(true)
        .collapsible(false)
        .resizable(false)
        .default_pos((
            self.mouse_position.x as f32 / gui::PIXELS_PER_POINT,
            self.mouse_position.y as f32 / gui::PIXELS_PER_POINT,
        ))
        .open(&mut self.is_window_open)
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                if let Some(open) = self.is_entity_open.as_mut() {
                    ui.horizontal(|ui| {
                        ui.checkbox(open, "Open");
                    });
                }
            });
        });
    }

    fn update(&mut self, components: &mut Components) {
        if let Some(open) = self.is_entity_open {
            components.open.insert(self.entity_id, open);
        }
    }
}