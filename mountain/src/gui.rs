use engine::egui;
use engine::engine::Engine;
use engine::graphics::Graphics;

use crate::handlers::builder;
use crate::Game;

pub fn run(game: &mut Game, _: &mut dyn Engine, graphics: &mut dyn Graphics) {
    let mut speed = game.components.services.clock.speed();

    let build_mode = game.handlers.builder.mode();
    let mut build_piste_clicked = false;

    let mut view_pistes_clicked = false;
    let mut view_trees_clicked = false;
    let mut view_skier_abilities_clicked = false;

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
                        let piste = ui.button("⛷").on_hover_text("Piste");
                        ui.button("🚶").on_hover_text("Path");
                        ui.button("🚡").on_hover_text("Lift");
                        ui.button("🚧").on_hover_text("Gates");
                        ui.button("🏠").on_hover_text("Hotel");
                        ui.button("🚪").on_hover_text("Hotel Entrance");
                        ui.button("💣").on_hover_text("Remove");

                        build_piste_clicked = piste.clicked();

                        match build_mode {
                            builder::Mode::Piste => {
                                piste.highlight();
                            }
                            builder::Mode::Path => todo!(),
                            builder::Mode::Lift => todo!(),
                            builder::Mode::Gates => todo!(),
                            builder::Mode::Building => todo!(),
                            builder::Mode::Door => todo!(),
                            builder::Mode::None => (),
                        }
                    });
                });
                ui.separator();
                ui.vertical(|ui| {
                    ui.label("View").highlight();
                    ui.horizontal(|ui| {
                        let pistes = ui.button("⛷").on_hover_text("Pistes");
                        view_pistes_clicked = pistes.clicked();
                        if game.systems.terrain_artist.is_showing_pistes() {
                            pistes.highlight();
                        }

                        let trees = ui.button("🌲").on_hover_text("Trees");
                        view_trees_clicked = trees.clicked();
                        if game.systems.tree_artist.is_visible() {
                            trees.highlight();
                        }

                        let skier_abilities = ui.button("📊").on_hover_text("Skier Abilities");
                        view_skier_abilities_clicked = skier_abilities.clicked();
                        if game.systems.skier_colors.is_showing_ability() {
                            skier_abilities.highlight();
                        }
                    });
                });
            });
        });
    });

    game.components.services.clock.set_speed(speed);

    if build_piste_clicked {
        game.handlers.selection.clear_selection();
        match build_mode {
            builder::Mode::Piste => game.handlers.builder.set_mode(builder::Mode::None),
            _ => game.handlers.builder.set_mode(builder::Mode::Piste),
        };
    }

    if view_pistes_clicked {
        game.systems.terrain_artist.toggle_show_pistes();
        game.systems.terrain_artist.update_whole_overlay();
    }

    if view_trees_clicked {
        game.systems.tree_artist.toggle_visible(graphics);
    }

    if view_skier_abilities_clicked {
        game.systems.skier_colors.toggle_show_ability();
    }
}
