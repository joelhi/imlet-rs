use std::{collections::VecDeque, sync::{Arc, Mutex}};

use bevy::{
    app::{App, Plugin, Update}, asset::Assets, input::ButtonInput, log::{tracing_subscriber::Layer, BoxedLayer, LogPlugin}, pbr::MaterialMeshBundle, prelude::{default, Commands, KeyCode, Res, ResMut, Resource, Transform}
};
use bevy_egui::{
    egui::{self, emath::Numeric, text::LayoutJob, Align, Color32, Id, Layout, Response, RichText, ScrollArea, Sense, Stroke, TextureId, Ui, Vec2},
    EguiContexts, EguiPlugin,
};
use bevy_normal_material::prelude::NormalMaterial;
use log::{debug, error, info};
use num_traits::Float;

use crate::{
    algorithms::marching_cubes::generate_iso_surface,
    types::{
        self,
        computation::{Component, Data, ImplicitModel},
        geometry::{BoundingBox, Mesh, Vec3},
    },
    viewer::{
        custom_layer::{CustomLayer, LogMessages}, raw_mesh_data::RawMeshData, utils::{build_mesh_from_data, custom_dnd_drag_source}
    },
};

use super::{CurrentMeshEntity, Icons};

pub struct ModelExplorerPlugin<T> {
    _marker: std::marker::PhantomData<T>,
}

// Implement a default constructor for ModelExplorerPlugin to make instantiation easier.
impl<T> ModelExplorerPlugin<T> {
    pub fn new() -> Self {
        ModelExplorerPlugin {
            _marker: std::marker::PhantomData,
        }
    }
}

// Implement the Plugin trait for ModelExplorerPlugin with a generic type T.
impl<T> Plugin for ModelExplorerPlugin<T>
where
    T: Float + Send + Sync + Numeric + 'static, // Ensure T meets the required constraints (adjust as needed).
{
    fn build(&self, app: &mut App) {
        let val = T::from(100.).expect("Should be able to convert the value to T");
        let config = Config {
            cell_size: T::one(),
            bounds: BoundingBox::new(Vec3::origin(), Vec3::new(val, val, val)),
            output: None,
            smoothing_iter: 1,
            smoothing_factor: T::from(0.75).expect("Should be able to convert value to T"),
        };

        app.insert_resource(AppModel::new(ImplicitModel::<T>::new()))
            .insert_resource(config)
            .add_plugins(EguiPlugin)
            .add_systems(Update, imlet_ui_panel::<T>)
            .add_systems(Update, compute_fast::<T>);

        app.add_plugins(
            LogPlugin {
                custom_layer,
                ..default()
            }
        );

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
            Box::new(CustomLayer{log_messages: inner_log_clone})
    ]))
}

#[derive(Resource)]
pub struct AppModel<T> {
    pub model: ImplicitModel<T>,
    pub component_order: Vec<String>,
}

impl<T: Float> AppModel<T> {
    pub fn new(model: ImplicitModel<T>) -> Self {
        let component_order: Vec<String> = model
            .all_components()
            .iter()
            .map(|c| c.0.to_owned())
            .collect();
        AppModel {
            model,
            component_order,
        }
    }
}

#[derive(Resource)]
pub struct Config<T> {
    pub cell_size: T,
    pub bounds: BoundingBox<T>,
    pub output: Option<String>,
    pub smoothing_iter: usize,
    pub smoothing_factor: T,
}

enum InputChange {
    Add(String, String, usize),
    Remove(String, usize),
    None(),
}

