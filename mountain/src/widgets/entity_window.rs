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
    location: Option<usize>,
    target: Option<usize>,
    hotel: Option<usize>,
    global_target: Option<usize>,
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
            location: None,
            target: None,
            hotel: None,
            global_target: None,
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
        self.location = components.locations.get(&self.entity_id).copied();
        self.target = components.targets.get(&self.entity_id).copied();
        self.global_target = components.global_targets.get(&self.entity_id).copied();
        self.hotel = components
            .skiers
            .get(&self.entity_id)
            .map(|skier| skier.hotel_id);
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
                if let Some(location) = self.location {
                    ui.label(format!("Location: {}", location));
                }
                if let Some(target) = self.target {
                    ui.label(format!("Target: {}", target));
                }
                if let Some(global_target) = self.global_target {
                    ui.label(format!("Global target: {}", global_target));
                }
                if let Some(hotel) = self.hotel {
                    ui.label(format!("Hotel: {}", hotel));
                }
                if let Some(status) = self.open_status.as_mut() {
                    egui::ComboBox::from_id_source(0)
                        .selected_text(open_status_text(status))
                        .show_ui(ui, |ui| {
                            for choice in open_status_choices(status) {
                                ui.selectable_value(status, choice, open_status_text(&choice));
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

static OPEN_STATUS_OPEN: &str = "Open";
static OPEN_STATUS_CLOSED: &str = "Closed";
static OPEN_STATUS_CLOSING: &str = "Closing";

fn open_status_text(status: &open::Status) -> &'static str {
    match status {
        open::Status::Open => OPEN_STATUS_OPEN,
        open::Status::Closing => OPEN_STATUS_CLOSING,
        open::Status::Closed => OPEN_STATUS_CLOSED,
    }
}

fn open_status_choices(status: &open::Status) -> impl Iterator<Item = open::Status> {
    match status {
        open::Status::Open => [open::Status::Open, open::Status::Closing].into_iter(),
        open::Status::Closing => [open::Status::Open, open::Status::Closing].into_iter(),
        open::Status::Closed => [open::Status::Open, open::Status::Closed].into_iter(),
    }
}
