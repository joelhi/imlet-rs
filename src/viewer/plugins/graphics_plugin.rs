use bevy::{
    app::{App, Plugin, Startup},
    asset::AssetServer,
    prelude::{Commands, ResMut, Resource},
};
use bevy_egui::{
    egui::{FontData, FontDefinitions, FontFamily, FontId, TextStyle, TextureId},
    EguiContexts,
};

use crate::types::computation::Component;

const FONT_SIZE: f32 = 12.;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, configure_fonts)
            .add_systems(Startup, load_icon_files);
    }
}

fn configure_fonts(mut contexts: EguiContexts) {
    let ctx = contexts.ctx_mut();

    let mut font_def = FontDefinitions::default();

    font_def.font_data.insert(
        "Inconsolata-Regular".to_string(),
        FontData::from_owned(
            include_bytes!("../../../assets/fonts/inconsolata-Regular.ttf").to_vec(),
        ),
    );
    font_def.font_data.insert(
        "Inconsolata-Bold".to_string(),
        FontData::from_owned(include_bytes!("../../../assets/fonts/inconsolata-Bold.ttf").to_vec()),
    );
    font_def.families.insert(
        FontFamily::Proportional,
        vec!["Inconsolata-Regular".to_string()],
    );
    font_def.families.insert(
        FontFamily::Name("Bold".into()),
        vec!["Inconsolata-Bold".to_string()],
    );

    ctx.set_fonts(font_def);

    let mut style = (*ctx.style()).clone();

    style.text_styles.insert(
        TextStyle::Heading,
        FontId::new(FONT_SIZE, FontFamily::Name("Bold".into())),
    ); // Heading size
    style.text_styles.insert(
        TextStyle::Body,
        FontId::new(FONT_SIZE, FontFamily::Proportional),
    ); // Body text size
    style.text_styles.insert(
        TextStyle::Button,
        FontId::new(FONT_SIZE, FontFamily::Proportional),
    ); // Button text size
    style.text_styles.insert(
        TextStyle::Monospace,
        FontId::new(FONT_SIZE, FontFamily::Monospace),
    ); // Monospace text size

    ctx.set_style(style);
}

#[derive(Resource)]
pub struct Icons {
    pub function_icon: TextureId,
    pub operation_icon: TextureId,
    pub delete_icon: TextureId,
    pub compute_icon: TextureId,
    pub show_bounds: TextureId,
    pub export: TextureId,
    pub show_edges: TextureId
}

impl Icons {
    pub(crate) fn component_icon<T>(&self, component: &Component<T>) -> &TextureId {
        match component {
            Component::Constant(_) => &self.operation_icon,
            Component::Function(_) => &self.function_icon,
            Component::Operation(_) => &self.operation_icon,
        }
    }

    pub(crate) fn delete_icon(&self) -> &TextureId {
        &self.delete_icon
    }
}

fn load_icon_files(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut contexts: EguiContexts,
) {
    let icons = Icons {
        function_icon: contexts.add_image(asset_server.load("icons/function_16x16.png")),
        operation_icon: contexts.add_image(asset_server.load("icons/graph_16x16.png")),
        delete_icon: contexts.add_image(asset_server.load("icons/delete_16x16.png")),
        compute_icon: contexts.add_image(asset_server.load("icons/compute_24x24.png")),
        show_bounds: contexts.add_image(asset_server.load("icons/show_bounds_24x24.png")),
        export: contexts.add_image(asset_server.load("icons/file_export_24x24.png")),
        show_edges: contexts.add_image(asset_server.load("icons/show_edges_24x24.png")),
    };

    commands.insert_resource(icons);
}
