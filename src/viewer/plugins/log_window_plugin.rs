use std::sync::{Arc, Mutex};

use bevy::{
    app::{App, Plugin, Update}, asset::Assets, log::{tracing_subscriber::Layer, BoxedLayer, LogPlugin}, pbr::{wireframe::WireframeConfig, MaterialMeshBundle}, prelude::{default, Commands, Res, ResMut, Transform}
};
use bevy_egui::{
    egui::{self, emath::Numeric, text::LayoutJob, Color32, Layout, ScrollArea, Ui},
    EguiContexts,
};
use bevy_normal_material::prelude::NormalMaterial;
use log::{error, info};
use num_traits::Float;

use crate::{algorithms::marching_cubes::generate_iso_surface, types::{computation::ImplicitModel, geometry::Mesh}, viewer::{custom_layer::{CustomLayer, LogMessages}, raw_mesh_data::RawMeshData, utils::build_mesh_from_data}};

use super::{AppModel, Config, CurrentMeshEntity, Icons};

pub struct LogWindowPlugin<T>{
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

const TINT: u8 = 100;

pub fn logging_panel<T: Float + Send + Sync + Numeric + 'static>(
    mut contexts: EguiContexts, 
    log_handle: Res<LogMessages>,
    model: ResMut<AppModel<T>>,
    model_config: ResMut<Config<T>>,
    mut wireframe_config: ResMut<WireframeConfig>,
    commands: Commands,
    materials: ResMut<Assets<NormalMaterial>>,
    meshes: ResMut<Assets<bevy::prelude::Mesh>>,
    current_mesh_entity: ResMut<CurrentMeshEntity>,
    icons: Res<Icons>,) {
    let ctx = contexts.ctx_mut();
    egui::TopBottomPanel::top("Toolbar")
        .resizable(false)
        .show(ctx, |ui|{
            ui.add_space(2.);
            ui.horizontal(|ui| {
                let edges_tint = if wireframe_config.global { 255 } else {TINT};
                if ui.add(
                    egui::widgets::ImageButton::new(egui::load::SizedTexture::new(
                        icons.compute_icon,
                        [24.0, 24.0],
                    )).frame(false)
                ).on_hover_text("Compute output [ENTER]").clicked(){
                    if let Some(target) = &model_config.output {
                        generate_mesh(
                            commands,
                            materials,
                            meshes,
                            &model.model,
                            &model_config,
                            target,
                            current_mesh_entity,
                        );
                    } else {
                        error!("Failed to generate mesh. No output selected for computation.");
                    }
                };
                ui.add_space(10.);
                if ui.add(
                    egui::widgets::ImageButton::new(egui::load::SizedTexture::new(
                        icons.export,
                        [24.0, 24.0],
                    )).frame(false)
                ).on_hover_text("Export").clicked(){
                    info!("Export clicked")
                };
                ui.add_space(10.);
                ui.separator();
                ui.add_space(10.);
                if ui.add(
                    egui::widgets::ImageButton::new(egui::load::SizedTexture::new(
                        icons.show_bounds,
                        [24.0, 24.0],
                    )).frame(false)
                ).on_hover_text("Show bounds [B]").clicked(){
                    info!("Show bounds clicked.")
                };
                ui.add_space(10.);
                if ui.add(
                    egui::widgets::ImageButton::new(egui::load::SizedTexture::new(
                        icons.show_edges,
                        [24.0, 24.0],
                    )).frame(false).tint(Color32::from_gray(edges_tint))
                ).on_hover_text("Show mesh edges [E]").clicked(){
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

pub fn generate_mesh<T: Float + Send + Sync + 'static>(
    mut commands: Commands,
    mut materials: ResMut<Assets<NormalMaterial>>,
    mut meshes: ResMut<Assets<bevy::prelude::Mesh>>,
    model: &ImplicitModel<T>,
    config: &ResMut<Config<T>>,
    target: &str,
    mut current_mesh_entity: ResMut<CurrentMeshEntity>,
) {
    info!("---");
    info!(
        "Generating output for node {}",
        config.output.clone().unwrap_or("None".to_string())
    );
    let result = model.generate_field(target, &config.bounds, config.cell_size);

    match result {
        Ok(mut field) => {
            if let Some(entity) = current_mesh_entity.0 {
                commands.entity(entity).despawn();
            }

            field.smooth_par(
                config.smoothing_factor,
                config.smoothing_iter.try_into().unwrap(),
            );

            let mesh = Mesh::from_triangles(&generate_iso_surface(&field, T::zero()), false);

            let bevy_mesh = build_mesh_from_data(RawMeshData::from_mesh(&mesh.convert::<f32>()));

            let target = mesh.centroid().convert::<f32>();
            let mat = materials.add(NormalMaterial {
                opacity: 1.,
                depth_bias: 0.,
                cull_mode: None,
                alpha_mode: Default::default(),
            });

            let mesh_entity = commands
                .spawn(MaterialMeshBundle {
                    mesh: meshes.add(bevy_mesh),
                    material: mat,
                    transform: Transform::from_translation(bevy::math::Vec3::new(
                        -target.x, -target.y, -target.z,
                    )),
                    ..default()
                })
                .id();

            current_mesh_entity.0 = Some(mesh_entity);
            info!("Successfully generated output.")
        }
        Err(err) => error!("{}", err),
    }
}
