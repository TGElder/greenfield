use std::collections::HashMap;

use engine::binding::Binding;
use engine::egui;
use engine::engine::Engine;
use engine::events::{Button, ButtonState, KeyboardKey};
use engine::graphics::Graphics;

use crate::services::mode;
use crate::widgets::entity_window::EntityWindow;
use crate::widgets::{
    self, building_editor, lift_editor, menu, piste_build_mode, toaster, ContextWidget, UiWidget,
};
use crate::{Bindings, Game};

pub const PIXELS_PER_POINT: f32 = 1.5;

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

const MODE_BUTTONS: [ModeButton; 9] = [
    ModeButton {
        icon: "❓",
        hover_text: "Info",
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

pub struct Widgets {
    pub building_editor: building_editor::Widget,
    pub lift_editor: lift_editor::Widget,
    pub piste_build_mode: piste_build_mode::Widget,
    pub menu: menu::Widget,
    pub toaster: toaster::Widget,
    pub windows: HashMap<usize, EntityWindow>,
}

pub fn run(
    game: &mut Game,
    event: &engine::events::Event,
    engine: &mut dyn Engine,
    graphics: &mut dyn Graphics,
) {
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

    for window in game.widgets.windows.values_mut() {
        window.init(&game.components);
    }

    game.widgets.menu.init(menu::Input {
        event,
        binding: &game.bindings.main_menu,
        save_file: &game.config.save_file,
        save_directory: &game.config.save_directory,
        save_extension: &game.config.save_extension,
    });
    game.widgets.building_editor.init(building_editor::Input {
        mode: build_mode,
        builder: &game.controllers.building_builder,
        buildings: &game.components.buildings,
    });
    game.widgets.lift_editor.init(lift_editor::Input {
        mode: build_mode,
        lift_builder: &game.controllers.lift_builder,
        lift_buildings: &game.components.lift_buildings,
    });
    game.widgets.piste_build_mode.init(piste_build_mode::Input {
        mode: build_mode,
        bindings: &game.bindings.piste_mode,
        piste_eraser: &game.controllers.piste_eraser,
    });
    game.widgets.toaster.init(());
    graphics.draw_gui(&mut |ctx| {
        ctx.set_pixels_per_point(PIXELS_PER_POINT);
        game.widgets.menu.draw(ctx);
        game.widgets.toaster.draw(ctx);
        egui::TopBottomPanel::bottom("base_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Run");
                    ui.horizontal(|ui| {
                        ui.add(egui::Slider::new(&mut speed, 0.0..=50.0).step_by(1.0));
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
                        let pistes = ui.button("⛷").on_hover_text(format!(
                            "Pistes ({})",
                            describe_binding(&game.bindings.view.toggle_pistes)
                        ));
                        view_pistes_clicked = pistes.clicked();
                        if game.systems.terrain_artist.is_showing_pistes() {
                            pistes.highlight();
                        }

                        let trees = ui.button("🌲").on_hover_text(format!(
                            "Trees ({})",
                            describe_binding(&game.bindings.view.toggle_trees)
                        ));
                        view_trees_clicked = trees.clicked();
                        if game.systems.tree_artist.is_visible() {
                            trees.highlight();
                        }

                        let skier_abilities = ui.button("📊").on_hover_text(format!(
                            "Skier Abilities ({})",
                            describe_binding(&game.bindings.view.toggle_skier_ability)
                        ));
                        view_skier_abilities_clicked = skier_abilities.clicked();
                        if game.systems.skier_colors.is_showing_ability() {
                            skier_abilities.highlight();
                        }
                    });
                });
                ui.separator();
                game.widgets.building_editor.draw(ui);
                game.widgets.lift_editor.draw(ui);
                game.widgets.piste_build_mode.draw(ui);
            });
        });
        for window in game.widgets.windows.values_mut() {
            window.draw(ctx);
        }
    });

    game.widgets.menu.update(menu::Output {
        components: &mut game.components,
        engine,
        messenger: &mut game.systems.messenger,
        save_file: &mut game.config.save_file,
        save_directory: &game.config.save_directory,
        save_extension: &game.config.save_extension,
        command: &mut game.command,
    });

    game.widgets
        .building_editor
        .update(building_editor::Output {
            buildings: &mut game.components.buildings,
            artist: &mut game.systems.building_artist,
        });
    game.widgets.lift_editor.update(lift_editor::Output {
        lift_buildings: &mut game.components.lift_buildings,
        lift_building_artist: &mut game.systems.lift_building_artist,
    });
    game.widgets
        .piste_build_mode
        .update(piste_build_mode::Output {
            path_builder: &mut game.controllers.path_builder,
            piste_builder: &mut game.controllers.piste_builder,
            piste_eraser: &mut game.controllers.piste_eraser,
        });

    game.widgets.windows.retain(|_, window| {
        window.update(widgets::entity_window::Output {
            components: &mut game.components,
            systems: &mut game.systems,
        });
        window.is_open()
    });

    game.components.services.clock.set_speed(speed);

    for (i, &clicked) in mode_button_clicked.iter().enumerate() {
        if clicked {
            let config = &MODE_BUTTONS[i];
            game.components
                .services
                .mode
                .set_mode(config.build_mode, &mut game.components.selection);
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
    let Some(binding) = bindings.mode.get(&mode_button.build_mode) else {
        return mode_button.hover_text.to_string();
    };
    format!("{} ({})", mode_button.hover_text, describe_binding(binding))
}

pub fn describe_binding(binding: &Binding) -> String {
    match binding {
        Binding::Single { button, state } => match state {
            ButtonState::Pressed => describe_button(button).to_string(),
            ButtonState::Released => format!("{} (Key Up)", describe_button(button)),
        },
        Binding::Multi(vec) => vec.first().map(describe_binding).unwrap_or_default(),
    }
}

fn describe_button(button: &Button) -> &str {
    match button {
        Button::Keyboard(key) => match key {
            KeyboardKey::String(str) => str,
            KeyboardKey::Backspace => "Backspace",
            KeyboardKey::Escape => "Esc",
            KeyboardKey::Shift => "Shift",
            KeyboardKey::Ctrl => "Ctrl",
            KeyboardKey::Alt => "Alt",
            KeyboardKey::AltGr => "AltGr",
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
