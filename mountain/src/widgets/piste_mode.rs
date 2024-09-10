use crate::controllers::{piste_builder, piste_eraser};
use crate::services;
use crate::widgets;

#[derive(Default)]
pub struct Widget {
    state: Option<State>,
}

struct State {
    building: bool,
    erasing: bool,
    building_clicked: bool,
    erasing_clicked: bool,
}

pub struct Input<'a> {
    pub mode: services::mode::Mode,
    pub piste_eraser: &'a piste_eraser::Controller,
}

pub struct Output<'a> {
    pub path_builder: &'a mut piste_builder::Controller,
    pub piste_builder: &'a mut piste_builder::Controller,
    pub piste_eraser: &'a mut piste_eraser::Controller,
}

impl<'a> widgets::Widget<Input<'a>, Output<'a>> for Widget {
    fn init(&mut self, input: Input) {
        if input.mode != services::mode::Mode::Path && input.mode != services::mode::Mode::Piste {
            return;
        }
        self.state = Some(State {
            building: !*input.piste_eraser.is_enabled(),
            erasing: *input.piste_eraser.is_enabled(),
            building_clicked: false,
            erasing_clicked: false,
        });
    }

    fn draw(&mut self, ui: &mut engine::egui::Ui) {
        let Some(State {
            building,
            erasing,
            building_clicked,
            erasing_clicked,
        }) = self.state.as_mut()
        else {
            return;
        };
        ui.vertical(|ui| {
            ui.label("Build");
            ui.horizontal(|ui| {
                let build_button = ui.button("+");
                *building_clicked = build_button.clicked();
                if *building {
                    build_button.highlight();
                }
                let erase_button = ui.button("-");
                *erasing_clicked = erase_button.clicked();
                if *erasing {
                    erase_button.highlight();
                }
            });
        });
    }

    fn update(&self, output: Output) {
        let Some(State {
            building_clicked,
            erasing_clicked,
            ..
        }) = self.state
        else {
            return;
        };

        if building_clicked {
            output.path_builder.set_enabled(true);
            output.piste_builder.set_enabled(true);
            output.piste_eraser.set_enabled(false);
        }

        if erasing_clicked {
            output.path_builder.set_enabled(false);
            output.piste_builder.set_enabled(true);
            output.piste_eraser.set_enabled(true);
        }
    }
}
