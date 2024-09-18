use engine::egui;

pub mod building_editor;
pub mod piste_build_mode;
pub mod toaster;

pub trait ContextWidget<T, U> {
    fn init(&mut self, value: T);
    fn draw(&mut self, ctx: &egui::Context);
    fn _update(&self, value: U);
}

pub trait UiWidget<T, U> {
    fn init(&mut self, value: T);
    fn draw(&mut self, ui: &mut egui::Ui);
    fn update(&self, value: U);
}
