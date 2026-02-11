use core::{f32, f64};
use std::error::Error;

use cgmath::num_traits::Signed;
use eframe::egui::{Ui, Vec2};
use eframe::wgpu::util::DeviceExt;
use eframe::{egui, egui_wgpu, wgpu};
use itertools::{Itertools, concat};
use log::{debug, error, info, warn};
use vec_utils::angle::{AngleDegrees, AngleRadians};
use vec_utils::vec3d::Vec3d;

use crate::car::Car;
use crate::graphics::camera::{Camera, CameraUniform};
use crate::graphics::color::{BLACK, BLUE, DARK_GRAY, GREEN, MIDDLE, RED, WHITE, coordinate_axis};
use crate::graphics::vertex::Vertex;
use crate::{ANGLE_EPSILON_DEGREES, get_test_car};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct BSApp {
    ride_car: Car,
    callback_data: AppRenderCallbackData
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
#[derive(Debug, Copy, Clone, Default)]
pub struct AppRenderCallbackData {
    #[serde(skip_serializing)]
    reset_camera: bool,
    #[serde(skip_serializing)]
    reset_car: bool,
    #[serde(skip_serializing)]
    debug_print: bool,
    motion: f64
}

struct BSRenderResources {
    render_pipeline: wgpu::RenderPipeline,
    camera: Camera,
    camera_uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    moved_car: Option<Car>,
    ride_car: Car,
    current_motion: f64,
    stop_moving: StopDirection
}

#[derive(Default, Debug, PartialEq)]
enum StopDirection {
    Positive,
    #[default]
    None,
    Negative
}

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub enum InteractionMode {
    Orbit(Vec2),
    Pan(Vec2),
    #[default]
    None
}

#[derive(Debug, Copy, Clone, Default)]
struct BSRenderCallback {
    interaction_mode: InteractionMode,
    scroll_delta: Option<f32>,
    reset_camera: bool,
    reset_car: bool,
    debug_print: bool,
    motion: f64
}

impl Default for BSApp {
    fn default() -> Self {
        Self {
            ride_car: get_test_car(),
            callback_data: AppRenderCallbackData::default()
        }
    }
}

impl AppRenderCallbackData {
    fn reset(&mut self) {
        self.reset_car = false;
        self.reset_camera = false;
        self.debug_print = false;
    }
}

impl BSApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Result<BSApp, Box<dyn Error>> {
        let wgpu_render_state = cc
            .wgpu_render_state
            .as_ref()
            .ok_or("No wgpu renderer exists")?;

        let device = &wgpu_render_state.device;

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into())
        });

        let initial_size_f32 = cc.egui_ctx.content_rect().size();
        let size = wgpu::Extent3d {
            width: initial_size_f32.x as u32,
            height: initial_size_f32.y as u32,
            depth_or_array_layers: 1
        };
        let aspect = size.width as f32 / size.height as f32;

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

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
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
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: &[],
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST
        });

        let num_indices: u32 = 0;

        let app: BSApp = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        };

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
                vertex_buffer,
                index_buffer,
                num_indices,
                moved_car: None,
                ride_car: app.ride_car,
                current_motion: 0.0,
                stop_moving: StopDirection::default()
            });

        Ok(app)
    }
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
            ui.with_layout(
                egui::Layout::top_down_justified(egui::Align::Center),
                |ui| {
                    ui.heading("Reset Buttons");
                    if ui.button("Reset Car").clicked() {
                        self.callback_data.reset_car = true;
                        self.callback_data.motion = 0.0;
                    }
                    if ui.button("Reset Camera").clicked() {
                        self.callback_data.reset_camera = true;
                    }
                    ui.heading("Motion");
                    ui.scope(|ui| {
                        ui.spacing_mut().slider_width = ui.available_width()
                            - ui.spacing().interact_size.x
                            - ui.spacing().button_padding.x
                            - 4.0;
                        ui.add(
                            egui::Slider::new(&mut self.callback_data.motion, -100.0..=40.0)
                                .update_while_editing(false)
                        );
                    });
                    ui.heading("Debug");
                    if ui.button("Print Car").clicked() {
                        self.callback_data.debug_print = true;
                    }

                    // ui.heading("Logs");
                    // egui_logger::logger_ui().show(ui);
                }
            );
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                let callback_data = self.callback_data;
                self.callback_data.reset();
                self.custom_painting(ui, &callback_data);
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

impl egui_wgpu::CallbackTrait for BSRenderCallback {
    fn prepare(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        _screen_descriptor: &egui_wgpu::ScreenDescriptor,
        _egui_encoder: &mut wgpu::CommandEncoder,
        resources: &mut egui_wgpu::CallbackResources
    ) -> Vec<wgpu::CommandBuffer> {
        if let Some(resources) = resources.get_mut::<BSRenderResources>() {
            resources.prepare(device, queue, self);
        } else {
            warn!("Resources not found");
        }
        Vec::new()
    }

