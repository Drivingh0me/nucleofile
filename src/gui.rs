use eframe::{egui, egui_wgpu};
use std::sync::Arc;
use eframe::wgpu;

use crate::error::{Result, Error};

// Triangle render and viewport was directly implemented with AI.

// Using standard error boxing or your custom crate::error::Result
// type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

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
            .with_inner_size([800.0, 600.0])
            .with_icon(Arc::new(icon)),
        renderer: eframe::Renderer::Wgpu, // Force wgpu backend
        ..Default::default()
    };

    eframe::run_native(
        "neucleofile - wgpu Render Viewport",
        options,
        Box::new(|cc| Ok(Box::new(AppState::new(cc)))),
    )
    .map_err(|e| e.into())
}

// AI generated struct - unvalidated
struct TriangleRenderCallback {
    pipeline: wgpu::RenderPipeline,
}

// AI generated impl - unvalidated
impl egui_wgpu::CallbackTrait for TriangleRenderCallback {
    fn prepare(
        &self,
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
        _screen_descriptor: &egui_wgpu::ScreenDescriptor,
        _egui_encoder: &mut wgpu::CommandEncoder,
        _callback_resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        Vec::new()
    }

    fn paint<'a>(
        &'a self,
        _info: egui::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'static>,
        _callback_resources: &'a egui_wgpu::CallbackResources,
    ) {
        render_pass.set_pipeline(&self.pipeline);
        // Draw 3 hardcoded vertices defined in the WGSL shader
        render_pass.draw(0..3, 0..1);
    }
}

struct AppState {
    label: String,
    value: f32,
}

impl AppState {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let wgpu_render_state = cc
            .wgpu_render_state
            .as_ref()
            .expect("eframe must be run with the wgpu backend");

        let device = &wgpu_render_state.device;

        // Embedded WGSL shader for a basic triangle
        let shader = device.create_shader_module(
            wgpu::include_wgsl!("shaders/viewportShader.wgsl")
        );

        // AI generated device - unvalidated
        let pipeline_layout = device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Triangle Pipeline Layout"),
            bind_group_layouts: &[],
            immediate_size: 0,
        });

        // AI generated pipeline - unvalidated
        let pipeline = device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Triangle Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu_render_state.target_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        // Register callback state into egui_wgpu paint callbacks resources
        wgpu_render_state
            .renderer
            .write()
            .callback_resources
            .insert(TriangleRenderCallback { pipeline });

        Self {
            label: "Structure Viewport".to_string(),
            value: 0.0,
        }
    }
}

impl eframe::App for AppState {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("top_panel").show_inside(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ui.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.menu_button("Edit", |ui| {
                    if ui.button("Clear").clicked() {
                        self.label.clear();
                    }
                });
            });
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading(&self.label);

            ui.horizontal(|ui| {
                ui.label("Title: ");
                ui.text_edit_singleline(&mut self.label);
            });

            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0)
                .text("value"));

            ui.add_space(8.0);
            ui.label("Viewport Canvas:");

            // Allocate spatial area for the wgpu render canvas
            let (rect, _response) =
                ui.allocate_exact_size(egui::vec2(500.0, 350.0),
                egui::Sense::drag());

            // Add PaintCallback targeting the custom callback resource
            ui.painter().add(egui_wgpu::Callback::new_paint_callback(
                rect,
                TriangleCallback,
            ));
        });
    }
}

struct TriangleCallback;

// AI generated impl - unvalidated
impl egui_wgpu::CallbackTrait for TriangleCallback {
    fn paint(
        &self,
        info: egui::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'static>,
        callback_resources: &egui_wgpu::CallbackResources,
    ) {
        if let Some(cb) = callback_resources.get::<TriangleRenderCallback>() {
            cb.paint(info, render_pass, callback_resources);
        }
    }
}
