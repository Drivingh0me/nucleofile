use eframe::egui;

use crate::error::Result;

pub fn run_gui() -> Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "eframe Test",
        native_options,
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
                ui.menu_button("file", |ui| {
                    if ui.button("Quit").clicked() {
                        ui.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add_space(16.0);
                // egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("Hello World!");

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

