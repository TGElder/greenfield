use std::fs::{read_dir, ReadDir};
use std::iter::once;
use std::path::{Path, PathBuf};

use engine::egui;

use crate::widgets::UiWidget;

#[derive(Default)]
pub struct Widget {
    saves: Vec<String>,
    focus: String,
}

pub struct Input<'a> {
    pub directory: &'a str,
    pub extension: &'a str,
}

impl Widget {
    pub fn focus(&self) -> &str {
        &self.focus
    }
}

impl UiWidget<Input<'_>, ()> for Widget {
    fn init(&mut self, input: Input) {
        self.saves = get_saves(input.directory, input.extension);
    }

    fn draw(&mut self, ui: &mut engine::egui::Ui) {
        let selected_text = if self.saves.contains(&self.focus) {
            &self.focus
        } else {
            ""
        };

        egui::ComboBox::from_id_source(0)
            .selected_text(selected_text)
            .show_ui(ui, |ui| {
                for file in self.saves.iter() {
                    ui.selectable_value(&mut self.focus, file.clone(), file);
                }
            });
    }

    fn update(&mut self, _: ()) {}
}

fn get_saves(directory: &str, extension: &str) -> Vec<String> {
    once(read_dir(directory))
        .flatten()
        .flat_map(get_paths)
        .filter(|path| has_extension(path, extension))
        .flat_map(try_to_string)
        .collect()
}

fn get_paths(read_dir: ReadDir) -> impl Iterator<Item = PathBuf> {
    read_dir.flatten().map(|dir_entry| dir_entry.path())
}

fn has_extension(path: &Path, target_extension: &str) -> bool {
    path.extension()
        .map(|extension| extension == target_extension)
        .unwrap_or(false)
}

fn try_to_string(path: PathBuf) -> Option<String> {
    path.file_stem()
        .and_then(|stem| stem.to_str())
        .map(|x| x.to_string())
}
