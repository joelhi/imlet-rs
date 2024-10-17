use bevy::prelude::PluginGroup;
use bevy::{
    app::{App, Plugin, Startup, Update},
    asset::Assets,
    color::Color,
    input::ButtonInput,
    math::Vec3,
    pbr::{
        wireframe::{WireframeConfig, WireframePlugin},
        MaterialMeshBundle,
    },
    prelude::{
        default, Camera3dBundle, Commands, KeyCode, Mesh, OrthographicProjection, Res, ResMut,
        Transform,
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
use bevy_egui::EguiPlugin;
use bevy_normal_material::{plugin::NormalMaterialPlugin, prelude::NormalMaterial};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

use crate::{types, viewer::raw_mesh_data::RawMeshData};

pub struct MeshViewerPlugin {
    mesh_data: RawMeshData,
}

impl MeshViewerPlugin {
    pub fn new(mesh: &types::geometry::Mesh<f32>) -> Self {
        Self {
            mesh_data: RawMeshData::from_mesh(mesh),
        }
    }
}

impl Plugin for MeshViewerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.mesh_data.clone())
            .add_systems(Startup, setup)
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
            .add_plugins(PanOrbitCameraPlugin)
            .add_plugins(NormalMaterialPlugin)
            .add_systems(Update, update_wireframe);
    }
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<NormalMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
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

fn build_mesh_from_data(mesh_data: Res<RawMeshData>) -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_data.vertex_data.clone())
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
