use engine::binding::Binding;

use crate::controllers::{piste_builder, piste_eraser};

pub struct Bindings {
    pub build: Binding,
    pub erase: Binding,
}

pub fn handle(
    bindings: &Bindings,
    event: &engine::events::Event,
    path_builder: &mut piste_builder::Controller,
    piste_builder: &mut piste_builder::Controller,
    piste_eraser: &mut piste_eraser::Controller,
) {
    if bindings.build.binds_event(event) || matches!(event, engine::events::Event::Init) {
        piste_eraser.set_enabled(false);
        path_builder.set_enabled(true);
        piste_builder.set_enabled(true);
    }

    if bindings.erase.binds_event(event) {
        piste_eraser.set_enabled(true);
        path_builder.set_enabled(false);
        piste_builder.set_enabled(false);
    }
}
