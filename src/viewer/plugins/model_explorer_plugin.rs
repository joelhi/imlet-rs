use bevy::{
    app::{App, Plugin, Startup, Update},
    asset::Assets,
    pbr::MaterialMeshBundle,
    prelude::{Commands, IntoSystemConfigs, Res, ResMut, Resource},
    utils::default,
};
use bevy_egui::{
    egui::{
        self, emath::Numeric, load::SizedTexture, Color32, Id, ImageButton, Layout, Response,
        Stroke, TextureId, Ui,
    },
    EguiContexts,
};
use bevy_normal_material::prelude::NormalMaterial;
use log::{debug, error, info};
use num_traits::Float;
use rfd::FileDialog;
use serde::Serialize;

use crate::{
    algorithms::marching_cubes::generate_iso_surface,
    types::{
        self,
        computation::{
            components::{
                function_components::{PUBLIC_FUNCTION_COMPONENTS, PUBLIC_GEOMETRY_COMPONENTS},
                operation_components::PUBLIC_OPERATIONS,
                Component, Data, DataType,
            },
            ImplicitModel, ModelError,
        },
        geometry::{BoundingBox, Mesh, Vec3},
    },
    utils::math_helper::Pi,
    viewer::{
        raw_mesh_data::RawMeshData,
        utils::{build_mesh_from_data, custom_dnd_drag_source},
    },
};

use super::{
    add_remove_bounds_in_scene, logging_panel, CurrentBounds, CurrentMeshEntity, Icons,
    LineMaterial, ModelMaterial, ViewSettings,
};

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
    T: Float + Send + Sync + Numeric + 'static + Pi + Serialize, // Ensure T meets the required constraints (adjust as needed).
{
    fn build(&self, app: &mut App) {
        let val = T::from(50.).expect("Should be able to convert the value to T");
        let config = Config {
            cell_size: T::one(),
            bounds: BoundingBox::new(Vec3::new(-val, -val, -val), Vec3::new(val, val, val)),
            output: None,
            smoothing_iter: 1,
            smoothing_factor: T::from(0.75).expect("Should be able to convert value to T"),
        };

        app.insert_resource(AppModel::new(ImplicitModel::<T>::new()))
            .insert_resource(config)
            .insert_resource(EditingState::default())
            .add_systems(Startup, init_bounds::<T>)
            .add_systems(Update, (imlet_model_panel::<T>).before(logging_panel::<T>));
    }
}

fn init_bounds<T: Send + Sync + 'static + Clone>(
    mut view_settings: ResMut<ViewSettings<T>>,
    config: Res<Config<T>>,
) {
    view_settings.bounds = Some(config.bounds.clone());
}

#[derive(Resource)]
pub struct AppModel<T: Float + Send + Sync + Serialize + 'static + Pi> {
    pub model: ImplicitModel<T>,
    pub component_order: Vec<String>,
}

impl<T: Float + Send + Sync + Serialize + 'static + Pi> AppModel<T> {
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

#[derive(Default, Resource)]
struct EditingState {
    item_name: String,
    edit_text: String,
    editing: bool,
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

fn imlet_model_panel<T: Float + Send + Sync + Numeric + 'static + Pi + Serialize>(
    mut contexts: EguiContexts,
    mut model: ResMut<AppModel<T>>,
    mut config: ResMut<Config<T>>,
    icons: Res<Icons>,
    mut view_settings: ResMut<ViewSettings<T>>,
    current_bounds: ResMut<CurrentBounds>,
    current_mesh_entity: ResMut<CurrentMeshEntity>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<bevy::prelude::Mesh>>,
    line_material: Res<ModelMaterial<LineMaterial>>,
    normal_material: Res<ModelMaterial<NormalMaterial>>,
    mut editing_state: ResMut<EditingState>,
) {
    let ctx = contexts.ctx_mut();
    let mut components = model.component_order.clone();

    egui::SidePanel::left("Left")
        .resizable(false)
        .min_width(350.)
        .show(ctx, |ui| {
            ui.heading("Imlet model");
            ui.separator();
            if render_computation_section(ui, &mut config) {
                view_settings.bounds = Some(config.bounds);
                add_remove_bounds_in_scene(
                    current_bounds,
                    view_settings,
                    &mut commands,
                    &mut meshes,
                    line_material,
                );
            };
            ui.separator();
            ui.add_space(5.);
            if render_components(
                ui,
                &mut components,
                &mut model,
                &mut config,
                &icons,
                &mut editing_state,
            ) {
                generate_mesh(
                    commands,
                    normal_material,
                    meshes,
                    &model.model,
                    &config,
                    current_mesh_entity,
                );
            }
        });

    model.component_order = components;
}

