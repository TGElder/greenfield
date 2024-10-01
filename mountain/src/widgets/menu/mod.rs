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
    pub save_file: &'a Option<String>,
    pub save_directory: &'a str,
    pub save_extension: &'a str,
}

pub struct Output<'a> {
    pub components: &'a mut Components,
    pub engine: &'a mut dyn Engine,
    pub messenger: &'a mut messenger::System,
    pub save_directory: &'a str,
    pub save_extension: &'a str,
    pub file_to_load: &'a mut Option<String>,
}

impl<'a> ContextWidget<Input<'a>, Output<'a>> for Widget {
    fn init(&mut self, input: Input) {
        if input.binding.binds_event(input.event) {
            match &mut self.page {
                Page::Closed => self.page = Page::Main(main::Widget::default()),
                Page::Main(_) => {
                    self.page = Page::Closed;
                }
                _ => {}
            }
        }
        match &mut self.page {
            Page::Main(ref mut widget) => {
                widget.init(main::Input {
                    save_file: input.save_file,
                });
            }
            Page::LoadDialog(ref mut widget) => {
                widget.init(load_dialog::Input {
                    save_directory: input.save_directory,
                    save_extension: input.save_extension,
                });
            }
            _ => {}
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
                    save_directory: output.save_directory,
                    save_extension: output.save_extension,
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