fn imlet_ui_panel<T: Float + Send + Sync + Numeric + 'static>(
    mut contexts: EguiContexts,
    mut model: ResMut<AppModel<T>>,
    mut config: ResMut<Config<T>>,
    commands: Commands,
    materials: ResMut<Assets<NormalMaterial>>,
    meshes: ResMut<Assets<bevy::prelude::Mesh>>,
    current_mesh_entity: ResMut<CurrentMeshEntity>,
    icons: Res<Icons>,
    log_handle: Res<LogMessages>
) {
    let ctx = contexts.ctx_mut();
    let mut components = model.component_order.clone();

    egui::SidePanel::left("Left")
        .resizable(false)
        .min_width(350.)
        .show(ctx, |ui| {
            render_computation_section(
                ui,
                &mut config,
                &components,
                &model,
                commands,
                materials,
                meshes,
                current_mesh_entity,
            );

            render_components(ui, &mut components, &mut model, &mut config, &icons);
        });
        
        egui::TopBottomPanel::bottom("OutputLogs")
        .resizable(true)
        .default_height(100.)
        .show(ctx, |ui| {
            render_logging_panel(ui, log_handle.messages.clone());
        });

    model.component_order = components;
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

fn render_logging_panel(ui: &mut Ui, log_handle: Arc<Mutex<Vec<String>>>) {
    if let Ok(logs) = log_handle.lock() {
        let style = ui.style().text_styles.get(&egui::TextStyle::Body).expect("Should be here").clone();
        ScrollArea::vertical()
            .auto_shrink([false; 2])
            .stick_to_bottom(true)
            .show(ui, |ui| {
                ui.vertical( |ui| {
                    for log in logs.iter() {
                        let parts: Vec<&str> = log.split(" | ").collect();
                        if parts.len() == 3 {
                            let timestamp = parts[0];
                            let level = parts[1];
                            let message = parts[2];

                            let color = get_log_color(level);

                            let format = egui::TextFormat{font_id: style.clone(), ..Default::default()};
                            let mut layout_job = LayoutJob::default();
                            layout_job.append(timestamp, 0.0, format.clone());
                            layout_job.append(" ", 0.0, format.clone()); // Space between parts
                            layout_job.append(level, 0.0, egui::TextFormat { font_id: style.clone(), color: color, ..Default::default() });
                            layout_job.append(" ", 0.0, format.clone()); // Space between parts
                            layout_job.append(message, 0.0, format.clone());
                            ui.label(layout_job);
                        }
                    }
                });
            });
    }
}

fn render_components<T: Float + Send + Sync + Numeric + 'static>(
    ui: &mut Ui,
    components: &mut Vec<String>,
    model: &mut ResMut<AppModel<T>>,
    config: &mut ResMut<Config<T>>,
    icons: &Res<Icons>,
) {
    ui.heading("Components");

    egui::ScrollArea::vertical()
        .auto_shrink([false; 2])
        .show(ui, |ui| {
            let mut from: Option<usize> = None;
            let mut to: Option<usize> = None;

            let frame = egui::Frame::dark_canvas(ui.style()).inner_margin(4.0);
            let default_fill = Color32::from_rgb(35, 35, 35);
            let selected_fill = Color32::from_rgb(35, 70, 65);
            let none_string = "None".to_string();
            let mut removed = (false, "None");
            let (_, dropped_payload) = ui.dnd_drop_zone::<usize, ()>(frame, |ui| {
                ui.set_min_width(ui.available_width());
                for (row_idx, item) in components.iter().enumerate() {
                    let mut change = InputChange::None();
                    let inputs = model.model.get_inputs(item).cloned();
                    let component = model.model.get_component_mut(item).unwrap();
                    let current_icon = icons.component_icon(&component);
                    let item_id = Id::new(("my_drag_and_drop_demo", row_idx));
                    let item_location = row_idx;
                    let response = custom_dnd_drag_source(ui, item_id, item_location, |ui| {
                        let mut all_responses = Vec::new();
                        let selected = config.output.as_ref().unwrap_or_else(|| &none_string);
                        let current_fill = if *item == *selected {
                            selected_fill
                        } else {
                            default_fill
                        };
                        let delete = render_collapsible_with_icon(
                            ui,
                            item,
                            component,
                            &mut all_responses,
                            &current_icon,
                            icons.delete_icon(),
                            &mut change,
                            &components,
                            inputs,
                            current_fill,
                        );

                        if delete {
                            removed = (true, item);
                        }

                        ((), all_responses)
                    })
                    .response;

                    let result = match change {
                        InputChange::Add(component, source, index) => {
                            model.model.add_input(&component, &source, index)
                        }
                        InputChange::Remove(component, index) => {
                            model.model.remove_input(&component, index)
                        }
                        InputChange::None() => Ok(()),
                    };

                    match result {
                        Ok(_) => (),
                        Err(model_error) => error!("{}", model_error),
                    }

                    if let (Some(pointer), Some(hovered_payload)) = (
                        ui.input(|i| i.pointer.interact_pos()),
                        response.dnd_hover_payload::<usize>(),
                    ) {
                        let rect = response.rect;

                        let stroke = egui::Stroke::new(2.0, egui::Color32::DARK_BLUE);
                        let insert_row_idx = if *hovered_payload == item_location {
                            // We are dragged onto ourselves
                            ui.painter().hline(rect.x_range(), rect.center().y, stroke);
                            row_idx
                        } else if pointer.y < rect.center().y {
                            // Above us
                            ui.painter().hline(rect.x_range(), rect.top() - 5., stroke);
                            row_idx
                        } else {
                            // Below us
                            ui.painter()
                                .hline(rect.x_range(), rect.bottom() + 5., stroke);
                            row_idx + 1
                        };

                        if let Some(dragged_payload) = response.dnd_release_payload() {
                            from = Some(*dragged_payload);
                            to = Some(insert_row_idx);
                        }
                    }

                    ui.add_space(5.);
                }
            });

            if removed.0 {
                // Remove from model.
                if let Some(pos) = components.iter().position(|x| x == removed.1) {
                    debug!("Removed component {}", removed.1);
                    match model.model.remove_component(removed.1) {
                        Ok(_) => {
                            components.remove(pos);
                        }
                        Err(error) => error!("{}", error),
                    };
                }
            }
            if let Some(dragged_payload) = dropped_payload {
                // The user dropped onto the column, but not on any one item.
                from = Some(*dragged_payload);
                to = Some(*dragged_payload);
            }

            if let (Some(from), Some(mut to)) = (from, to) {
                debug!("Dropped Component -> From: {} To: {}", from, to);

                // Adjust `to.row` if necessary, in case the dragged element is being moved down the list
                if to > from {
                    to = to - 1; // Since removing the element will shift the indices
                }

                // Remove the element from `from.row` and store it
                let item = components.remove(from);
                // Insert the item at the new location `to.row`
                components.insert(to, item);

                model.component_order = components.clone();
            }
        });
}