fn render_components<T: Float + Send + Sync + Numeric + 'static + Pi + Serialize>(
    ui: &mut Ui,
    components: &mut Vec<String>,
    model: &mut ResMut<AppModel<T>>,
    config: &mut ResMut<Config<T>>,
    icons: &Res<Icons>,
    editing_state: &mut ResMut<EditingState>,
) -> bool {
    let recompute = render_component_menus(ui, &mut model.model, components, icons);

    egui::ScrollArea::vertical()
        .auto_shrink([false; 2])
        .show(ui, |ui| {
            let mut from: Option<usize> = None;
            let mut to: Option<usize> = None;

            let frame = egui::Frame::dark_canvas(ui.style()).inner_margin(4.0);
            let default_fill = Color32::from_rgb(35, 35, 35);
            let selected_fill = Color32::from_rgb(10, 50, 65);
            let none_string = "None".to_string();
            let mut removed = (false, "None");
            let mut renamed = (false, "None".to_string(), "None".to_string());
            let cloned_components = components.clone();
            let (_, dropped_payload) = ui.dnd_drop_zone::<usize, ()>(frame, |ui| {
                ui.set_min_width(ui.available_width());
                if components.len() == 0 {
                    ui.label("No components in model. Add one from the bar above.");
                }
                for (row_idx, item) in cloned_components.iter().enumerate() {
                    let mut change = InputChange::None();
                    let inputs = model.model.get_inputs(item).cloned();
                    let component = model.model.get_component_mut(item).unwrap();
                    let current_icon = icons.component_icon(component);
                    let item_id = Id::new(("drag_drop_source", row_idx));
                    let item_location = row_idx;
                    let response = custom_dnd_drag_source(ui, item_id, item_location, |ui| {
                        let mut all_responses = Vec::new();
                        let mut selected = config.output.as_ref().unwrap_or(&none_string).clone();
                        let current_fill = if *item == *selected {
                            selected_fill
                        } else {
                            default_fill
                        };
                        let mut item_copy = item.clone();
                        let delete = render_collapsible_with_icon(
                            ui,
                            &mut item_copy,
                            component,
                            &mut all_responses,
                            current_icon,
                            icons,
                            &mut change,
                            &cloned_components,
                            inputs,
                            current_fill,
                            &mut selected,
                            editing_state,
                        );

                        if item_copy.as_str() != item.as_str() {
                            renamed = (true, item.to_owned(), item_copy);
                        }
                        config.output = Some(selected);

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
            if renamed.0 {
                if let Some(pos) = components.iter().position(|x| *x == renamed.1) {
                    match model.model.rename_component(&renamed.1, &renamed.2) {
                        Ok(_) => {
                            components[pos] = renamed.2;
                        }
                        Err(error) => error!("{}", error),
                    };
                }
            }
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

                if to > from {
                    to -= 1;
                }
                let item = components.remove(from);

                components.insert(to, item);

                model.component_order = components.clone();
            }
        });

    recompute
}

fn render_component_menus<T: Float + Send + Sync + Numeric + 'static + Pi + Serialize>(
    ui: &mut Ui,
    implicit_model: &mut ImplicitModel<T>,
    components: &mut Vec<String>,
    icons: &Res<Icons>,
) -> bool {
    let mut recompute = false;
    let available_funcs = PUBLIC_FUNCTION_COMPONENTS;
    let available_geo_funcs = PUBLIC_GEOMETRY_COMPONENTS;
    let available_ops = PUBLIC_OPERATIONS;
    egui::menu::bar(ui, |ui| {
        ui.with_layout(Layout::left_to_right(egui::Align::Min), |ui| {
            ui.menu_image_button(SizedTexture::new(icons.add, [16., 16.]), |ui| {
                ui.menu_button("Primitives", |ui| {
                    for &function in available_geo_funcs {
                        if ui.button(format!("{:?}", function)).clicked() {
                            let function_component = function.create_default();
                            let result = implicit_model
                                .add_component(function_component.type_name(), function_component);
                            match result {
                                Ok(tag) => components.push(tag.to_owned()),
                                Err(error) => error!("{}", error),
                            }
                            ui.close_menu();
                        };
                    }
                });

                ui.menu_button("Functions", |ui| {
                    for &function in available_funcs {
                        if ui.button(format!("{:?}", function)).clicked() {
                            let function_component = function.create_default();
                            let result = implicit_model
                                .add_component(function_component.type_name(), function_component);
                            match result {
                                Ok(tag) => components.push(tag.to_owned()),
                                Err(error) => error!("{}", error),
                            }
                            ui.close_menu();
                        };
                    }
                });

                ui.menu_button("Operations", |ui| {
                    for &operation in available_ops {
                        let operation_name = format!("{:?}", operation);
                        if ui.button(operation_name).clicked() {
                            let component = operation.create_default();
                            let result =
                                implicit_model.add_component(component.type_name(), component);
                            match result {
                                Ok(tag) => components.push(tag),
                                Err(error) => error!("{}", error),
                            }
                            ui.close_menu();
                        };
                    }
                });
                ui.menu_button("Values", |ui| {
                    if ui.button("Constant").clicked() {
                        let result =
                            implicit_model.add_component("Value", Component::Constant(T::zero()));
                        match result {
                            Ok(tag) => components.push(tag),
                            Err(error) => error!("{}", error),
                        }
                    };
                });
            })
            .response
            .on_hover_text("Add new component.");
        });
        ui.with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
            if ui
                .add(
                    egui::widgets::ImageButton::new(egui::load::SizedTexture::new(
                        icons.compute_icon,
                        [16.0, 16.0],
                    ))
                    .frame(true),
                )
                .on_hover_text("Compute model")
                .clicked()
            {
                recompute = true;
            };
            if ui
                .add(
                    egui::widgets::ImageButton::new(egui::load::SizedTexture::new(
                        icons.export,
                        [16.0, 16.0],
                    ))
                    .frame(true),
                )
                .on_hover_text("Export")
                .clicked()
            {
                info!("Export clicked");
            };
        });
    });

    recompute
}

fn render_computation_section<T: Float + Send + Sync + Numeric + 'static>(
    ui: &mut Ui,
    config: &mut ResMut<Config<T>>,
) -> bool {
    let bounds_changed = render_computation_controls(ui, config);
    ui.add_space(5.);

    return bounds_changed;
}

