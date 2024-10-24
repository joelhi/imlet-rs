use std::fmt::{format, Debug};

use bevy::{
    app::{App, Startup, Update},
    asset::Assets,
    color::Color,
    input::ButtonInput,
    math::Vec3,
    pbr::{
        wireframe::{WireframeConfig, WireframePlugin},
        MaterialMeshBundle,
    },
    prelude::{
        default, Camera3dBundle, Commands, Entity, KeyCode, OrthographicProjection, Res, ResMut,
        Resource, Transform,
    },
    render::{
        camera::ScalingMode,
        mesh::{self, PrimitiveTopology},
        render_asset::RenderAssetUsages,
        render_resource::Face,
        settings::{RenderCreation, WgpuFeatures, WgpuSettings},
        RenderPlugin,
    },
    DefaultPlugins,
};
use bevy_egui::{
    egui::{
        self, emath::Numeric, vec2, Color32, FontData, FontDefinitions, FontFamily, FontId, Grid,
        Id, InnerResponse, Response, ScrollArea, Stroke, TextStyle, Ui,
    },
    EguiContexts, EguiPlugin,
};
use bevy_normal_material::{plugin::NormalMaterialPlugin, prelude::NormalMaterial};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use log::info;
use num_traits::Float;

use crate::{
    algorithms::marching_cubes::generate_iso_surface,
    types::{
        self,
        computation::{Component, Data, ImplicitModel, ModelError},
        geometry::{BoundingBox, Mesh},
    },
};
use bevy::prelude::PluginGroup;

use super::{plugins::MeshViewerPlugin, raw_mesh_data::RawMeshData};

/// Open an interactive window which renders a mesh object with orbit controls.
///
/// # Arguments
///
/// * `mesh` - The mesh to render.
///
pub fn show_mesh<T: Float>(mesh: &Mesh<T>) {
    App::new()
        .add_plugins(MeshViewerPlugin::new(&mesh.convert::<f32>()))
        .run();
}

#[derive(Resource)]
pub struct AppModel<T> {
    model: ImplicitModel<T>,
    component_order: Vec<String>,
}

#[derive(Resource)]
pub struct Config<T> {
    pub cell_size: T,
    pub bounds: BoundingBox<T>,
    pub output: Option<String>,
    pub smoothing_iter: usize,
    pub smoothing_factor: T,
}

#[derive(Resource)]
struct CurrentMeshEntity(Option<Entity>);

impl<T: Float> AppModel<T> {
    pub fn new(model: ImplicitModel<T>, bounds: BoundingBox<T>) -> Self {
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

pub fn run_explorer<T: Float + Debug + Send + Sync + 'static + Numeric>(
    model: ImplicitModel<T>,
    bounds: &BoundingBox<T>,
) {
    let app_model = AppModel::new(model, bounds.clone());
    let config = Config {
        cell_size: T::one(),
        bounds: bounds.clone(),
        output: None,
        smoothing_iter: 1,
        smoothing_factor: T::from(0.75).expect("Should be able to convert the value to T"),
    };
    App::new()
        .insert_resource(app_model)
        .insert_resource(config)
        .insert_resource(CurrentMeshEntity(None))
        .add_plugins((
            DefaultPlugins
                // Mesh viewer
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        features: WgpuFeatures::POLYGON_MODE_LINE,
                        ..default()
                    }),
                    synchronous_pipeline_compilation: false,
                })
                .disable::<bevy::log::LogPlugin>(),
            WireframePlugin,
        ))
        // Mesh viewer
        .insert_resource(WireframeConfig {
            global: true,
            default_color: Color::WHITE,
        })
        .add_plugins(EguiPlugin)
        // Mesh viewer
        .add_systems(Startup, setup)
        // Ui setup
        .add_systems(Startup, configure_font)
        // Ui
        .add_systems(Update, test_drag_drop::<T>)
        // Mesh viewer
        .add_plugins(NormalMaterialPlugin)
        // Mesh viewer
        .add_systems(Update, update_wireframe)
        .add_systems(Update, compute_fast::<T>)
        // Mesh viewer
        .add_plugins(PanOrbitCameraPlugin)
        .run();
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Location {
    row: usize,
}

