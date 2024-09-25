mod load_dialog;
mod main;

use engine::binding::Binding;
use engine::engine::Engine;

use crate::systems::messenger;
use crate::widgets::ContextWidget;
use crate::Components;

#[derive(Default)]
pub struct Widget {
    page: Page,
}

#[derive(Default)]
pub enum Page {
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
    pub file_to_load: &'a mut Option<String>,
}

impl<'a> ContextWidget<Input<'a>, Output<'a>> for Widget {
    fn init(&mut self, input: Input) {
        match &mut self.page {
            Page::Closed => {
                if input.binding.binds_event(input.event) {
                    let mut widget = main::Widget::default();
                    widget.init(());
                    self.page = Page::Main(widget);
                }
            }
            Page::Main(ref mut widget) => {
                if input.binding.binds_event(input.event) {
                    self.page = Page::Closed;
                } else {
                    widget.init(());
                }
            }
            Page::LoadDialog(ref mut widget) => {
                widget.init(());
            }
        }
    }

    fn draw(&mut self, ctx: &engine::egui::Context) {
        match &mut self.page {
            Page::Closed => {}
            Page::Main(ref mut widget) => {
                widget.draw(ctx);
            }
            Page::LoadDialog(ref mut widget) => {
                widget.draw(ctx);
            }
        }
    }

    fn update(&mut self, output: Output) {
        let mut new_page = None;
        match &mut self.page {
            Page::Closed => {}
            Page::Main(widget) => {
                widget.update(main::Output {
                    components: output.components,
                    engine: output.engine,
                    messenger: output.messenger,
                });
                if widget.load {
                    new_page = Some(Page::LoadDialog(load_dialog::Widget::default()));
                }
            }
            Page::LoadDialog(ref mut widget) => {
                widget.update(load_dialog::Output {
                    file_to_load: output.file_to_load,
                });
                if widget.cancel {
                    new_page = Some(Page::Main(main::Widget::default()));
                }
            }
        }

        if let Some(new_mode) = new_page {
            self.page = new_mode;
        }
    }
}