fn render_computation_controls<T: Float + Send + Sync + 'static + Numeric>(
    ui: &mut Ui,
    config: &mut Config<T>,
) -> bool {
    let mut bounds_updated = false;
    egui::CollapsingHeader::new("Model space")
        .id_salt("Model space")
        .default_open(true)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                    ui.label("Min Coordinate:");
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    if ui
                        .add(egui::DragValue::new(&mut config.bounds.min.x).speed(0.1))
                        .changed()
                    {
                        bounds_updated = true;
                    };
                    ui.label("x:");

                    if ui
                        .add(egui::DragValue::new(&mut config.bounds.min.y).speed(0.1))
                        .changed()
                    {
                        bounds_updated = true;
                    };
                    ui.label("y:");

                    if ui
                        .add(egui::DragValue::new(&mut config.bounds.min.z).speed(0.1))
                        .changed()
                    {
                        bounds_updated = true;
                    };
                    ui.label("z:");
                });
            });

            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                    ui.label("Max Coordinate:");
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    if ui
                        .add(egui::DragValue::new(&mut config.bounds.max.x).speed(0.1))
                        .changed()
                    {
                        bounds_updated = true;
                    };
                    ui.label("x:");

                    if ui
                        .add(egui::DragValue::new(&mut config.bounds.max.y).speed(0.1))
                        .changed()
                    {
                        bounds_updated = true;
                    };
                    ui.label("y:");

                    if ui
                        .add(egui::DragValue::new(&mut config.bounds.max.z).speed(0.1))
                        .changed()
                    {
                        bounds_updated = true;
                    };
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
        });

    ui.separator();

    egui::CollapsingHeader::new("Smoothing")
        .id_salt("Smoothing")
        .default_open(true)
        .show(ui, |ui| {
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
        });

    return bounds_updated;
}

