use commons::geometry::XY;
use engine::egui::{self, RichText};

use crate::model::open;
use crate::utils::opener;
use crate::widgets::ContextWidget;
use crate::Components;
use crate::{gui, Systems};

pub struct EntityWindow {
    entity_id: usize,
    mouse_position: XY<u32>,
    open_status: Option<open::Status>,
    is_window_open: bool,
}

pub struct Output<'a> {
    pub components: &'a mut Components,
    pub systems: &'a mut Systems,
}

impl EntityWindow {
    pub fn new(id: usize, mouse_pos: XY<u32>) -> EntityWindow {
        EntityWindow {
            entity_id: id,
            mouse_position: mouse_pos,
            open_status: None,
            is_window_open: true,
        }
    }

    pub fn is_open(&self) -> bool {
        self.is_window_open
    }
}

impl ContextWidget<&Components, Output<'_>> for EntityWindow {
    fn init(&mut self, components: &Components) {
        self.open_status = components.open.get(&self.entity_id).copied();
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
                if let Some(status) = self.open_status.as_mut() {
                    egui::ComboBox::from_id_source(0)
                        .selected_text(text(status))
                        .show_ui(ui, |ui| {
                            for choice in choices(status) {
                                ui.selectable_value(status, choice, text(&choice));
                            }
                        });
                }
            });
        });
    }

    fn update(&mut self, output: Output) {
        if output.components.open.get(&self.entity_id) != self.open_status.as_ref() {
            if let Some(open_status) = self.open_status {
                opener::set_open_status(
                    &self.entity_id,
                    open_status,
                    output.components,
                    output.systems,
                );
            }
        }
    }
}

static OPEN: &str = "Open";
static CLOSED: &str = "Closed";
static CLOSING: &str = "Closing";

fn text(status: &open::Status) -> &'static str {
    match status {
        open::Status::Open => OPEN,
        open::Status::Closing => CLOSING,
        open::Status::Closed => CLOSED,
    }
}

fn choices(status: &open::Status) -> impl Iterator<Item = open::Status> {
    match status {
        open::Status::Open => [open::Status::Open, open::Status::Closing].into_iter(),
        open::Status::Closing => [open::Status::Open, open::Status::Closing].into_iter(),
        open::Status::Closed => [open::Status::Open, open::Status::Closed].into_iter(),
    }
}
