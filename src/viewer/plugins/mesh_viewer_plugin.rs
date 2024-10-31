use bevy::prelude::{Entity, PluginGroup, Resource};
use bevy::{
    app::{App, Plugin, Startup, Update},
    color::Color,
    input::ButtonInput,
    math::Vec3,
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::{
        default, Camera3dBundle, Commands, KeyCode, OrthographicProjection, Res, ResMut, Transform,
    },
    render::{
        camera::ScalingMode,
        settings::{RenderCreation, WgpuFeatures, WgpuSettings},
        RenderPlugin,
    },
    DefaultPlugins,
};
use bevy_normal_material::plugin::NormalMaterialPlugin;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

pub struct MeshViewerPlugin;

impl Plugin for MeshViewerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
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
            .insert_resource(CurrentMeshEntity(None))
            .add_plugins(PanOrbitCameraPlugin)
            .add_plugins(NormalMaterialPlugin)
            .add_systems(Update, update_wireframe);
    }
}

#[derive(Resource)]
pub struct CurrentMeshEntity(pub Option<Entity>);

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

fn update_wireframe(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut config: ResMut<WireframeConfig>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyE) {
        config.global = !config.global;
    }
}
