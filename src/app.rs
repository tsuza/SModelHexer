use eframe::egui::{self, RichText};
use rfd::FileDialog;
use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    path::PathBuf,
};

use crate::modelwindow::ModelWindow;

#[derive(Default)]
pub struct SModelHexerApp {
    dropped_files: Vec<PathBuf>,
    model_windows: Vec<ModelWindow>,
}

impl SModelHexerApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for SModelHexerApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Navigation bar
        egui::TopBottomPanel::top("nav").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.heading("SModelHexer");
            });
        });

        let mut test = true;

        for model_window in &mut self.model_windows {
            egui::Window::new(RichText::new(model_window.model.get_model_path()))
                .open(&mut test)
                .min_width(500.0)
                .show(ctx, |ui| {
                    model_window.show(ctx, ui);
                });
        }

        // Main content
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                if ui
                    .add(
                        egui::Label::new("Drag & Drop files, or click here...")
                            .sense(egui::Sense::click()),
                    )
                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                    .clicked()
                {
                    if let Some(paths) = FileDialog::new()
                        .add_filter("Model Files", &["mdl"])
                        .pick_files()
                    {
                        self.dropped_files = paths;

                        for dropped_file in &self.dropped_files {
                            self.model_windows.push(ModelWindow::new(&dropped_file));
                        }
                    }
                }
            });

            ctx.input(|i| {
                if !i.raw.dropped_files.is_empty() {
                    self.dropped_files = i
                        .raw
                        .dropped_files
                        .iter()
                        .filter_map(|dropped_file| dropped_file.path.clone())
                        .collect();

                    for dropped_file in &self.dropped_files {
                        self.model_windows.push(ModelWindow::new(&dropped_file));
                    }
                }
            });
        });
    }
}
