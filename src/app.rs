use std::error::Error;
use std::io::ErrorKind::AddrNotAvailable;
use std::iter;

use eframe::egui::Ui;
use eframe::wgpu::util::DeviceExt;
use eframe::{egui, egui_wgpu, wgpu};
use itertools::{Itertools, concat};
use log::info;
use vec_utils::angle::{AngleDegrees, AngleRadians};
use vec_utils::vec3d::Vec3d;

use crate::car::Car;
use crate::get_test_car;
use crate::graphics::camera::{Camera, CameraController, CameraUniform};
use crate::graphics::color::{BLACK, BLUE, DARK_GRAY, GREEN, MIDDLE, RED, WHITE, coordinate_axis};
use crate::graphics::input::InputHandler;
use crate::graphics::vertex::Vertex;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct BSApp {
    ride_car: Car
}

#[derive(Default)]
pub struct BSRender {}

impl Default for BSApp {
    fn default() -> Self {
        Self {
            ride_car: get_test_car()
        }
    }
}

impl BSApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app: BSApp = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        };
        initialize_renderer(cc, app.ride_car);
        app
    }
}

fn initialize_renderer(cc: &eframe::CreationContext<'_>, car: Car) -> Result<(), Box<dyn Error>> {
    let wgpu_render_state = cc
        .wgpu_render_state
        .as_ref()
        .ok_or("No wgpu renderer exists")?;

    let initial_size_f32 = cc.egui_ctx.content_rect().size();
    let size = wgpu::Extent3d {
        width: initial_size_f32.x as u32,
        height: initial_size_f32.y as u32,
        depth_or_array_layers: 1
    };
    let aspect = size.width as f32 / size.height as f32;
    let device = &wgpu_render_state.device;

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into())
    });

    let camera = Camera {
        eye: (5.0, 5.0, 5.0).into(),
        target: (0.0, 0.0, 0.0).into(),
        up: cgmath::Vector3::unit_z(),
        aspect,
        fovy: 45.0,
        znear: 0.01,
        zfar: 100.0
    };
    let mut camera_uniform = CameraUniform::new();
    camera_uniform.update_view_proj(&camera);

    let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Camera Buffer"),
        contents: bytemuck::cast_slice(&[camera_uniform]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
    });

    let camera_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None
                },
                count: None
            }],
            label: Some("camera_bind_group_layout")
        });

    let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &camera_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: camera_buffer.as_entire_binding()
        }],
        label: Some("camera_bind_group")
    });

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[&camera_bind_group_layout],
        push_constant_ranges: &[]
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[Vertex::desc()],
            compilation_options: Default::default()
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format: wgpu_render_state.target_format,
                blend: Some(wgpu::BlendState {
                    color: wgpu::BlendComponent::REPLACE,
                    alpha: wgpu::BlendComponent::REPLACE
                }),
                write_mask: wgpu::ColorWrites::ALL
            })],
            compilation_options: Default::default()
        }),
        primitive: wgpu::PrimitiveState {
            // topology: wgpu::PrimitiveTopology::TriangleList,
            topology: wgpu::PrimitiveTopology::LineList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            // Setting this to anything other than Fill requires Features::POLYGON_MODE_LINE
            // or Features::POLYGON_MODE_POINT
            polygon_mode: wgpu::PolygonMode::Fill,
            // Requires Features::DEPTH_CLIP_CONTROL
            unclipped_depth: false,
            // Requires Features::CONSERVATIVE_RASTERIZATION
            conservative: false
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
        cache: None
    });

    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: &[],
        usage: wgpu::BufferUsages::VERTEX
    });

    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: &[],
        usage: wgpu::BufferUsages::INDEX
    });

    let camera_controller = CameraController::new(0.025);
    let input_handler = InputHandler::new();
    let num_indices: u32 = 0;

    wgpu_render_state
        .renderer
        .write()
        .callback_resources
        .insert(BSRenderResources {
            render_pipeline,
            camera,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            camera_controller,
            vertex_buffer,
            index_buffer,
            num_indices,
            moved_car: car,
            ride_car: car,
            input_handler
        });

    Ok(())
}

impl eframe::App for BSApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add_space(16.0);
                ui.menu_button("View", |ui| {
                    egui::widgets::global_theme_preference_buttons(ui);
                });
            });
        });
        egui::SidePanel::left("Config").show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.label("Your name: ");
            if ui.button("Increment").clicked() {}
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                custom_painting(ui);
            });
        });
    }
}