fn test_drag_drop<T: Float + Send + Sync + Numeric + 'static>(
    mut contexts: EguiContexts,
    mut model: ResMut<AppModel<T>>,
    mut config: ResMut<Config<T>>,
    commands: Commands,
    materials: ResMut<Assets<NormalMaterial>>,
    meshes: ResMut<Assets<bevy::prelude::Mesh>>,
    current_mesh_entity: ResMut<CurrentMeshEntity>,
) {
    let ctx = contexts.ctx_mut();
    let mut components = model.component_order.clone();

    egui::SidePanel::left("Left")
        .resizable(false)
        .show(ctx, |ui| {
            egui::TopBottomPanel::top("Top Computation")
                .resizable(false) // Disable resizing of the bottom panel
                .show_inside(ui, |ui| {
                    // New section below the scrollable area
                    ui.heading("Computation");

                    ui.horizontal(|ui| {
                        ui.label("Output");
                        // Get the current input name
                        let current_input_name = match &config.output {
                            Some(name) => name.to_string(),
                            None => "None".to_string(),
                        };

                        let mut selected_input = current_input_name.clone();
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
                                    .selectable_value(
                                        &mut selected_input,
                                        "None".to_string(),
                                        "None",
                                    )
                                    .clicked()
                                {
                                    config.output = None;
                                };
                            });
                    });

                    render_computation_controls(ui, &mut config);

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
                                info!("No output selected for computation.");
                            }
                        }

                        // Button to generate mesh
                        if ui.button("Export").clicked() {
                            info!("Exporting not implemented.");
                        }
                    });

                    ui.add_space(10.);
                });

            ui.heading("Components");

            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    let mut from = None;
                    let mut to = None;

                    let frame = egui::Frame::dark_canvas(ui.style()).inner_margin(4.0);
                    let default_fill = Color32::from_rgb(35, 35, 35);
                    let selected_fill = Color32::from_rgb(35, 75, 100);
                    let none_string = "None".to_string();
                    let (_, dropped_payload) = ui.dnd_drop_zone::<Location, ()>(frame, |ui| {
                        ui.set_min_size(vec2(64.0, 100.0));
                        for (row_idx, item) in components.iter().enumerate() {
                            let mut change = InputChange::None();
                            let inputs = model.model.get_inputs(item).cloned();
                            let component = model.model.get_component_mut(item).unwrap();
                            let item_id = Id::new(("my_drag_and_drop_demo", row_idx));
                            let item_location = Location { row: row_idx };
                            let response =
                                custom_dnd_drag_source(ui, item_id, item_location, |ui| {
                                    let mut all_responses = Vec::new(); // Vector to collect all responses
                                    let selected = config.output.as_ref().unwrap_or_else(|| &none_string);
                                    let current_fill = if *item == *selected { selected_fill } else { default_fill };
                                    let response = egui::Frame::group(ui.style())
                                        .stroke(Stroke::new(2., Color32::from_rgb(170, 170, 170)))
                                        .fill(current_fill)
                                        .show(ui, |ui| {
                                            ui.set_width(ui.available_width());
                                            ui.add_space(1.5);
                                            let response = egui::CollapsingHeader::new(format!(
                                                "{} [{}]",
                                                item,
                                                component.type_name()
                                            ))
                                            .default_open(true)
                                            .show(ui, |ui| {
                                                // Expose parameters
                                                let param_responses =
                                                    render_parameters(ui, component);

                                                all_responses.extend_from_slice(&param_responses);

                                                // Expose inputs
                                                if let Some(inputs) = inputs {
                                                    let (input_change, inner_responses) =
                                                        render_inputs(
                                                            ui,
                                                            &inputs,
                                                            item,
                                                            &components,
                                                        );
                                                    all_responses
                                                        .extend_from_slice(&inner_responses);

                                                    change = input_change;
                                                }
                                            });

                                            ui.add_space(1.5);
                                            all_responses.push(response.header_response);
                                        });

                                    (response, all_responses)
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
                                Err(model_error) => info!("{}", model_error),
                            }

                            if let (Some(pointer), Some(hovered_payload)) = (
                                ui.input(|i| i.pointer.interact_pos()),
                                response.dnd_hover_payload::<Location>(),
                            ) {
                                let rect = response.rect;

                                // Preview insertion:
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
                                    // The user dropped onto this item.
                                    from = Some(dragged_payload);
                                    to = Some(Location {
                                        row: insert_row_idx,
                                    });
                                }
                            }

                            ui.add_space(5.);
                        }
                    });

                    if let Some(dragged_payload) = dropped_payload {
                        // The user dropped onto the column, but not on any one item.
                        from = Some(dragged_payload.clone());
                        to = Some(Location {
                            row: dragged_payload.row,
                        });
                    }

                    if let (Some(from), Some(mut to)) = (from, to) {
                        info!("Dropped Component -> From: {} To: {}", from.row, to.row);

                        // Adjust `to.row` if necessary, in case the dragged element is being moved down the list
                        if to.row > from.row {
                            to.row -= 1; // Since removing the element will shift the indices
                        }

                        // Remove the element from `from.row` and store it
                        let item = components.remove(from.row);
                        // Insert the item at the new location `to.row`
                        components.insert(to.row, item);

                        model.component_order = components.clone();
                    }
                });
        });
}

