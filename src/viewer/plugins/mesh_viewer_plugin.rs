use bevy::asset::{Asset, Assets, Handle};
use bevy::color::LinearRgba;
use bevy::pbr::MaterialPlugin;
use bevy::prelude::{Entity, Gizmos, PluginGroup, Resource};
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
use bevy_egui::EguiPlugin;
use bevy_normal_material::plugin::NormalMaterialPlugin;
use bevy_normal_material::prelude::NormalMaterial;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

use crate::types::geometry::BoundingBox;

use super::LineMaterial;

#[derive(Resource)]
pub struct ViewSettings<T> {
    pub show_bounds: bool,
    pub show_world_axes: bool,
    pub bounds: Option<BoundingBox<T>>,
}

pub struct MeshViewerPlugin<T> {
    _marker: std::marker::PhantomData<T>,
}

impl<T> MeshViewerPlugin<T> {
    pub fn new() -> Self {
        MeshViewerPlugin {
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T: Send + Sync + 'static> Plugin for MeshViewerPlugin<T> {
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
            .add_plugins(EguiPlugin)
            .add_plugins(MaterialPlugin::<LineMaterial>::default())
            .insert_resource(CurrentMeshEntity(None))
            .insert_resource(CurrentBounds(None))
            .add_plugins(PanOrbitCameraPlugin)
            .add_plugins(NormalMaterialPlugin)
            .insert_resource(ViewSettings::<T> {
                show_bounds: false,
                show_world_axes: true,
                bounds: None,
            })
            .add_systems(Startup, init_materials)
            .add_systems(Update, update_wireframe)
            .add_systems(Update, draw_axes::<T>);
    }
}

#[derive(Resource)]
pub struct CurrentMeshEntity(pub Option<Entity>);

#[derive(Resource)]
pub struct CurrentBounds(pub Option<Entity>);

#[derive(Resource)]
pub struct ModelMaterial<T: Asset>(pub Handle<T>);

pub fn init_materials(
    mut commands: Commands,
    mut normal_materials: ResMut<Assets<NormalMaterial>>,
    mut line_materials: ResMut<Assets<LineMaterial>>,
) {
    let normal_material_handle = normal_materials.add(NormalMaterial {
        opacity: 1.0,
        depth_bias: 0.0,
        cull_mode: None,
        alpha_mode: Default::default(),
    });

    commands.insert_resource(ModelMaterial(normal_material_handle.clone()));

    let line_material_handle = line_materials.add(LineMaterial {
        color: LinearRgba::WHITE,
    });

    commands.insert_resource(ModelMaterial(line_material_handle.clone()));
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

fn update_wireframe(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut config: ResMut<WireframeConfig>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyE) {
        config.global = !config.global;
    }
}

fn draw_axes<T: Send + Sync + 'static>(mut gizmos: Gizmos, view_settings: Res<ViewSettings<T>>) {
    if view_settings.show_world_axes {
        gizmos.axes(Transform::default(), 10.);
    }
}
