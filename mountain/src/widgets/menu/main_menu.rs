use engine::binding::Binding;
use engine::egui;
use engine::engine::Engine;

use crate::controllers::save;
use crate::systems::messenger;
use crate::widgets::menu::load_dialog;
use crate::widgets::ContextWidget;
use crate::Components;

#[derive(Default)]
pub struct Widget {
    mode: Mode,
}

#[derive(Default)]
pub enum Mode {
    #[default]
    Closed,
    MainMenu(State),
    LoadDialog(load_dialog::Widget),
}

#[derive(Default)]
pub struct State {
    quit: bool,
    load: bool,
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
    pub load_dialog: load_dialog::Output<'a>,
}

impl<'a> ContextWidget<Input<'a>, Output<'a>> for Widget {
    fn init(&mut self, input: Input) {
        match &mut self.mode {
            Mode::Closed => {
                if input.binding.binds_event(input.event) {
                    self.mode = Mode::MainMenu(State::default());
                }
            }
            Mode::MainMenu(..) => {
                if input.binding.binds_event(input.event) {
                    self.mode = Mode::Closed;
                }
            }
            Mode::LoadDialog(ref mut widget) => {
                widget.init(());
            }
        }
    }

    fn draw(&mut self, ctx: &engine::egui::Context) {
        match &mut self.mode {
            Mode::Closed => {}
            Mode::MainMenu(state) => {
                egui::Window::new("Main Menu")
                    .resizable(false)
                    .movable(false)
                    .collapsible(false)
                    .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                    .show(ctx, |ui| {
                        ui.vertical_centered(|ui| {
                            state.load = ui.button("Load").clicked();
                            state.save = ui.button("Save").clicked();
                            state.quit = ui.button("Quit").clicked();
                        });
                    });
            }
            Mode::LoadDialog(ref mut widget) => {
                widget.draw(ctx);
            }
        }
    }

    fn update(&mut self, output: Output) {
        let mut new_mode = None;
        match &mut self.mode {
            Mode::Closed => {}
            Mode::MainMenu(state) => {
                if state.load {
                    new_mode = Some(Mode::LoadDialog(load_dialog::Widget::default()));
                }

                if state.save {
                    output.messenger.send("Saving...");
                    save::trigger(output.components);
                    output.messenger.send("Saved game");
                }

                if state.quit {
                    output.engine.shutdown();
                }
            }
            Mode::LoadDialog(ref mut widget) => {
                widget.update(output.load_dialog);
                if widget.cancel {
                    new_mode = Some(Mode::MainMenu(State::default()));
                }
            }
        }

        if let Some(new_mode) = new_mode {
            self.mode = new_mode;
        }
    }
}
