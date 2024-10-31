use std::fmt::{Debug, Display};

use bevy::app::App;
use bevy_egui::egui::emath::Numeric;
use num_traits::Float;

use crate::types::{
        computation::ImplicitModel,
        geometry::BoundingBox,
    };

use super::plugins::{
        GraphicsPlugin, MeshViewerPlugin,
        ModelExplorerPlugin, ModelInitializerPlugin,
    };

    pub fn run_explorer<T: Float + Debug + Send + Sync + 'static + Numeric + Debug + Display>(
        model: ImplicitModel<T>,
        bounds: BoundingBox<T>,
    ) {
        let mut app = App::new();

        app.add_plugins(MeshViewerPlugin)
            .add_plugins(ModelExplorerPlugin::<T>::new())
            .add_plugins(GraphicsPlugin);
            
        ModelInitializerPlugin::<T>::new(model, bounds)(&mut app);

        app.run();
    }
