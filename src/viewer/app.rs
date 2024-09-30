use super::raw_mesh_data::RawMeshData;
use bevy::{
    app::{App, Plugin, Startup},
    asset::Assets,
    color::Color,
    math::Vec3,
    pbr::{AmbientLight, DirectionalLight, DirectionalLightBundle, PbrBundle, StandardMaterial},
    prelude::{
        Camera3dBundle, Commands, Mesh, OrthographicProjection, Res, ResMut, TextBundle, Transform,
    },
    render::{
        camera::ScalingMode,
        mesh::{self, PrimitiveTopology},
        render_asset::RenderAssetUsages,
        settings::{RenderCreation, WgpuFeatures, WgpuSettings},
        RenderPlugin,
    },
    text::TextStyle,
    ui::{PositionType, Style, Val},
    utils::default,
    DefaultPlugins,
};
use bevy_normal_material::{plugin::NormalMaterialPlugin, prelude::NormalMaterial};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

use bevy::{
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
};

const BASIC_TEXT: &'static str = "imlet viewer";

/// Open the viewer with a mesh geometry
pub fn run_viewer(mesh: &crate::types::geometry::Mesh<f32>) {
    App::new()
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
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(NormalMaterialPlugin)
        .add_plugins(AppPlugin(RawMeshData::from_mesh(mesh)))
        .add_systems(Update, update_wireframe)
        .insert_resource(AmbientLight {
            color: Color::srgb(0.7, 0.7, 0.8),
            brightness: 1000.0,
        })
        .run();
}

struct AppPlugin(RawMeshData);

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Pass the mesh to the setup system
        app.insert_resource(self.0.clone())
            .add_systems(Startup, setup);
    }
}

// Setup function to create the scene
fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<NormalMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    custom_mesh_data: Res<RawMeshData>,
) {
    let centroid = custom_mesh_data.bounds.centroid();
    let target = Vec3::new(centroid.x, centroid.y, centroid.z);
    // Set up the orthographic camera with Z as up
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

    // commands.spawn(DirectionalLightBundle {
    //     transform: Transform::from_xyz(50.0, 50.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
    //     directional_light: DirectionalLight {
    //         illuminance: 5000.,
    //         ..default()
    //     },
    //     ..default()
    // });

    // Text used to show controls
    commands.spawn(
        TextBundle::from_section(BASIC_TEXT, TextStyle::default()).with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
    );
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
    // Toggle showing a wireframe on all meshes
    if keyboard_input.just_pressed(KeyCode::KeyE) {
        config.global = !config.global;
    }
}
