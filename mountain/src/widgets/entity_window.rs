use commons::geometry::XY;
use engine::egui::{self, RichText};

use crate::gui;
use crate::widgets::ContextWidget;
use crate::Components;

pub struct EntityWindow {
    id: usize,
    mouse_pos: XY<u32>,
    open: Option<bool>,
    is_open: bool,
}

impl EntityWindow {
    pub fn new(id: usize, mouse_pos: XY<u32>) -> EntityWindow {
        EntityWindow {
            id,
            mouse_pos,
            open: None,
            is_open: true,
        }
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }
}

impl ContextWidget<&Components, &mut Components> for EntityWindow {
    fn init(&mut self, components: &Components) {
        self.open = components.open.get(&self.id).copied();
    }

    fn draw(&mut self, ctx: &engine::egui::Context) {
        egui::Window::new(
            RichText::new(format!("Entity {}", self.id)).text_style(egui::TextStyle::Body),
        )
        .default_pos((
            self.mouse_pos.x as f32 / gui::PIXELS_PER_POINT,
            self.mouse_pos.y as f32 / gui::PIXELS_PER_POINT,
        ))
        .movable(true)
        .collapsible(false)
        .resizable(false)
        .open(&mut self.is_open)
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
