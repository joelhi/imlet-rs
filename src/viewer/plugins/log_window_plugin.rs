use std::sync::{Arc, Mutex};

use bevy::{
    app::{App, Plugin, Update},
    asset::{Asset, Assets},
    color::LinearRgba,
    log::{tracing_subscriber::Layer, BoxedLayer, LogPlugin},
    pbr::{
        wireframe::WireframeConfig, Material, MaterialMeshBundle, MaterialPipeline,
        MaterialPipelineKey,
    },
    prelude::{default, Commands, Res, ResMut},
    reflect::TypePath,
    render::{
        mesh::{MeshVertexBufferLayoutRef, PrimitiveTopology},
        render_asset::RenderAssetUsages,
        render_resource::{
            AsBindGroup, PolygonMode, RenderPipelineDescriptor, ShaderRef,
            SpecializedMeshPipelineError,
        },
    },
};
use bevy_egui::{
    egui::{self, emath::Numeric, text::LayoutJob, Color32, Layout, ScrollArea, Ui},
    EguiContexts,
};
use num_traits::Float;

use crate::viewer::custom_layer::{CustomLayer, LogMessages};

use super::{CurrentBounds, Icons, ModelMaterial, ViewSettings};

pub struct LogWindowPlugin<T> {
    _marker: std::marker::PhantomData<T>,
}

// Implement a default constructor for ModelExplorerPlugin to make instantiation easier.
impl<T> LogWindowPlugin<T> {
    pub fn new() -> Self {
        LogWindowPlugin {
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T> Plugin for LogWindowPlugin<T>
where
    T: Float + Send + Sync + Numeric + 'static, // Ensure T meets the required constraints (adjust as needed).
{
    fn build(&self, app: &mut App) {
        app.add_plugins(LogPlugin {
            custom_layer,
            ..default()
        })
        .add_systems(Update, logging_panel::<T>);
    }
}

fn custom_layer(app: &mut App) -> Option<BoxedLayer> {
    let log_messages = LogMessages::default();
    let inner_log_clone = log_messages.messages.clone();
    app.insert_resource(log_messages);
    Some(Box::new(vec![
        bevy::log::tracing_subscriber::fmt::layer()
            .with_file(true)
            .boxed(),
        Box::new(CustomLayer {
            log_messages: inner_log_clone,
        }),
    ]))
}

const TINT: u8 = 155;

pub fn logging_panel<T: Float + Send + Sync + Numeric + 'static>(
    mut contexts: EguiContexts,
    log_handle: Res<LogMessages>,
    mut wireframe_config: ResMut<WireframeConfig>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<bevy::prelude::Mesh>>,
    icons: Res<Icons>,
    mut view_settings: ResMut<ViewSettings<T>>,
    bounds: ResMut<CurrentBounds>,
    line_material: Res<ModelMaterial<LineMaterial>>,
) {
    let ctx = contexts.ctx_mut();
    egui::TopBottomPanel::top("Toolbar")
        .resizable(false)
        .show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                let edges_tint = if wireframe_config.global { 255 } else { TINT };
                let bounds_tint = if view_settings.show_bounds { 255 } else { TINT };
                if ui
                    .add(
                        egui::widgets::ImageButton::new(egui::load::SizedTexture::new(
                            icons.show_bounds,
                            [24.0, 24.0],
                        ))
                        .frame(true)
                        .tint(Color32::from_gray(bounds_tint)),
                    )
                    .on_hover_text("Show bounds")
                    .clicked()
                {
                    view_settings.show_bounds = !view_settings.show_bounds;
                    add_remove_bounds_in_scene(
                        bounds,
                        view_settings,
                        &mut commands,
                        &mut meshes,
                        line_material,
                    );
                };
                if ui
                    .add(
                        egui::widgets::ImageButton::new(egui::load::SizedTexture::new(
                            icons.show_edges,
                            [24.0, 24.0],
                        ))
                        .frame(true)
                        .tint(Color32::from_gray(edges_tint)),
                    )
                    .on_hover_text("Show mesh edges [E]")
                    .clicked()
                {
                    wireframe_config.global = !wireframe_config.global;
                };
            });
            ui.add_space(2.);
        });
    egui::TopBottomPanel::bottom("OutputLogs")
        .resizable(true)
        .default_height(100.)
        .show(ctx, |ui| {
            ui.add_space(5.);
            ui.horizontal(|ui| {
                ui.with_layout(Layout::left_to_right(egui::Align::Min), |ui| {
                    ui.heading("Log");
                });
                ui.with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
                    if ui.button("Clear").clicked() {
                        let mut logs = log_handle
                            .messages
                            .lock()
                            .expect("Should be able to access log list mutex");
                        logs.clear();
                    };
                })
            });
            egui::Frame::dark_canvas(ui.style())
                .outer_margin(0.)
                .show(ui, |ui| {
                    render_logging_panel(ui, log_handle.messages.clone());
                });
        });
}