fn render_computation_section<T: Float + Send + Sync + Numeric + 'static>(
    ui: &mut Ui,
    mut config: &mut ResMut<Config<T>>,
    components: &[String],
    model: &AppModel<T>,
    commands: Commands,
    materials: ResMut<Assets<NormalMaterial>>,
    meshes: ResMut<Assets<bevy::prelude::Mesh>>,
    current_mesh_entity: ResMut<CurrentMeshEntity>,
) {
    egui::TopBottomPanel::top("Top Computation")
        .resizable(false)
        .show_inside(ui, |ui| {
            // Computation controls
            render_computation_controls(ui, &mut config);

            ui.separator();

            ui.horizontal(|ui| {
                ui.with_layout(Layout::left_to_right(egui::Align::Min), |ui| {
                    ui.label("Output");
                });

                // Get the current input name
                let current_input_name = match &config.output {
                    Some(name) => name.to_string(),
                    None => "None".to_string(),
                };
                ui.with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
                    let mut selected_input: String = current_input_name.clone();
                    egui::ComboBox::from_id_source(&format!("Output"))
                        .selected_text(&selected_input)
                        .show_ui(ui, |ui| {
                            for available_component in components.iter() {
                                if ui
                                    .selectable_value(
                                        &mut selected_input,
                                        available_component.to_string(),
                                        available_component,
                                    )
                                    .clicked()
                                {
                                    config.output = Some(selected_input.clone());
                                };
                            }

                            if ui
                                .selectable_value(&mut selected_input, "None".to_string(), "None")
                                .clicked()
                            {
                                config.output = None;
                            };
                        });
                });
            });

            ui.add_space(5.);

            ui.horizontal(|ui| {
                // Button to generate mesh
                if ui.button("Generate").clicked() {
                    if let Some(target) = &config.output {
                        generate_mesh(
                            commands,
                            materials,
                            meshes,
                            &model.model,
                            &config,
                            target,
                            current_mesh_entity,
                        );
                    } else {
                        error!("Failed to generate mesh. No output selected for computation.");
                    }
                }

                // Button to generate mesh
                if ui.button("Export").clicked() {
                    error!("Exporting not yet implemented.");
                }
            });

            ui.add_space(10.);
        });
}

