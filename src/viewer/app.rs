use bevy::{
    app::{App, Plugin, Startup},
    asset::Assets,
    color::Color,
    math::{Quat, Vec3},
    pbr::{DirectionalLight, DirectionalLightBundle, PbrBundle, StandardMaterial},
    prelude::{Camera3dBundle, Commands, Mesh, OrthographicProjection, Res, ResMut, Transform},
    render::{
        camera::ScalingMode,
        mesh::{self, PrimitiveTopology},
        render_asset::RenderAssetUsages,
    },
    DefaultPlugins,
};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

use super::raw_mesh_data::RawMeshData;

/// Open the viewer with a mesh geometry
pub fn run_viewer(mesh: &crate::types::geometry::Mesh<f32>) {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(AppPlugin(RawMeshData::from_mesh(mesh)))
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
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    custom_mesh_data: Res<RawMeshData>,
) {
    // Set up the orthographic camera with Z as up
    let orthographic_camera = Camera3dBundle {
        projection: OrthographicProjection {
            scale: 1.0,
            near: 1e-1,
            far: 1e4,
            scaling_mode: ScalingMode::FixedVertical(2.0),
            ..Default::default()
        }
        .into(),
        transform: Transform::from_translation(Vec3::new(0.0, 00.0, 10.0)) // Position above the origin
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y), // Looking down the Y-axis
        ..Default::default()
    };

    commands.spawn((orthographic_camera, PanOrbitCamera::default()));

    let mesh = build_mesh_from_data(custom_mesh_data);

    commands.spawn(PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.1, 0.7, 0.6),
            double_sided: true,
            cull_mode: None,
            ..Default::default()
        }),
        ..Default::default()
    });
    // light
    // Add a directional light (for ambient directional lighting)
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::srgb(1.0, 1.0, 1.0), // White light
            illuminance: 5000.0,               // Set the strength of the directional light
            shadows_enabled: true,             // Enable shadows for realism
            ..Default::default()
        },
        transform: Transform {
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..Default::default()
        },
        ..Default::default()
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
