use eframe::{egui, egui_wgpu};
use std::sync::Arc;
use eframe::wgpu;
use wgpu::util::DeviceExt;

use crate::error::{Result, Error};

// GPU representation of the uniform data (must align to 16 bytes for WGSL structs)
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    angle: f32,
    _padding: [f32; 3], // Align struct to 16-byte boundary
}

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
        renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    };

    eframe::run_native(
        "nucleofile - wgpu Render Viewport",
        options,
        Box::new(|cc| Ok(Box::new(AppState::new(cc)))),
    )
    .map_err(|e| e.into())
}

// Stores pipeline and GPU uniform resources inside egui callback resources
struct TriangleRenderResources {
    pipeline: wgpu::RenderPipeline,
    uniform_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
}

struct AppState {
    label: String,
    angle: f32,
}

impl AppState {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let wgpu_render_state = cc
            .wgpu_render_state
            .as_ref()
            .expect("eframe must be run with the wgpu backend");

        let device = &wgpu_render_state.device;

        // 1. Create uniform buffer
        let initial_uniforms = Uniforms { angle: 0.0, _padding: [0.0; 3] };
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Triangle Uniform Buffer"),
            contents: bytemuck::bytes_of(&initial_uniforms),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // 2. Create bind group layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Triangle Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        // 3. Create bind group
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Triangle Bind Group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        // 4. Create shader and pipeline layout with bind group
        let shader = device.create_shader_module(
            wgpu::include_wgsl!("shaders/viewportShader.wgsl")
        );

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Triangle Pipeline Layout"),
            bind_group_layouts: &[Some(&bind_group_layout)],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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

        // Store shared GPU resources in callback_resources
        wgpu_render_state
            .renderer
            .write()
            .callback_resources
            .insert(TriangleRenderResources {
                pipeline,
                uniform_buffer,
                bind_group,
            });

        Self {
            label: "Structure Viewport".to_string(),
            angle: 0.0,
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

            // Angle slider in radians (0.0 to ~6.28 for full revolution)
            ui.add(egui::Slider::new(&mut self.angle, 0.0..=std::f32::consts::TAU)
                .text("angle (rad)"));

            ui.add_space(8.0);
            ui.label("Viewport Canvas:");

            let (rect, _response) = ui.allocate_exact_size(
                egui::vec2(500.0, 350.0),
                egui::Sense::drag(),
            );

            // Pass the current angle to the paint callback
            ui.painter().add(egui_wgpu::Callback::new_paint_callback(
                rect,
                TriangleCallback { angle: self.angle },
            ));
        });
    }
}

struct TriangleCallback {
    angle: f32,
}

impl egui_wgpu::CallbackTrait for TriangleCallback {
    fn prepare(
        &self,
        _device: &wgpu::Device,
        queue: &wgpu::Queue,
        _screen_descriptor: &egui_wgpu::ScreenDescriptor,
        _egui_encoder: &mut wgpu::CommandEncoder,
        callback_resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        // Retrieve resources and write updated angle to the uniform buffer before painting
        if let Some(resources) = callback_resources.get_mut::<TriangleRenderResources>() {
            let data = Uniforms {
                angle: self.angle,
                _padding: [0.0; 3],
            };
            queue.write_buffer(&resources.uniform_buffer, 0, bytemuck::bytes_of(&data));
        }
        Vec::new()
    }

    fn paint<'a>(
        &'a self,
        _info: egui::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'static>,
        callback_resources: &'a egui_wgpu::CallbackResources,
    ) {
        if let Some(resources) = callback_resources.get::<TriangleRenderResources>() {
            render_pass.set_pipeline(&resources.pipeline);
            render_pass.set_bind_group(0, &resources.bind_group, &[]);
            render_pass.draw(0..3, 0..1);
        }
    }
}