fn render_logging_panel(ui: &mut Ui, log_handle: Arc<Mutex<Vec<String>>>) {
    if let Ok(logs) = log_handle.lock() {
        let style = ui
            .style()
            .text_styles
            .get(&egui::TextStyle::Body)
            .expect("Should be here")
            .clone();
        ScrollArea::vertical()
            .auto_shrink([false; 2])
            .stick_to_bottom(true)
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    for log in logs.iter() {
                        let parts: Vec<&str> = log.split(" | ").collect();
                        if parts.len() == 3 {
                            let timestamp = parts[0];
                            let level = parts[1];
                            let message = parts[2];

                            let color = get_log_color(level);

                            let format = egui::TextFormat {
                                font_id: style.clone(),
                                ..Default::default()
                            };
                            let mut layout_job = LayoutJob::default();
                            layout_job.append(timestamp, 0.0, format.clone());
                            layout_job.append(" ", 0.0, format.clone()); // Space between parts
                            layout_job.append(
                                level,
                                0.0,
                                egui::TextFormat {
                                    font_id: style.clone(),
                                    color,
                                    ..Default::default()
                                },
                            );
                            layout_job.append(" ", 0.0, format.clone()); // Space between parts
                            layout_job.append(message, 0.0, format.clone());
                            ui.label(layout_job);
                        }
                    }
                    ui.label("");
                });
            });
    }
}

fn get_log_color(level: &str) -> Color32 {
    match level {
        "ERROR" => Color32::RED,
        "WARN " => Color32::YELLOW,
        "INFO " => Color32::DARK_GRAY,
        "DEBUG" => Color32::BLUE,
        _ => Color32::WHITE,
    }
}

pub fn add_remove_bounds_in_scene<T: Send + Sync + 'static + Float>(
    mut current_bounds: ResMut<CurrentBounds>,
    view_settings: ResMut<ViewSettings<T>>,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<bevy::prelude::Mesh>>,
    material: Res<ModelMaterial<LineMaterial>>,
) {
    if !view_settings.show_bounds {
        if let Some(bounds) = current_bounds.0 {
            commands.entity(bounds).despawn();
            current_bounds.0 = None;
        }
    } else {
        if let Some(bounds) = view_settings.bounds {
            if let Some(bounds) = current_bounds.0 {
                commands.entity(bounds).despawn();
            }

            let lines: Vec<[f32; 3]> = bounds
                .as_wireframe()
                .iter()
                .map(|line| {
                    [
                        [
                            line.start.x.to_f32().unwrap(),
                            line.start.y.to_f32().unwrap(),
                            line.start.z.to_f32().unwrap(),
                        ],
                        [
                            line.end.x.to_f32().unwrap(),
                            line.end.y.to_f32().unwrap(),
                            line.end.z.to_f32().unwrap(),
                        ],
                    ]
                })
                .flatten()
                .collect();

            let mesh = bevy::prelude::Mesh::new(
                PrimitiveTopology::LineList,
                RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
            )
            .with_inserted_attribute(bevy::prelude::Mesh::ATTRIBUTE_POSITION, lines);

            let bounds_entity = commands
                .spawn(MaterialMeshBundle {
                    mesh: meshes.add(mesh),
                    material: material.0.clone(),
                    ..default()
                })
                .id();

            current_bounds.0 = Some(bounds_entity);
        }
    }
}

#[derive(Asset, TypePath, Default, AsBindGroup, Debug, Clone)]
pub struct LineMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
}

const SHADER_ASSET_PATH: &str = "shaders/line_material.wgsl";

impl Material for LineMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        // This is the important part to tell bevy to render this material as a line between vertices
        descriptor.primitive.polygon_mode = PolygonMode::Line;
        Ok(())
    }
}
