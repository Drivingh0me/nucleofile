use eframe::egui;
use std::sync::Arc;

use crate::error::Result;

pub fn run_gui() -> Result<()> {
    let icon = image::load_from_memory(include_bytes!("../assets/icon.png"))
        .expect("Failed to load icon path")
        .into_rgba8();

    let (width, height) = icon.dimensions();
    let icon = egui::IconData {
        rgba: icon.into_raw(),
        width,
        height,
    };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
        .with_inner_size([640.0, 480.0])
            .with_icon(Arc::new(icon)),
        ..Default::default()
    };

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "eframe Test",
        options,
        Box::new(|cc| Ok(Box::new(AppState::new(cc))))
    );
    Ok(())
}

#[derive(Default)]
struct AppState {
    label: String,
    value: f32,
}

impl AppState {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and
        // cc.egui_ctx.set_global_style.
        // Restore app state using cc.storage (requires the "persistence"
        // feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and
        // buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for AppState {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        egui::Panel::top("top_panel").show_inside(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ui.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add_space(16.0);
                ui.menu_button("Edit", |ui| {
                    if ui.button("Clear").clicked() {
                        self.label = "".to_string();
                    }
                });
                ui.add_space(16.0);
                ui.menu_button("View", |ui| {
                    if ui.button("Clear").clicked() {
                        self.label = "".to_string();
                    }
                });
                ui.add_space(16.0);
                ui.menu_button("Tools", |ui| {
                    if ui.button("Clear").clicked() {
                        self.label = "".to_string();
                    }
                });
                ui.add_space(16.0);
                ui.menu_button("Help", |ui| {
                    if ui.button("Clear").clicked() {
                        self.label = "".to_string();
                    }
                });
                ui.add_space(16.0);
            });
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading(&mut self.label);

            ui.horizontal(|ui| {
                ui.label("write something: ");
                ui.text_edit_singleline(&mut self.label);
            });

            ui.add(egui::Slider::new(
                &mut self.value,
                0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                self.value += 1.0;
            }
        });
    }
}