fn render_computation_controls<T: Float + Send + Sync + 'static + Numeric>(
    ui: &mut Ui,
    config: &mut Config<T>,
) {
    ui.heading("Model Space");
    ui.horizontal(|ui| {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
            ui.label("Min Coordinate:");
        });
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
            ui.add(egui::DragValue::new(&mut config.bounds.min.x).speed(0.1));
            ui.label("x:");

            ui.add(egui::DragValue::new(&mut config.bounds.min.y).speed(0.1));
            ui.label("y:");

            ui.add(egui::DragValue::new(&mut config.bounds.min.z).speed(0.1));
            ui.label("z:");
        });
    });

    ui.horizontal(|ui| {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
            ui.label("Max Coordinate:");
        });
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
            ui.add(egui::DragValue::new(&mut config.bounds.max.x).speed(0.1));
            ui.label("x:");

            ui.add(egui::DragValue::new(&mut config.bounds.max.y).speed(0.1));
            ui.label("y:");

            ui.add(egui::DragValue::new(&mut config.bounds.max.z).speed(0.1));
            ui.label("z:");
        });
    });

    ui.horizontal(|ui| {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
            ui.label("Cell Size:");
        });
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
            let cell_size = &mut config.cell_size;
            ui.add(egui::DragValue::new(cell_size).speed(0.1));

            config.cell_size =
                cell_size.max(T::from(0.1).expect("Should be able to convert 0.1 to T"));
        });
    });

    ui.separator();

    ui.heading("Smoothing");

    ui.horizontal(|ui| {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
            ui.label("Iterations:");
        });
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
            ui.add(egui::DragValue::new(&mut config.smoothing_iter).speed(0.1));
        });
    });

    // Input for smoothing factor
    ui.horizontal(|ui| {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
            ui.label("Factor:");
        });
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
            let smmoothing_factor = &mut config.smoothing_factor;
            ui.add(egui::DragValue::new(smmoothing_factor).speed(0.1));
            config.smoothing_factor = smmoothing_factor.max(T::zero()).min(T::one());
        });
    });
}

fn render_inputs(
    ui: &mut egui::Ui,
    input_names: &[&str],
    inputs: &Vec<Option<String>>,
    component_name: &str,
    components: &[String],
) -> (InputChange, Vec<egui::Response>) {
    let mut change = InputChange::None();
    let mut input_responses = Vec::new();
    for (i, input) in inputs.clone().iter().enumerate() {
        let response = ui
            .horizontal(|ui| {
                ui.label(&format!("{}:", input_names[i]));

                let current_input_name = match input {
                    Some(name) => name.to_string(),
                    None => "None".to_string(),
                };

                let mut selected_input = current_input_name.clone();

                // Capture the response of the ComboBox
                let combo_box_response = egui::ComboBox::from_id_source(&format!(
                    "Select input for {}, {}",
                    component_name, i
                ))
                .selected_text(&selected_input)
                .show_ui(ui, |ui| {
                    let mut combo_responses = Vec::new();

                    // Iterate over available components
                    for available_component in components.iter() {
                        if available_component == component_name{
                            continue;
                        }

                        let item_response = ui.selectable_value(
                            &mut selected_input,
                            available_component.to_string(),
                            available_component,
                        );
                        combo_responses.push(item_response.clone());

                        if item_response.clicked() {
                            change = InputChange::Add(
                                component_name.to_string(),
                                selected_input.clone(),
                                i,
                            );
                        }
                    }

                    let none_response =
                        ui.selectable_value(&mut selected_input, "None".to_string(), "None");
                    combo_responses.push(none_response.clone());

                    if none_response.clicked() {
                        change = InputChange::Remove(component_name.to_string(), i);
                    }

                    combo_responses
                })
                .response;

                combo_box_response
            })
            .inner;

        input_responses.push(response);
    }

    (change, input_responses)
}

