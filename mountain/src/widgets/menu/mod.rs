mod load_dialog;
mod main;

use engine::binding::Binding;
use engine::engine::Engine;

use crate::systems::messenger;
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
    Main(main::Widget),
    LoadDialog(load_dialog::Widget),
}

pub struct Input<'a> {
    pub event: &'a engine::events::Event,
    pub binding: &'a Binding,
}

pub struct Output<'a> {
    pub components: &'a mut Components,
    pub engine: &'a mut dyn Engine,
    pub messenger: &'a mut messenger::System,
    pub load: &'a mut Option<String>,
}

impl<'a> ContextWidget<Input<'a>, Output<'a>> for Widget {
    fn init(&mut self, input: Input) {
        match &mut self.mode {
            Mode::Closed => {
                if input.binding.binds_event(input.event) {
                    let mut widget = main::Widget::default();
                    widget.init(());
                    self.mode = Mode::Main(widget);
                }
            }
            Mode::Main(ref mut widget) => {
                if input.binding.binds_event(input.event) {
                    self.mode = Mode::Closed;
                } else {
                    widget.init(());
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
            Mode::Main(ref mut widget) => {
                widget.draw(ctx);
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
            Mode::Main(widget) => {
                widget.update(main::Output {
                    components: output.components,
                    engine: output.engine,
                    messenger: output.messenger,
                });
                if widget.load {
                    new_mode = Some(Mode::LoadDialog(load_dialog::Widget::default()));
                }
            }
            Mode::LoadDialog(ref mut widget) => {
                widget.update(load_dialog::Output { load: output.load });
                if widget.cancel {
                    new_mode = Some(Mode::Main(main::Widget::default()));
                }
            }
        }

        if let Some(new_mode) = new_mode {
            self.mode = new_mode;
        }
    }
}