fn render_computation_controls<T: Float + Send + Sync + 'static + Numeric>(
    ui: &mut Ui,
    config: &mut Config<T>,
) {
    // Input for cell size
    let mut cell_size = config.cell_size;
    ui.horizontal(|ui| {
        ui.label("Cell Size:");
        if ui
            .add(egui::DragValue::new(&mut cell_size).speed(0.1))
            .changed()
        {
            config.cell_size = cell_size;
        };
    });

    // Input for smoothing iter
    let mut smooth_iter = config.smoothing_iter;
    ui.horizontal(|ui| {
        ui.label("Smoothing Iterations:");
        if ui
            .add(egui::DragValue::new(&mut smooth_iter).speed(0.1))
            .changed()
        {
            config.smoothing_iter = smooth_iter;
        };
    });

    // Input for smoothing factor
    let mut smooth_factor = config.smoothing_factor;
    ui.horizontal(|ui| {
        ui.label("Smoothing Factor:");
        if ui
            .add(egui::DragValue::new(&mut smooth_factor).speed(0.1))
            .changed()
        {
            config.smoothing_factor = smooth_factor.clamp(T::zero(), T::one());
        };
    });
}

enum InputChange {
    Add(String, String, usize),
    Remove(String, usize),
    None(),
}

fn render_inputs(
    ui: &mut egui::Ui,
    inputs: &Vec<Option<String>>,
    component_name: &str,
    components: &[String],
) -> (InputChange, Vec<egui::Response>) {
    ui.label("Inputs:");

    let mut change = InputChange::None();
    let mut input_responses = Vec::new(); // To collect the input responses

    for (i, input) in inputs.clone().iter().enumerate() {
        let response = ui
            .horizontal(|ui| {
                ui.label(&format!(":{}", i));

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
                    let mut combo_responses = Vec::new(); // To collect responses inside the ComboBox

                    // Iterate over available components
                    for available_component in components.iter() {
                        let item_response = ui.selectable_value(
                            &mut selected_input,
                            available_component.to_string(),
                            available_component,
                        );
                        combo_responses.push(item_response.clone()); // Add each response to the vector

                        if item_response.clicked() {
                            change = InputChange::Add(
                                component_name.to_string(),
                                selected_input.clone(),
                                i,
                            );
                        }
                    }

                    // Handle the "None" option
                    let none_response =
                        ui.selectable_value(&mut selected_input, "None".to_string(), "None");
                    combo_responses.push(none_response.clone()); // Add the "None" response

                    if none_response.clicked() {
                        change = InputChange::Remove(component_name.to_string(), i);
                    }

                    combo_responses // Return the collected responses
                })
                .response; // Get the Response from the ComboBox

                combo_box_response
            })
            .inner;

        input_responses.push(response);
    }

    (change, input_responses) // Return both the change and the responses
}

fn render_parameters<T: Float + Send + Sync + 'static + Numeric>(
    ui: &mut egui::Ui,
    component: &mut Component<T>,
) -> Vec<egui::Response> {
    let parameters = component.get_parameters();
    let mut param_responses = Vec::new(); // Collect responses

    if !parameters.is_empty() {
        ui.label("Params:");
        for (param, data) in parameters.iter() {
            ui.horizontal(|ui| {
                match data {
                    // Handle numerical values (use sliders or number inputs)
                    Data::Value(val) => {
                        ui.label(&param.name);

                        let mut value = *val;
                        let response = ui.add(egui::DragValue::new(&mut value)); // Capture response
                        if response.changed() {
                            component.set_parameter(&param.name, Data::Value(value));
                        }
                        param_responses.push(response); // Store the response
                    }

                    // Handle Vec3 (3D coordinates input)
                    Data::Vec3(vec3) => {
                        ui.label(&param.name);
                        ui.horizontal(|ui| {
                            let mut x = vec3.x;
                            let mut y = vec3.y;
                            let mut z = vec3.z;

                            ui.label("x:");
                            let x_response = ui.add(egui::DragValue::new(&mut x)); // Capture response
                            if x_response.changed() {
                                component.set_parameter(
                                    &param.name,
                                    Data::Vec3(types::geometry::Vec3::new(x, y, z)),
                                );
                            }

                            ui.label("y:");
                            let y_response = ui.add(egui::DragValue::new(&mut y)); // Capture response
                            if y_response.changed() {
                                component.set_parameter(
                                    &param.name,
                                    Data::Vec3(types::geometry::Vec3::new(x, y, z)),
                                );
                            }

                            ui.label("z:");
                            let z_response = ui.add(egui::DragValue::new(&mut z)); // Capture response
                            if z_response.changed() {
                                component.set_parameter(
                                    &param.name,
                                    Data::Vec3(types::geometry::Vec3::new(x, y, z)),
                                );
                            }

                            // Collect Vec3 responses
                            param_responses.push(x_response);
                            param_responses.push(y_response);
                            param_responses.push(z_response);
                        });
                    }

                    // Handle booleans (checkbox)
                    Data::Boolean(boolean) => {
                        ui.label(&param.name);
                        let mut value = *boolean;
                        let response = ui.checkbox(&mut value, ""); // Capture response
                        param_responses.push(response); // Store the response
                    }

                    // Handle text (text input)
                    Data::Text(text) => {
                        ui.label(&param.name);
                        let mut value = text.clone();
                        let response = ui.text_edit_singleline(&mut value); // Capture response
                        param_responses.push(response); // Store the response
                    }
                }
            });

            ui.add_space(5.0);
        }
    }

    param_responses // Return all captured responses
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
        Err(err) => info!("{}", err),
    }
}

