use engine::egui;
use engine::engine::Engine;
use engine::graphics::Graphics;

use crate::Game;

pub fn run(game: &mut Game, _: &mut dyn Engine, graphics: &mut dyn Graphics) {
    let mut speed = game.components.services.clock.speed();

    graphics.draw_gui(&mut |ctx| {
        ctx.set_pixels_per_point(1.5);
        egui::TopBottomPanel::bottom("base_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Run");
                    ui.horizontal(|ui| {
                        ui.add(egui::Slider::new(&mut speed, 0.0..=8.0));
                        ui.button("🚦").on_hover_text("Status");
                        ui.button("❓").on_hover_text("Query");
                    });
                });
                ui.separator();
                ui.vertical(|ui| {
                    ui.label("Build");
                    ui.horizontal(|ui| {
                        ui.button("⛷").on_hover_text("Piste");
                        ui.button("🚡").on_hover_text("Lift");
                        ui.button("🚧").on_hover_text("Gates");
                        ui.button("🏠").on_hover_text("Hotel");
                        ui.button("🚪").on_hover_text("Hotel Entrance");
                        ui.button("💣").on_hover_text("Remove");
                    });
                });
                ui.separator();
                ui.vertical(|ui| {
                    ui.label("View");
                    ui.horizontal(|ui| {
                        ui.button("⛷").on_hover_text("Piste");
                        ui.button("🌲").on_hover_text("Trees");
                        ui.button("📊").on_hover_text("Skier Abilities");
                    });
                });
            });
        });
    });

    game.components.services.clock.set_speed(speed);
}
