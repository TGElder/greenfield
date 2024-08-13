use engine::egui;
use engine::engine::Engine;
use engine::graphics::Graphics;

use crate::handlers::builder;
use crate::Game;

pub fn run(game: &mut Game, _: &mut dyn Engine, graphics: &mut dyn Graphics) {
    let mut speed = game.components.services.clock.speed();

    let build_mode = game.handlers.builder.mode();
    let mut build_piste_clicked = false;
    let mut build_path_clicked = false;
    let mut build_lift_clicked = false;
    let mut build_gate_clicked = false;
    let mut build_hotel_clicked = false;
    let mut build_hotel_entrance_clicked = false;

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
                        let path = ui.button("🚶").on_hover_text("Path");
                        let lift = ui.button("🚡").on_hover_text("Lift");
                        let gate = ui.button("🚧").on_hover_text("Gate");
                        let hotel = ui.button("🏠").on_hover_text("Hotel");
                        let hotel_entrance = ui.button("🚪").on_hover_text("Hotel Entrance");
                        ui.button("💣").on_hover_text("Remove");

                        build_piste_clicked = piste.clicked();
                        build_path_clicked = path.clicked();
                        build_lift_clicked = lift.clicked();
                        build_gate_clicked = gate.clicked();
                        build_hotel_clicked = hotel.clicked();
                        build_hotel_entrance_clicked = hotel_entrance.clicked();

                        match build_mode {
                            builder::Mode::Piste => {
                                piste.highlight();
                            }
                            builder::Mode::Path => {
                                path.highlight();
                            }
                            builder::Mode::Lift => {
                                lift.highlight();
                            }
                            builder::Mode::Gate => {
                                gate.highlight();
                            }
                            builder::Mode::Building => {
                                hotel.highlight();
                            }
                            builder::Mode::Door => {
                                hotel_entrance.highlight();
                            }
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

    if build_path_clicked {
        game.handlers.selection.clear_selection();
        match build_mode {
            builder::Mode::Path => game.handlers.builder.set_mode(builder::Mode::None),
            _ => game.handlers.builder.set_mode(builder::Mode::Path),
        };
    }

    if build_lift_clicked {
        game.handlers.selection.clear_selection();
        match build_mode {
            builder::Mode::Lift => game.handlers.builder.set_mode(builder::Mode::None),
            _ => game.handlers.builder.set_mode(builder::Mode::Lift),
        };
    }

    if build_gate_clicked {
        game.handlers.selection.clear_selection();
        match build_mode {
            builder::Mode::Gate => game.handlers.builder.set_mode(builder::Mode::None),
            _ => game.handlers.builder.set_mode(builder::Mode::Gate),
        };
    }

    if build_hotel_clicked {
        game.handlers.selection.clear_selection();
        match build_mode {
            builder::Mode::Building => game.handlers.builder.set_mode(builder::Mode::None),
            _ => game.handlers.builder.set_mode(builder::Mode::Building),
        };
    }

    if build_hotel_entrance_clicked {
        game.handlers.selection.clear_selection();
        match build_mode {
            builder::Mode::Door => game.handlers.builder.set_mode(builder::Mode::None),
            _ => game.handlers.builder.set_mode(builder::Mode::Door),
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
