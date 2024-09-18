use engine::binding::Binding;
use engine::egui;
use engine::engine::Engine;

use crate::controllers::save;
use crate::systems::messenger;
use crate::widgets::ContextWidget;
use crate::Components;

#[derive(Default)]
pub struct Widget {
    open: bool,
    state: Option<State>,
}

pub struct State {
    quit: bool,
    save: bool,
}

pub struct Input<'a> {
    pub event: &'a engine::events::Event,
    pub binding: &'a Binding,
}

pub struct Output<'a> {
    pub components: &'a mut Components,
    pub engine: &'a mut dyn Engine,
    pub messenger: &'a mut messenger::System,
}

impl<'a> ContextWidget<Input<'a>, Output<'a>> for Widget {
    fn init(&mut self, input: Input) {
        if input.binding.binds_event(input.event) {
            self.open = !self.open;
        }
        if !self.open {
            self.state = None;
            return;
        }
        self.state = Some(State {
            quit: false,
            save: false,
        });
    }

    fn draw(&mut self, ctx: &engine::egui::Context) {
        let Some(state) = self.state.as_mut() else {
            return;
        };

        egui::Window::new("Main Menu")
            .resizable(false)
            .movable(false)
            .collapsible(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    state.save = ui.button("Save").clicked();
                    state.quit = ui.button("Quit").clicked();
                });
            });
    }

    fn update(&self, output: Output) {
        let Some(state) = self.state.as_ref() else {
            return;
        };

        if state.save {
            output.messenger.send("Saving...");
            save::trigger(output.components);
            output.messenger.send("Saved game");
        }

        if state.quit {
            output.engine.shutdown();
        }
    }
}
