use engine::egui;

pub mod building_editor;

pub trait Widget<T, U> {
    fn init(&mut self, value: T);
    fn draw(&mut self, ui: &mut egui::Ui);
    fn update(&self, value: U);
}
