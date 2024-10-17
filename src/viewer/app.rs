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
        default, Camera3dBundle, Commands, KeyCode,
        OrthographicProjection, Res, ResMut, Transform,
    },
    render::{
        camera::ScalingMode,
        mesh::{self, PrimitiveTopology},
        render_asset::RenderAssetUsages,
        settings::{RenderCreation, WgpuFeatures, WgpuSettings},
        RenderPlugin,
    },
    DefaultPlugins,
};
use bevy_egui::{
    egui::{self, ScrollArea},
    EguiContexts, EguiPlugin,
};
use bevy_normal_material::{plugin::NormalMaterialPlugin, prelude::NormalMaterial};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use num_traits::Float;

use crate::types::{
    computation::ImplicitModel,
    geometry::{BoundingBox, Mesh},
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

pub fn run_explorer<T: Float + Send + Sync>(
    model: ImplicitModel<T>,
    cell_size: T,
    bounds: &BoundingBox<T>,
    target: &str,
) {
    let mesh = model
        .generate_iso_surface(target, bounds, cell_size)
        .unwrap();

    App::new()
        .insert_resource(RawMeshData::from_mesh(&mesh.convert::<f32>()))
        .add_plugins((
            DefaultPlugins
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        // WARN this is a native only feature. It will not work with webgl or webgpu
                        features: WgpuFeatures::POLYGON_MODE_LINE,
                        ..default()
                    }),
                    synchronous_pipeline_compilation: false,
                })
                .disable::<bevy::log::LogPlugin>(),
            WireframePlugin,
        ))
        .insert_resource(WireframeConfig {
            global: true,
            default_color: Color::WHITE,
        })
        .add_plugins(EguiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, ui_system)
        .add_plugins(NormalMaterialPlugin)
        .add_systems(Update, update_wireframe)
        .add_plugins(PanOrbitCameraPlugin)
        .run();
}

fn egui_setup(mut contexts: EguiContexts) {
    let ctx = contexts.ctx_mut();
}

fn ui_system(mut contexts: EguiContexts) {
    let components = vec!["Sphere", "Gyroid", "Offset"];
    let ctx = contexts.ctx_mut();

    egui::SidePanel::left("Left").show(ctx, |ui| {
        ScrollArea::new([true, false]).show(ui, |ui| {
            for &c in &components {
                ui.label(c);
                ui.label("Params");
            }
        });

        ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
    });
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<NormalMaterial>>,
    mut meshes: ResMut<Assets<bevy::prelude::Mesh>>,
    custom_mesh_data: Res<RawMeshData>,
) {
    let centroid = custom_mesh_data.centroid();
    let target = Vec3::new(centroid.x, centroid.y, centroid.z);

    let orthographic_camera = Camera3dBundle {
        projection: OrthographicProjection {
            scale: 1.0,
            near: 1e-1,
            far: 1e4,
            scaling_mode: ScalingMode::WindowSize(600. / custom_mesh_data.bounds.dimensions().1),
            ..Default::default()
        }
        .into(),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0))
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    };

    commands.spawn((orthographic_camera, PanOrbitCamera::default()));

    let mesh = build_mesh_from_data(custom_mesh_data);

    let mat = materials.add(NormalMaterial::default());

    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(mesh),
        material: mat,
        transform: Transform::from_translation(-target),
        ..default()
    });
}

fn build_mesh_from_data(mesh_data: Res<RawMeshData>) -> bevy::prelude::Mesh {
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