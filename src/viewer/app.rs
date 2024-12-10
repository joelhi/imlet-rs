use std::fmt::{Debug, Display};

use bevy::app::App;
use bevy_egui::egui::emath::Numeric;
use num_traits::Float;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    types::{computation::model::ImplicitModel, geometry::BoundingBox},
    utils::math_helper::Pi,
};

use super::plugins::{
    GraphicsPlugin, LogWindowPlugin, MeshViewerPlugin, ModelExplorerPlugin, ModelInitializerPlugin,
};

/// Run the explorer app with an empty model.
pub fn run_explorer<
    T: Float + Debug + Send + Sync + 'static + Numeric + Display + Pi + Serialize + DeserializeOwned,
>() {
    let mut app = App::new();

    app.add_plugins(MeshViewerPlugin::<T>::new())
        .add_plugins(ModelExplorerPlugin::<T>::new())
        .add_plugins(GraphicsPlugin)
        .add_plugins(LogWindowPlugin::<T>::new());

    app.run();
}

///
pub fn run_explorer_with_model<
    T: Float + Debug + Send + Sync + 'static + Numeric + Display + Pi + Serialize + DeserializeOwned,
>(
    model: ImplicitModel<T>,
    bounds: BoundingBox<T>,
) {
    let mut app = App::new();

    app.add_plugins(MeshViewerPlugin::<T>::new())
        .add_plugins(ModelExplorerPlugin::<T>::new())
        .add_plugins(GraphicsPlugin)
        .add_plugins(LogWindowPlugin::<T>::new());

    ModelInitializerPlugin::<T>::init(model, bounds)(&mut app);

    app.run();
}
