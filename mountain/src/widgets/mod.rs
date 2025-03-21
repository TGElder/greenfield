use engine::egui;

pub mod building_editor;
pub mod entity_window;
pub mod lift_editor;
pub mod menu;
pub mod piste_build_mode;
pub mod save_picker;
pub mod toaster;

pub trait ContextWidget<T, U> {
    fn init(&mut self, input: T);
    fn draw(&mut self, ctx: &egui::Context);
    fn update(&mut self, output: U);
}

pub trait UiWidget<T, U> {
    fn init(&mut self, input: T);
    fn draw(&mut self, ui: &mut egui::Ui);
    fn update(&mut self, output: U);
}