fn render_inputs(
    ui: &mut egui::Ui,
    input_names: &[&str],
    inputs: &[Option<String>],
    component_name: &str,
    components: &[String],
) -> (InputChange, Vec<egui::Response>) {
    let mut change = InputChange::None();
    let mut input_responses = Vec::new();
    for (i, input) in inputs.iter().enumerate() {
        let response = ui
            .horizontal(|ui| {
                ui.label(format!("{}:", input_names[i]));

                let current_input_name = match input {
                    Some(name) => name.to_string(),
                    None => "None".to_string(),
                };

                let mut selected_input = current_input_name.clone();

                // Capture the response of the ComboBox
                egui::ComboBox::from_id_salt(format!("Select input for {}, {}", component_name, i))
                    .selected_text(&selected_input)
                    .show_ui(ui, |ui| {
                        let mut combo_responses = Vec::new();

                        // Iterate over available components
                        for available_component in components.iter() {
                            if available_component == component_name {
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
                    .response
            })
            .inner;

        input_responses.push(response);
    }

    (change, input_responses)
}

fn render_parameters<T: Float + Send + Sync + 'static + Numeric + Serialize + Pi>(
    ui: &mut egui::Ui,
    component: &mut Component<T>,
    component_name: &str,
    icons: &Icons,
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
                Data::File(text) => {
                    egui::menu::bar(ui, |ui| {
                        ui.label(param.name);
                        let mut value = text.clone();

                        let response = ui.add(
                            egui::TextEdit::singleline(&mut value)
                                .desired_width(140.)
                                .clip_text(true),
                        );

                        let show_response = ui
                            .add(
                                egui::widgets::ImageButton::new(egui::load::SizedTexture::new(
                                    icons.show_edges,
                                    [12.0, 12.0],
                                ))
                                .frame(true)
                                .rounding(2.),
                            )
                            .on_hover_text("Show");

                        if show_response.clicked() {
                            info!("Show mesh");
                        }

                        let load_response = ui
                            .add(
                                egui::widgets::ImageButton::new(egui::load::SizedTexture::new(
                                    icons.more_options,
                                    [12.0, 12.0],
                                ))
                                .frame(true)
                                .rounding(2.),
                            )
                            .on_hover_text("Select file.");

                        if load_response.clicked() {
                            if let Some(path) = FileDialog::new().pick_file() {
                                component.set_parameter(
                                    param.name,
                                    Data::File(path.display().to_string()),
                                );
                            }
                        }

                        param_responses.push(load_response);
                        param_responses.push(show_response);
                        param_responses.push(response);
                    });
                }
                Data::EnumValue(selected) => {
                    if let DataType::Enum(options) = param.data_type {
                        ui.label(param.name);
                        // Capture the response of the ComboBox
                        ui.horizontal(|ui| {
                            let response = egui::ComboBox::from_id_salt(format!(
                                "Select input for {}, {}",
                                component.type_name(),
                                component_name
                            ))
                            .selected_text(selected.clone())
                            .show_ui(ui, |ui| {
                                let mut combo_responses = Vec::new();
                                let mut selected_clone = selected.clone();
                                for &option in options {
                                    let item_response = ui.selectable_value(
                                        &mut selected_clone,
                                        option.to_string(),
                                        option.to_string(),
                                    );

                                    combo_responses.push(item_response.clone());

                                    if item_response.clicked() {
                                        component.set_parameter(
                                            param.name,
                                            Data::EnumValue(selected_clone.to_string()),
                                        );
                                    }
                                }
                                param_responses.extend(combo_responses);
                            })
                            .response;
                            param_responses.push(response);
                        });
                    }
                }
            });

            ui.add_space(5.0);
        }
    }

    param_responses
}

pub fn generate_mesh<T: Float + Send + Sync + 'static + Serialize + Pi>(
    mut commands: Commands,
    material: Res<ModelMaterial<NormalMaterial>>,
    mut meshes: ResMut<Assets<bevy::prelude::Mesh>>,
    model: &ImplicitModel<T>,
    config: &ResMut<Config<T>>,
    mut current_mesh_entity: ResMut<CurrentMeshEntity>,
) {
    info!("---");
    info!(
        "Generating output for node {}",
        config.output.clone().unwrap_or("None".to_string())
    );
    let result = if let Some(target) = &config.output {
        model.generate_field(target, &config.bounds, config.cell_size)
    } else {
        Result::Err(ModelError::MissingOutput())
    };

    if let Some(entity) = current_mesh_entity.0 {
        // Remove current mesh
        commands.entity(entity).despawn();
        current_mesh_entity.0 = None;
    }

    match result {
        Ok(mut field) => {
            if let Some(entity) = current_mesh_entity.0 {
                commands.entity(entity).despawn();
            }

            field.smooth_par(
                config.smoothing_factor,
                config.smoothing_iter.try_into().unwrap(),
            );

            field.padding(T::one());

            let mesh = Mesh::from_triangles(&generate_iso_surface(&field, T::zero()), false);

            let bevy_mesh = build_mesh_from_data(RawMeshData::from_mesh(&mesh.convert::<f32>()));

            let mesh_entity = commands
                .spawn(MaterialMeshBundle {
                    mesh: meshes.add(bevy_mesh),
                    material: material.0.clone(),
                    ..default()
                })
                .id();

            current_mesh_entity.0 = Some(mesh_entity);
            info!("Successfully generated output.")
        }
        Err(err) => {
            error!("{}", err)
        }
    }
}

