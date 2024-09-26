use crate::controllers::{piste_builder, piste_eraser};
use crate::gui::describe_binding;
use crate::handlers::piste_build_mode;
use crate::services;
use crate::widgets;

#[derive(Default)]
pub struct Widget {
    state: Option<State>,
}

struct State {
    build: Button,
    erase: Button,
}

struct Button {
    hover_text: Option<String>,
    highlighted: bool,
    clicked: bool,
}

pub struct Input<'a> {
    pub mode: services::mode::Mode,
    pub bindings: &'a piste_build_mode::Bindings,
    pub piste_eraser: &'a piste_eraser::Controller,
}

pub struct Output<'a> {
    pub path_builder: &'a mut piste_builder::Controller,
    pub piste_builder: &'a mut piste_builder::Controller,
    pub piste_eraser: &'a mut piste_eraser::Controller,
}

impl<'a> widgets::UiWidget<Input<'a>, Output<'a>> for Widget {
    fn init(&mut self, input: Input) {
        self.state = None;

        if input.mode != services::mode::Mode::Path && input.mode != services::mode::Mode::Piste {
            return;
        }
        let state = State {
            build: Button {
                hover_text: Some(describe_binding(&input.bindings.build)),
                highlighted: !input.piste_eraser.is_enabled(),
                clicked: false,
            },
            erase: Button {
                hover_text: Some(describe_binding(&input.bindings.erase)),
                highlighted: *input.piste_eraser.is_enabled(),
                clicked: false,
            },
        };
        self.state = Some(state);
    }

    fn draw(&mut self, ui: &mut engine::egui::Ui) {
        let Some(State { build, erase }) = self.state.as_mut() else {
            return;
        };
        ui.vertical(|ui| {
            ui.label("Build");
            ui.horizontal(|ui| {
                let build_button = ui
                    .button("+")
                    .on_hover_text(build.hover_text.take().unwrap());
                build.clicked = build_button.clicked();
                if build.highlighted {
                    build_button.highlight();
                }

                let erase_button = ui
                    .button("-")
                    .on_hover_text(erase.hover_text.take().unwrap());
                erase.clicked = erase_button.clicked();
                if erase.highlighted {
                    erase_button.highlight();
                }
            });
        });
    }

    fn update(&mut self, output: Output) {
        let Some(State { build, erase }) = &self.state else {
            return;
        };

        if build.clicked {
            output.path_builder.set_enabled(true);
            output.piste_builder.set_enabled(true);
            output.piste_eraser.set_enabled(false);
        } else if erase.clicked {
            output.path_builder.set_enabled(false);
            output.piste_builder.set_enabled(true);
            output.piste_eraser.set_enabled(true);
        }
    }
}