fn render_parameters<T: Float + Send + Sync + 'static + Numeric>(
    ui: &mut egui::Ui,
    component: &mut Component<T>,
) -> Vec<egui::Response> {
    let parameters = component.get_parameters();
    let mut param_responses = Vec::new();

    if !parameters.is_empty() {
        for (param, data) in parameters.iter() {
            ui.horizontal(|ui| match data {
                Data::Value(val) => {
                    ui.label(param.name);

                    let mut value = *val;
                    let response = ui.add(egui::DragValue::new(&mut value).speed(0.1));
                    if response.changed() {
                        component.set_parameter(param.name, Data::Value(value));
                    }
                    param_responses.push(response);
                }

                Data::Vec3(vec3) => {
                    ui.label(param.name);
                    ui.horizontal(|ui| {
                        let mut x = vec3.x;
                        let mut y = vec3.y;
                        let mut z = vec3.z;

                        ui.label("x:");
                        let x_response = ui.add(egui::DragValue::new(&mut x).speed(0.1));
                        if x_response.changed() {
                            component.set_parameter(
                                param.name,
                                Data::Vec3(types::geometry::Vec3::new(x, y, z)),
                            );
                        }

                        ui.label("y:");
                        let y_response = ui.add(egui::DragValue::new(&mut y).speed(0.1));
                        if y_response.changed() {
                            component.set_parameter(
                                param.name,
                                Data::Vec3(types::geometry::Vec3::new(x, y, z)),
                            );
                        }

                        ui.label("z:");
                        let z_response = ui.add(egui::DragValue::new(&mut z).speed(0.1));
                        if z_response.changed() {
                            component.set_parameter(
                                param.name,
                                Data::Vec3(types::geometry::Vec3::new(x, y, z)),
                            );
                        }
                        param_responses.push(x_response);
                        param_responses.push(y_response);
                        param_responses.push(z_response);
                    });
                }
                Data::Boolean(boolean) => {
                    ui.label(param.name);
                    let mut value = *boolean;
                    let response = ui.checkbox(&mut value, "");
                    if response.changed() {
                        component.set_parameter(param.name, Data::Boolean(value));
                    }
                    param_responses.push(response);
                }
                Data::Text(text) => {
                    ui.label(param.name);
                    let mut value = text.clone();
                    let response = ui.text_edit_singleline(&mut value);
                    param_responses.push(response);
                }
            });

            ui.add_space(5.0);
        }
    }

    param_responses
}

fn generate_mesh<T: Float + Send + Sync + 'static>(
    mut commands: Commands,
    mut materials: ResMut<Assets<NormalMaterial>>,
    mut meshes: ResMut<Assets<bevy::prelude::Mesh>>,
    model: &ImplicitModel<T>,
    config: &ResMut<Config<T>>,
    target: &str,
    mut current_mesh_entity: ResMut<CurrentMeshEntity>,
) {
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
        }
        Err(err) => error!("{}", err),
    }
}

fn compute_fast<T: Float + Send + Sync + 'static + Numeric>(
    model: ResMut<AppModel<T>>,
    config: ResMut<Config<T>>,
    commands: Commands,
    materials: ResMut<Assets<NormalMaterial>>,
    meshes: ResMut<Assets<bevy::prelude::Mesh>>,
    current_mesh_entity: ResMut<CurrentMeshEntity>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Enter) {
        if let Some(target) = &config.output {
            generate_mesh(
                commands,
                materials,
                meshes,
                &model.model,
                &config,
                target,
                current_mesh_entity,
            );
        }
    }
}

// Usage in the UI function
fn render_collapsible_with_icon<T: Float + Send + Sync + 'static + Numeric>(
    ui: &mut Ui,
    item: &str,
    component: &mut Component<T>,
    all_responses: &mut Vec<Response>,
    icon: &TextureId,
    delete_icon: &TextureId,
    change: &mut InputChange,
    components: &[String],
    inputs: Option<Vec<Option<String>>>,
    fill: Color32,
) -> bool {
    let mut delete = false;
    egui::Frame::group(ui.style())
        .stroke(Stroke::new(2., Color32::from_rgb(170, 170, 170)))
        .fill(fill)
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.add_space(1.5);

            ui.horizontal(|ui| {
                ui.with_layout(Layout::left_to_right(egui::Align::Min), |ui| {
                    ui.add(egui::widgets::Image::new(egui::load::SizedTexture::new(
                        *icon,
                        [16.0, 16.0],
                    )));
                    ui.heading(format!("{} [{}]", item, component.type_name()));
                });

                ui.with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
                    let r = ui.add(
                        egui::widgets::ImageButton::new(egui::load::SizedTexture::new(
                            *delete_icon,
                            [16.0, 16.0],
                        ))
                        .frame(false),
                    );

                    if r.clicked() {
                        delete = true;
                    }
                    all_responses.push(r);
                });
            });

            let response = egui::CollapsingHeader::new("Inputs")
                .id_source(format!("{} [{}]", item, component.type_name()))
                .default_open(false)
                .show(ui, |ui| {
                    // Expose inputs
                    if let Some(inputs) = inputs {
                        let input_names = component.input_names();
                        let (input_change, inner_responses) =
                            render_inputs(ui, input_names, &inputs, item, components);
                        all_responses.extend_from_slice(&inner_responses);

                        *change = input_change;
                    }

                    ui.separator();

                    // Expose parameters
                    let param_responses = render_parameters(ui, component);

                    all_responses.extend_from_slice(&param_responses);
                });

            ui.add_space(1.5);
            all_responses.push(response.header_response);
        });

    delete
}