// Usage in the UI function
fn render_collapsible_with_icon<T: Float + Send + Sync + 'static + Numeric + Serialize + Pi>(
    ui: &mut Ui,
    item: &mut String,
    component: &mut Component<T>,
    all_responses: &mut Vec<Response>,
    icon: &TextureId,
    icons: &Icons,
    change: &mut InputChange,
    components: &[String],
    inputs: Option<Vec<Option<String>>>,
    fill: Color32,
    current_selection: &mut String,
    editing_state: &mut ResMut<EditingState>,
) -> bool {
    let mut delete = false;
    egui::Frame::group(ui.style())
        .stroke(Stroke::new(2., Color32::from_rgb(170, 170, 170)))
        .fill(fill)
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.add_space(1.5);

            egui::menu::bar(ui, |ui| {
                ui.with_layout(Layout::left_to_right(egui::Align::Min), |ui| {
                    ui.add(egui::widgets::Image::new(egui::load::SizedTexture::new(
                        *icon,
                        [16.0, 16.0],
                    )));
                    let heading_resp = ui
                        .heading(format!("{} [{}]", item, component.type_name()));
                    all_responses.push(heading_resp);
                });

                ui.with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
                   let r1 = ui.menu_image_button(egui::load::SizedTexture::new(
                        icons.more_options,
                        [16.0, 16.0],
                    ), |ui|{
                        if ui.button("Rename").clicked() {
                            if !editing_state.editing{
                                editing_state.editing = true;
                                editing_state.item_name = item.to_string();
                                editing_state.edit_text = item.to_string();
                            }else{
                                error!("Already editing component {}. Please finish before opening a new dialog.", editing_state.item_name);
                            }
                        }

                        if ui.button("Delete").clicked(){
                            delete = true;
                            ui.close_menu();
                        }
                    }).response;

                    if editing_state.editing && editing_state.item_name.as_str() == item{
                        if let Some(name) = show_text_input_window(ui, editing_state) {
                            info!("Renamed component {} to {}", item, name);
                            *item = name;
                        }
                    }

                    let button_icon = if current_selection == item {
                        icons.checked
                    } else {
                        icons.unchecked
                    };
                    let r2 = ui.add(
                        ImageButton::new(egui::load::SizedTexture::new(
                            button_icon,
                            [14.0, 14.0])).rounding(4.)
                    );

                    if r2.clicked() {
                        *current_selection = item.to_string();
                    }
                    all_responses.push(r1);
                    all_responses.push(r2);
                });
            });

            let response = egui::CollapsingHeader::new("Inputs")
                .id_salt(format!("{} [{}]", item, component.type_name()))
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
                    let param_responses = render_parameters(ui, component, item, icons);

                    all_responses.extend_from_slice(&param_responses);
                });

            ui.add_space(1.5);
            all_responses.push(response.header_response);
        });

    delete
}

fn show_text_input_window(
    ui: &mut egui::Ui,
    editing_state: &mut ResMut<EditingState>,
) -> Option<String> {
    let mut result = None;
    if editing_state.editing {
        let screen_rect = ui.ctx().screen_rect();

        let mut center_pos = ui.next_widget_position();
        center_pos.x += 75.;
        let blackout_layer_id =
            egui::LayerId::new(egui::Order::Background, egui::Id::new("blackout_layer"));
        let blackout_painter = ui.ctx().layer_painter(blackout_layer_id);
        blackout_painter.rect_filled(screen_rect, 0.0, egui::Color32::from_black_alpha(150));

        egui::Area::new(Id::new("Popup area"))
            .order(egui::Order::Foreground)
            .fixed_pos(center_pos)
            .show(ui.ctx(), |ui| {
                egui::Frame::window(&ui.ctx().style()).show(ui, |ui| {
                    ui.label("Enter the new name:");
                    ui.text_edit_singleline(&mut editing_state.edit_text);

                    if ui.button("OK").clicked() {
                        result = Some(editing_state.edit_text.clone());
                        editing_state.edit_text.clear();
                        editing_state.editing = false;
                    }

                    if ui.button("Cancel").clicked() {
                        editing_state.edit_text.clear();
                        editing_state.editing = false;
                    }
                });
            });
    }

    result
}
