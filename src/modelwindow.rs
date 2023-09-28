use eframe::egui::{self, RichText};
use rfd::FileDialog;
use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    path::PathBuf,
};

use crate::structs::model::Model;

pub struct ModelWindow {
    pub model: Model,
    pub model_path: String,
    number_of_materials: u8,
    material_paths: Vec<String>,
}

impl ModelWindow {
    /// Called once before the first frame.
    pub fn new(model_path: &PathBuf) -> Self {
        let model = Model::new(model_path);

        Self {
            model_path: model.get_model_path(),
            number_of_materials: model.get_materials_number(),
            material_paths: model.get_material_paths(),
            model: model,
        }
    }
}

impl ModelWindow {
    pub fn show(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        egui::TopBottomPanel::top(format!("{}-nav", self.model_path)).show_inside(ui, |ui| {
            ui.heading("SModelHexer");
        });

        ui.add_space(20.0);

        egui::Grid::new("model info")
            .num_columns(2)
            .spacing([4.0, 8.0])
            .min_col_width(50.0)
            .striped(true)
            .show(ui, |ui| {
                let model_path_label = ui.label("Model Path: ").on_hover_text("$includemodel");

                ui.text_edit_singleline(&mut self.model_path)
                    .labelled_by(model_path_label.id);

                ui.end_row();

                let material_paths_label =
                    ui.label("Material Paths: ").on_hover_text("$cdmaterials");

                ui.vertical(|ui| {
                    for material_path in &mut self.material_paths {
                        ui.text_edit_singleline(material_path);
                    }
                });

                ui.end_row();
            });

        ui.add_space(10.0);

        ui.vertical_centered(|ui| {
            if ui.button("Save").clicked() {
                if let Some(save_path) = FileDialog::new().save_file() {
                    println!("{}", self.model_path);
                    self.model.set_model_path(&self.model_path);
                    self.model.set_material_paths(&self.material_paths);
                    self.model.save(&save_path);
                }
            }
        });
    }
}