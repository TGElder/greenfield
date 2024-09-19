use std::collections::HashSet;
use std::fs::{read_dir, DirEntry};
use std::io::Error;

use engine::egui;

use crate::widgets::UiWidget;

#[derive(Default)]
pub struct Widget {
    pub focus: String,
}

impl UiWidget<(), ()> for Widget {
    fn init(&mut self, value: ()) {}

    fn draw(&mut self, ui: &mut engine::egui::Ui) {
        let paths = read_dir("./saves/").unwrap();

        let files = paths.filter_map(file_stem).collect::<Vec<_>>();
        let selected_text = if files.contains(&self.focus) {
            &self.focus
        } else {
            ""
        };

        egui::ComboBox::from_id_source(0)
            .selected_text(selected_text)
            .show_ui(ui, |ui| {
                for file in files {
                    ui.selectable_value(&mut self.focus, file.clone(), file);
                }
            });
    }

    fn update(&self, value: ()) {}
}

fn file_stem(maybe_path: Result<DirEntry, Error>) -> Option<String> {
    maybe_path.map(|path| path.path()).ok().and_then(|path| {
        if !path
            .extension()
            .map(|extension| extension == "save")
            .unwrap_or(false)
        {
            return None;
        }
        path.file_stem()
            .and_then(|stem| stem.to_str())
            .map(|x| x.to_string())
    })
}
