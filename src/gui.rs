use eframe::egui;
// v0.31
// use eframe::egui::{
//     self, CentralPanel, Context, TextStyle, FontId, 
//     FontFamily, TopBottomPanel, ComboBox,
// };

use crate::error::Result;

pub fn run_gui() -> Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "eframe Test",
        native_options,
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc))))
    );
    Ok(())
}

#[derive(Default)]
struct MyEguiApp {}

impl MyEguiApp {
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

impl eframe::App for MyEguiApp {
   fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
       egui::CentralPanel::default().show_inside(ui, |ui| {
           ui.heading("Hello World!");
       });
   }
}

// 0.31
// State of the app
// #[derive(Default)]
// struct App {
//     subs: Vec<(String, String)>,
//     name_input: String,
//     url_input: String,
//     selected_value: Option<usize>,
// }
//
// impl eframe::App for App {
//     fn update(
//         &mut self, 
//         ctx: &eframe::egui::Context, 
//         _frame: &mut eframe::Frame
//     ) {
//         set_styles(ctx);
//         show_top_bar(ctx);
//         CentralPanel::default()
//             .show(ctx, |ui| {
//                 self.show_rss_form(ui);
//                 ComboBox::from_label("Select Rss")
//                     .selected_text(if let Some(index) = self.selected_value {
//                         if let Some(sub) = self.subs.get(index) {
//                                 &sub.0
//                         } else {"Select me"}
//                     } else { "Select me"
//                     }).show_ui(ui, |ui| {
//                         for (i, rss) in self.subs.iter().enumerate() {
//                             if ui.selectable_value(
//                                 &mut self.selected_value,
//                                 Some(i),
//                                 &rss.0,
//                             ).clicked() {
//                                 if let Some(sub) = self.subs.get(i) {
//                                 }
//                             }
//                         }
//                     });
//                 // Debug Sub
//                 for sub in &self.subs {
//                     ui.heading(&sub.0);
//                     ui.heading(&sub.1);
//                 }
//             });
//     }
// }
//
// impl App {
//     fn show_rss_form(&mut self, ui: &mut egui::Ui) {
//         ui.collapsing("New Rss", |ui| {
//             ui.vertical_centered_justified(|ui| {
//                 ui.label("Name");
//                 ui.text_edit_singleline(&mut self.name_input);
//                 ui.label("Url");
//                 ui.text_edit_singleline(&mut self.url_input);
//                 ui.horizontal(|ui| {
//                     if ui.button("Submit").clicked() {
//                         self.subs.push((
//                             self.name_input.clone(),
//                             self.url_input.clone(),
//                         ));
//                         self.name_input.clear();
//                         self.url_input.clear();
//                     }
//                     if ui.button("Clear").clicked() {
//                         self.name_input.clear();
//                         self.url_input.clear();
//                     }
//                 });
//             });
//         });
//     }
// }
//
// fn show_top_bar(ctx: &Context) {
//     TopBottomPanel::top("menu_bar").show(ctx, |ui|{
//         egui::menu::bar(ui, |ui|{
//             ui.menu_button("File", |ui|{
//                 if ui.button("Exit").clicked() {
//                     ctx.send_viewport_cmd(egui::ViewportCommand::Close);
//                 }
//                 if ui.button("Close").clicked() {
//                     ctx.send_viewport_cmd(egui::ViewportCommand::Close);
//                 }
//             });
//             ui.menu_button("View", |ui|{
//                 if ui.button("Exit").clicked() {
//                     ctx.send_viewport_cmd(egui::ViewportCommand::Close);
//                 }
//                 if ui.button("Close").clicked() {
//                     ctx.send_viewport_cmd(egui::ViewportCommand::Close);
//                 }
//             });
//         });
//     });
// }
//
// fn set_styles(ctx: &Context) {
//     let mut style = (*ctx.style()).clone();
//     style.text_styles = [
//         (TextStyle::Heading, FontId::new(30.0, FontFamily::Monospace)),
//         (TextStyle::Body, FontId::new(18.0, FontFamily::Monospace)),
//         (TextStyle::Button, FontId::new(22.0, FontFamily::Monospace)),
//         (TextStyle::Small, FontId::new(14.0, FontFamily::Monospace)),
//     ].into();
//     ctx.set_style(style);
// }
//
// in main
    // let options = eframe::NativeOptions {
    //     viewport: eframe::egui::ViewportBuilder::default()
    //         .with_resizable(true)
    //         .with_inner_size([320.0, 240.0]),
    //     ..Default::default()
    // };
    // eframe::run_native("Jrss", options, 
    //     Box::new(|_cc| Ok(Box::<App>::default())))