// Callbacks in egui_wgpu have 3 stages:
// * prepare (per callback impl)
// * finish_prepare (once)
// * paint (per callback impl)
//
// The prepare callback is called every frame before paint and is given access to the wgpu
// Device and Queue, which can be used, for instance, to update buffers and uniforms before
// rendering.
// If [`egui_wgpu::Renderer`] has [`egui_wgpu::FinishPrepareCallback`] registered,
// it will be called after all `prepare` callbacks have been called.
// You can use this to update any shared resources that need to be updated once per frame
// after all callbacks have been processed.
//
// On both prepare methods you can use the main `CommandEncoder` that is passed-in,
// return an arbitrary number of user-defined `CommandBuffer`s, or both.
// The main command buffer, as well as all user-defined ones, will be submitted together
// to the GPU in a single call.
//
// The paint callback is called after finish prepare and is given access to egui's main render pass,
// which can be used to issue draw commands.
struct BSRenderCallback {}

impl egui_wgpu::CallbackTrait for BSRenderCallback {
    fn prepare(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        _screen_descriptor: &egui_wgpu::ScreenDescriptor,
        _egui_encoder: &mut wgpu::CommandEncoder,
        resources: &mut egui_wgpu::CallbackResources
    ) -> Vec<wgpu::CommandBuffer> {
        if let Some(resources) = resources.get_mut::<&mut BSRenderResources>() {
            resources.prepare(device, queue);
        }
        Vec::new()
    }

    fn paint(
        &self,
        _info: egui::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'static>,
        resources: &egui_wgpu::CallbackResources
    ) {
        if let Some(resources) = resources.get::<&BSRenderResources>() {
            resources.paint(render_pass);
        }
    }
}

fn custom_painting(ui: &mut egui::Ui) {
    let (id, rect) = ui.allocate_space(ui.available_size());

    ui.painter().add(egui_wgpu::Callback::new_paint_callback(
        rect,
        BSRenderCallback {}
    ));
}

struct BSRenderResources {
    render_pipeline: wgpu::RenderPipeline,
    camera: Camera,
    camera_uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    camera_controller: CameraController,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    moved_car: Car,
    ride_car: Car,
    input_handler: InputHandler
}

impl BSRenderResources {
    fn prepare(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        self.camera_controller.update_camera(&mut self.camera);
        let update_car = self
            .input_handler
            .update_car(&self.ride_car, &mut self.moved_car);
        if update_car.0 {
            if update_car.1 {
                self.moved_car = self.ride_car;
            }
            self.write_buffers(device, queue);
        }
        self.camera_uniform.update_view_proj(&self.camera);
        queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform])
        );
    }

    fn write_buffers(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        let mut buffers: Vec<(Vec<Vertex>, Vec<u16>)> = Vec::new();

        buffers = [buffers, self.ride_car.get_vertex_data(DARK_GRAY)].concat();

        buffers = [buffers, self.moved_car.get_vertex_data(WHITE)].concat();

        buffers.push(coordinate_axis());

        self.update_buffers(device, queue, &buffers);
    }

    fn update_buffers(
        &mut self,
        _device: &wgpu::Device,
        queue: &wgpu::Queue,
        data: &Vec<(Vec<Vertex>, Vec<u16>)>
    ) {
        let mut vertex_data: Vec<Vertex> = Vec::new();
        let mut index_data: Vec<u16> = Vec::new();
        let mut start: u16 = 0;

        for (i, j) in data {
            vertex_data = [vertex_data, i.clone()].concat();
            index_data = [index_data, j.iter().map(|i| *i + start).collect()].concat();
            start = vertex_data.len() as u16;
        }

        self.num_indices = index_data.len() as u32;
        println!(
            "{} Indices, {} Vertexs",
            self.num_indices,
            vertex_data.len()
        );
        queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&*vertex_data));
        queue.write_buffer(&self.index_buffer, 0, bytemuck::cast_slice(&*index_data));
        // self.vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        //     label: Some("Vertex Buffer"),
        //     contents: bytemuck::cast_slice(&*vertex_data),
        //     usage: wgpu::BufferUsages::VERTEX
        // });
        // self.index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        //     label: Some("Index Buffer"),
        //     contents: bytemuck::cast_slice(&*index_data),
        //     usage: wgpu::BufferUsages::INDEX
        // });
    }

    fn paint(&self, render_pass: &mut wgpu::RenderPass<'_>) {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }
}