    fn paint(
        &self,
        _info: egui::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'static>,
        resources: &egui_wgpu::CallbackResources
    ) {
        if let Some(resources) = resources.get::<BSRenderResources>() {
            resources.paint(render_pass);
        } else {
            warn!("Resources not found");
        }
    }
}

impl BSApp {
    fn custom_painting(&mut self, ui: &mut egui::Ui, callback_data: &AppRenderCallbackData) {
        let (rect, response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::drag());
        let interaction_mode = if response.dragged() {
            let delta = response.drag_delta();
            ui.input(|i| {
                // if i.pointer.button_down(egui::PointerButton::Middle) {
                if i.pointer.button_down(egui::PointerButton::Primary) {
                    InteractionMode::Orbit(delta)
                } else if i.pointer.button_down(egui::PointerButton::Secondary) {
                    InteractionMode::Pan(delta)
                } else {
                    InteractionMode::None
                }
            })
        } else {
            InteractionMode::None
        };

        let scroll_delta = if response.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            let scroll_delta = ui.input(|i| i.smooth_scroll_delta.y);
            if scroll_delta != 0.0 {
                Some(scroll_delta)
            } else {
                None
            }
        } else {
            None
        };

        ui.painter().add(egui_wgpu::Callback::new_paint_callback(
            rect,
            BSRenderCallback {
                interaction_mode,
                scroll_delta,
                reset_car: callback_data.reset_car,
                reset_camera: callback_data.reset_camera,
                motion: callback_data.motion,
                debug_print: callback_data.debug_print
            }
        ));
    }
}

impl BSRenderResources {
    fn prepare(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        callback_info: &BSRenderCallback
    ) {
        match callback_info.interaction_mode {
            InteractionMode::Pan(delta) => {
                self.camera.pan(delta);
            }
            InteractionMode::Orbit(delta) => {
                self.camera.orbit(delta);
            }
            InteractionMode::None => {}
        }
        if let Some(delta) = callback_info.scroll_delta {
            self.camera.zoom(delta);
        }
        if self.moved_car.is_none()
            || callback_info.reset_car && self.current_motion.abs() > f64::EPSILON
        {
            self.moved_car = Some(self.ride_car);
            self.current_motion = 0.0;
            self.stop_moving = StopDirection::None;
            self.write_buffers(device, queue);
        } else if (self.current_motion - callback_info.motion).abs() > f64::EPSILON {
            let delta = (self.current_motion - callback_info.motion)
                .clamp(-ANGLE_EPSILON_DEGREES, ANGLE_EPSILON_DEGREES);
            if self.stop_moving.dir_ok(delta) {
                if let Err(e) = self
                    .moved_car
                    .as_mut()
                    .unwrap()
                    .rotate(AngleDegrees::new(delta))
                {
                    error!(
                        "Error rotating car: {e}. Current motion: {}",
                        self.current_motion
                    );
                    self.stop_moving = StopDirection::from_f64(delta);
                } else {
                    self.current_motion -= delta;
                    self.write_buffers(device, queue);
                    self.stop_moving = StopDirection::None;
                }
            }
        }
        if callback_info.reset_camera {
            self.camera.reset();
        }
        self.camera_uniform.update_view_proj(&self.camera);
        queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform])
        );
        if callback_info.debug_print {
            dbg!(self.moved_car.map(|car| car.front));
        }
    }

    fn write_buffers(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        let mut buffers: Vec<(Vec<Vertex>, Vec<u16>)> = Vec::new();

        buffers = [buffers, self.ride_car.get_vertex_data(DARK_GRAY)].concat();

        buffers = [buffers, self.moved_car.unwrap().get_vertex_data(WHITE)].concat();

        buffers.push(coordinate_axis());
        self.update_buffers(device, queue, &buffers);
    }

    fn update_buffers(
        &mut self,
        device: &wgpu::Device,
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
        debug!(
            "{} Indices, {} Vertexs",
            self.num_indices,
            vertex_data.len()
        );

        let v_bytes = bytemuck::cast_slice(&vertex_data);
        let i_bytes = bytemuck::cast_slice(&index_data);

        if self.vertex_buffer.size() == v_bytes.len() as u64 {
            queue.write_buffer(&self.vertex_buffer, 0, v_bytes);
        } else {
            self.vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: v_bytes,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
            });
        }

        if self.index_buffer.size() == i_bytes.len() as u64 {
            queue.write_buffer(&self.index_buffer, 0, i_bytes);
        } else {
            self.index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: i_bytes,
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST
            });
        }
    }

    fn paint(&self, render_pass: &mut wgpu::RenderPass<'_>) {
        if self.num_indices == 0 {
            return;
        }
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }
}

impl StopDirection {
    fn from_f64(value: f64) -> Self {
        if value > 0.0 {
            StopDirection::Positive
        } else if value < 0.0 {
            StopDirection::Negative
        } else {
            StopDirection::None
        }
    }

    fn dir_ok(&self, value: f64) -> bool {
        match self {
            StopDirection::Positive => value < 0.0,
            StopDirection::Negative => value > 0.0,
            StopDirection::None => true
        }
    }
}