fn configure_font(mut contexts: EguiContexts) {
    let ctx = contexts.ctx_mut();

    let mut font_def = FontDefinitions::default();

    font_def.font_data.insert(
        "Inconsolata".to_string(),
        FontData::from_owned(include_bytes!("../../assets/fonts/inconsolata-Regular.ttf").to_vec()),
    );
    font_def
        .families
        .insert(FontFamily::Proportional, vec!["Inconsolata".to_string()]);

    ctx.set_fonts(font_def);

    let mut style = (*ctx.style()).clone();

    style.text_styles.insert(
        TextStyle::Heading,
        FontId::new(16.0, FontFamily::Proportional),
    ); // Heading size
    style
        .text_styles
        .insert(TextStyle::Body, FontId::new(12.0, FontFamily::Proportional)); // Body text size
    style.text_styles.insert(
        TextStyle::Button,
        FontId::new(12.0, FontFamily::Proportional),
    ); // Button text size
    style.text_styles.insert(
        TextStyle::Monospace,
        FontId::new(12.0, FontFamily::Monospace),
    ); // Monospace text size

    ctx.set_style(style);
}

fn setup(mut commands: Commands) {
    let orthographic_camera = Camera3dBundle {
        projection: OrthographicProjection {
            scale: 1.0,
            near: 1e-1,
            far: 1e4,
            scaling_mode: ScalingMode::WindowSize(1.),
            ..Default::default()
        }
        .into(),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0))
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    };

    commands.spawn((orthographic_camera, PanOrbitCamera::default()));
}

fn build_mesh_from_data(mesh_data: RawMeshData) -> bevy::prelude::Mesh {
    bevy::prelude::Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(
        bevy::prelude::Mesh::ATTRIBUTE_POSITION,
        mesh_data.vertex_data.clone(),
    )
    .with_inserted_indices(mesh::Indices::U32(mesh_data.faces.clone()))
    .with_computed_smooth_normals()
}

fn update_wireframe(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut config: ResMut<WireframeConfig>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyE) {
        config.global = !config.global;
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

pub fn custom_dnd_drag_source<Payload, R>(
    ui: &mut egui::Ui,
    id: egui::Id,
    payload: Payload,
    add_contents: impl FnOnce(&mut egui::Ui) -> (R, Vec<egui::Response>),
) -> egui::InnerResponse<R>
where
    Payload: std::any::Any + Send + Sync,
{
    let is_being_dragged = ui.ctx().is_being_dragged(id);

    if is_being_dragged {
        egui::DragAndDrop::set_payload(ui.ctx(), payload);

        let layer_id = egui::LayerId::new(egui::Order::Tooltip, id);
        let InnerResponse { inner, response } = ui.with_layer_id(layer_id, |ui| add_contents(ui));

        if let Some(pointer_pos) = ui.ctx().pointer_interact_pos() {
            let delta = pointer_pos - response.rect.center();
            ui.ctx().transform_layer_shapes(
                layer_id,
                egui::emath::TSTransform::from_translation(delta),
            );
        }

        InnerResponse::new(inner.0, response)
    } else {
        let InnerResponse {
            inner,
            mut response,
        } = ui.scope(|ui| add_contents(ui));

        let mut hovering_anything = false;
        for res in inner.1.iter() {
            if res.hovered() {
                hovering_anything = true;
            }
        }

        if !hovering_anything {
            let drag_response = ui
                .interact(response.rect, id, egui::Sense::drag())
                .on_hover_cursor(egui::CursorIcon::Grab);
            response = response.union(drag_response);
        }

        InnerResponse::new(inner.0, response)
    }
}
