use engine::binding::Binding;
use engine::egui::{self};
use engine::engine::Engine;
use engine::events::{Button, KeyboardKey};
use engine::graphics::Graphics;

use crate::services::mode;
use crate::{Bindings, Game};

struct ModeButton {
    icon: &'static str,
    hover_text: &'static str,
    build_mode: mode::Mode,
    panel: Panel,
}

#[derive(PartialEq)]
enum Panel {
    Run,
    Build,
}

const MODE_BUTTONS: [ModeButton; 10] = [
    ModeButton {
        icon: "🚦",
        hover_text: "Open/Close",
        build_mode: mode::Mode::Open,
        panel: Panel::Run,
    },
    ModeButton {
        icon: "❓",
        hover_text: "Query",
        build_mode: mode::Mode::Query,
        panel: Panel::Run,
    },
    ModeButton {
        icon: "⛷",
        hover_text: "Piste",
        build_mode: mode::Mode::Piste,
        panel: Panel::Build,
    },
    ModeButton {
        icon: "✂",
        hover_text: "Erase Piste",
        build_mode: mode::Mode::PisteEraser,
        panel: Panel::Build,
    },
    ModeButton {
        icon: "🚶",
        hover_text: "Path",
        build_mode: mode::Mode::Path,
        panel: Panel::Build,
    },
    ModeButton {
        icon: "🚡",
        hover_text: "Lift",
        build_mode: mode::Mode::Lift,
        panel: Panel::Build,
    },
    ModeButton {
        icon: "🚧",
        hover_text: "Gate",
        build_mode: mode::Mode::Gate,
        panel: Panel::Build,
    },
    ModeButton {
        icon: "🏠",
        hover_text: "Hotel",
        build_mode: mode::Mode::Building,
        panel: Panel::Build,
    },
    ModeButton {
        icon: "🚪",
        hover_text: "Entrance",
        build_mode: mode::Mode::Door,
        panel: Panel::Build,
    },
    ModeButton {
        icon: "💣",
        hover_text: "Demolish",
        build_mode: mode::Mode::Demolish,
        panel: Panel::Build,
    },
];

pub trait View<T> {
    fn init(&mut self, value: &T);
    fn draw(&mut self, ui: &mut egui::Ui);
    fn update(&self, value: &mut T);
}

pub fn run(game: &mut Game, _: &mut dyn Engine, graphics: &mut dyn Graphics) {
    let mut speed = game.components.services.clock.speed();

    let build_mode = game.components.services.mode.mode();
    let mut mode_button_clicked = [false; MODE_BUTTONS.len()];
    let mut draw_mode_buttons = |ui: &mut egui::Ui, panel: Panel| {
        for (i, config) in MODE_BUTTONS
            .iter()
            .enumerate()
            .filter(|(_, config)| config.panel == panel)
        {
            let hover_text = mode_button_hover_text(&game.bindings, config);
            let button = ui.button(config.icon).on_hover_text(hover_text);
            mode_button_clicked[i] = button.clicked();
            if build_mode == config.build_mode {
                button.highlight();
            }
        }
    };

    let mut view_pistes_clicked = false;
    let mut view_trees_clicked = false;
    let mut view_skier_abilities_clicked = false;

    let mut mode_view = game.components.services.mode.view();
    mode_view.init(game);

    graphics.draw_gui(&mut |ctx| {
        ctx.set_pixels_per_point(1.5);
        egui::TopBottomPanel::bottom("base_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Run");
                    ui.horizontal(|ui| {
                        ui.add(egui::Slider::new(&mut speed, 0.0..=8.0));
                        draw_mode_buttons(ui, Panel::Run);
                    });
                });
                ui.separator();
                ui.vertical(|ui| {
                    ui.label("Build");
                    ui.horizontal(|ui| {
                        draw_mode_buttons(ui, Panel::Build);
                    });
                });
                ui.separator();
                ui.vertical(|ui| {
                    ui.label("View");
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
                ui.separator();
                mode_view.draw(ui);
            });
        });
    });

    mode_view.update(game);

    game.components.services.clock.set_speed(speed);

    for (i, &clicked) in mode_button_clicked.iter().enumerate() {
        if clicked {
            game.handlers.selection.clear_selection();
            let config = &MODE_BUTTONS[i];
            if build_mode == config.build_mode {
                game.components.services.mode.set_mode(mode::Mode::None);
            } else {
                game.components.services.mode.set_mode(config.build_mode);
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

fn mode_button_hover_text(bindings: &Bindings, mode_button: &ModeButton) -> String {
    let Some(Binding::Single { button, .. }) = bindings.mode.get(&mode_button.build_mode) else {
        return mode_button.hover_text.to_string();
    };
    format!("{} ({})", mode_button.hover_text, describe_button(button))
}

fn describe_button(button: &Button) -> &str {
    match button {
        Button::Keyboard(key) => match key {
            KeyboardKey::String(str) => str,
            KeyboardKey::Backspace => "Backspace",
            KeyboardKey::Escape => "Esc",
            KeyboardKey::Unknown => "?",
        },
        Button::Mouse(button) => match button {
            engine::events::MouseButton::Left => "Left Click",
            engine::events::MouseButton::Middle => "Middle Click",
            engine::events::MouseButton::Right => "Right Click",
            engine::events::MouseButton::WheelUp => "Mouse Wheel Up",
            engine::events::MouseButton::WheelDown => "Mouse Wheel Down",
            engine::events::MouseButton::Unknown => "?",
        },
    }
}
