use engine::egui;

pub mod building_editor;
pub mod piste_build_mode;

pub trait Widget<T, U> {
    fn init(value: T) -> Self;
    fn draw(&mut self, ui: &mut egui::Ui);
    fn update(&self, value: U);
}
