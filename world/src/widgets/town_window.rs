use commons::geometry::XY;
use engine::egui::{self, RichText};

use crate::model::resource::{Resource, RESOURCES};
use crate::Components;

pub struct Widget {
    town: XY<u32>,
    rows: Vec<(Resource, usize, usize, usize, usize, f32)>,
    is_window_open: bool,
}

impl Widget {
    pub fn new(town: XY<u32>) -> Widget {
        Widget {
            town,
            rows: vec![],
            is_window_open: true,
        }
    }

    pub fn init(&mut self, components: &Components) {
        self.rows = RESOURCES
            .iter()
            .map(|resource| {
                (
                    *resource,
                    components
                        .allocation
                        .iter()
                        .filter(|allocation| allocation.from_market == self.town)
                        .filter(|allocation| allocation.resource == *resource)
                        .count(),
                    components.markets[self.town]
                        .iter()
                        .filter(|source| source.resource == *resource)
                        .count(),
                    components
                        .allocation
                        .iter()
                        .filter(|allocation| allocation.to_market == self.town)
                        .filter(|allocation| allocation.resource == *resource)
                        .count(),
                    components.demand[self.town]
                        .iter()
                        .filter(|source| source.resource == *resource)
                        .count(),
                    components.prices[self.town]
                        .get(resource)
                        .copied()
                        .unwrap_or(f32::NAN),
                )
            })
            .collect::<Vec<_>>();
    }

    pub fn draw(&mut self, ctx: &engine::egui::Context) {
        egui::Window::new(
            RichText::new(format!("Town {}", self.town)).text_style(egui::TextStyle::Body),
        )
        .movable(true)
        .collapsible(false)
        .resizable(false)
        .default_pos((256.0, 256.0))
        .open(&mut self.is_window_open)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Resource");
                    for (resource, _, _, _, _, _) in &self.rows {
                        ui.label(format!("{:?}", resource));
                    }
                });
                ui.vertical(|ui| {
                    ui.label("Supply");
                    for (_, used, supply, _, _, _) in &self.rows {
                        ui.label(format!("{}/{}", used, supply));
                    }
                });
                ui.vertical(|ui| {
                    ui.label("Demand");
                    for (_, _, _, met, demand, _) in &self.rows {
                        ui.label(format!("{}/{}", met, demand));
                    }
                });
                ui.vertical(|ui| {
                    ui.label("Price");
                    for (_, _, _, _, _, price) in &self.rows {
                        ui.label(format!("{}", price));
                    }
                });
            });
        });
    }
}
