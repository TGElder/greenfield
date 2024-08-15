use engine::egui;
use engine::engine::Engine;
use engine::graphics::Graphics;

use crate::handlers::builder;
use crate::Game;

struct BuildButton {
    icon: &'static str,
    hover_text: &'static str,
    build_mode: builder::Mode,
}

const BUILD_BUTTONS: [BuildButton; 7] = [
    BuildButton {
        icon: "⛷",
        hover_text: "Piste",
        build_mode: builder::Mode::Piste,
    },
    BuildButton {
        icon: "🚶",
        hover_text: "Path",
        build_mode: builder::Mode::Path,
    },
    BuildButton {
        icon: "🚡",
        hover_text: "Lift",
        build_mode: builder::Mode::Lift,
    },
    BuildButton {
        icon: "🚧",
        hover_text: "Gate",
        build_mode: builder::Mode::Gate,
    },
    BuildButton {
        icon: "🏠",
        hover_text: "Hotel",
        build_mode: builder::Mode::Building,
    },
    BuildButton {
        icon: "🚪",
        hover_text: "Entrance",
        build_mode: builder::Mode::Door,
    },
    BuildButton {
        icon: "💣",
        hover_text: "Demolish",
        build_mode: builder::Mode::Demolish,
    },
];

pub fn run(game: &mut Game, _: &mut dyn Engine, graphics: &mut dyn Graphics) {
    let mut speed = game.components.services.clock.speed();

    let build_mode = game.handlers.builder.mode();
    let mut build_button_clicked = [false; BUILD_BUTTONS.len()];

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
                        for (i, config) in BUILD_BUTTONS.iter().enumerate() {
                            let button = ui.button(config.icon).on_hover_text(config.hover_text);
                            build_button_clicked[i] = button.clicked();
                            if build_mode == config.build_mode {
                                button.highlight();
                            }
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

    for (i, &clicked) in build_button_clicked.iter().enumerate() {
        if clicked {
            game.handlers.selection.clear_selection();
            let config = &BUILD_BUTTONS[i];
            if build_mode == config.build_mode {
                game.handlers.builder.set_mode(builder::Mode::None);
            } else {
                game.handlers.builder.set_mode(config.build_mode);
            };
        }
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
