use std::fmt::{Debug, Display};

use bevy::{
    app::{App, Plugin, Startup},
    prelude::{Commands, ResMut, Resource},
};
use bevy_egui::egui::emath::Numeric;
use num_traits::Float;

use crate::types::{computation::ImplicitModel, geometry::BoundingBox};

use super::{AppModel, Config};

/// Temporary resources to hold model and bounds
#[derive(Resource)]
struct TempResource<Q>(Option<Q>);

/// Plugin to initialize the app with an pre-made implicit model and bounds
pub struct ModelInitializerPlugin<T> {
    _marker: std::marker::PhantomData<T>,
}

impl<T: Float + Send + Sync + 'static + Numeric + Display + Debug> ModelInitializerPlugin<T> {
    pub fn init(model: ImplicitModel<T>, bounds: BoundingBox<T>) -> impl FnOnce(&mut App) {
        move |app: &mut App| {
            app.insert_resource(TempResource(Some(model)));
            app.insert_resource(TempResource(Some(bounds)));

            app.add_plugins(ModelInitializerPlugin {
                _marker: std::marker::PhantomData::<T>,
            });
        }
    }
}

impl<T: Float + Send + Sync + 'static + Numeric + Display + Debug> Plugin
    for ModelInitializerPlugin<T>
{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_model::<T>);
    }
}

// System to move `model` and `bounds` from the temporary resources into `AppModel` and `Config`
fn initialize_model<T: Float + Send + Sync + 'static + Numeric + Display + Debug>(
    mut commands: Commands,
    mut temp_model: ResMut<TempResource<ImplicitModel<T>>>,
    mut temp_bounds: ResMut<TempResource<BoundingBox<T>>>,
    mut app_model: ResMut<AppModel<T>>,
    mut config: ResMut<Config<T>>,
) {
    if let Some(model) = temp_model.0.take() {
        log::info!("Initalizing model.");
        *app_model = AppModel::new(model);
    }

    if let Some(bounds) = temp_bounds.0.take() {
        log::info!("Initializing bounds.");
        config.bounds = bounds;
    }

    // Remove the temporary resources after assignment to free up memory
    commands.remove_resource::<TempResource<Option<ImplicitModel<T>>>>();
    commands.remove_resource::<TempResource<Option<BoundingBox<T>>>>();
}
