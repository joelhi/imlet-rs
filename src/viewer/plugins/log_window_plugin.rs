use std::sync::{Arc, Mutex};

use bevy::{
    app::{App, Plugin, Update},
    log::{tracing_subscriber::Layer, BoxedLayer, LogPlugin},
    prelude::{default, Res},
};
use bevy_egui::{
    egui::{self, text::LayoutJob, Color32, Layout, ScrollArea, Ui},
    EguiContexts,
};

use crate::viewer::custom_layer::{CustomLayer, LogMessages};

pub struct LogWindowPlugin;

impl Plugin for LogWindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LogPlugin {
            custom_layer,
            ..default()
        })
        .add_systems(Update, logging_panel);
    }
}

fn custom_layer(app: &mut App) -> Option<BoxedLayer> {
    let log_messages = LogMessages::default();
    let inner_log_clone = log_messages.messages.clone();
    app.insert_resource(log_messages);
    Some(Box::new(vec![
        bevy::log::tracing_subscriber::fmt::layer()
            .with_file(true)
            .boxed(),
        Box::new(CustomLayer {
            log_messages: inner_log_clone,
        }),
    ]))
}

pub fn logging_panel(mut contexts: EguiContexts, log_handle: Res<LogMessages>) {
    let ctx = contexts.ctx_mut();
    egui::TopBottomPanel::bottom("OutputLogs")
        .resizable(true)
        .default_height(100.)
        .show(ctx, |ui| {
            ui.add_space(5.);
            ui.horizontal(|ui| {
                ui.with_layout(Layout::left_to_right(egui::Align::Min), |ui| {
                    ui.heading("Log");
                });
                ui.with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
                    if ui.button("Clear").clicked() {
                        let mut logs = log_handle
                            .messages
                            .lock()
                            .expect("Should be able to access log list mutex");
                        logs.clear();
                    };
                })
            });
            egui::Frame::dark_canvas(ui.style())
                .outer_margin(0.)
                .show(ui, |ui| {
                    render_logging_panel(ui, log_handle.messages.clone());
                });
        });
}

fn render_logging_panel(ui: &mut Ui, log_handle: Arc<Mutex<Vec<String>>>) {
    if let Ok(logs) = log_handle.lock() {
        let style = ui
            .style()
            .text_styles
            .get(&egui::TextStyle::Body)
            .expect("Should be here")
            .clone();
        ScrollArea::vertical()
            .auto_shrink([false; 2])
            .stick_to_bottom(true)
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    for log in logs.iter() {
                        let parts: Vec<&str> = log.split(" | ").collect();
                        if parts.len() == 3 {
                            let timestamp = parts[0];
                            let level = parts[1];
                            let message = parts[2];

                            let color = get_log_color(level);

                            let format = egui::TextFormat {
                                font_id: style.clone(),
                                ..Default::default()
                            };
                            let mut layout_job = LayoutJob::default();
                            layout_job.append(timestamp, 0.0, format.clone());
                            layout_job.append(" ", 0.0, format.clone()); // Space between parts
                            layout_job.append(
                                level,
                                0.0,
                                egui::TextFormat {
                                    font_id: style.clone(),
                                    color,
                                    ..Default::default()
                                },
                            );
                            layout_job.append(" ", 0.0, format.clone()); // Space between parts
                            layout_job.append(message, 0.0, format.clone());
                            ui.label(layout_job);
                        }
                    }
                    ui.label("");
                });
            });
    }
}

fn get_log_color(level: &str) -> Color32 {
    match level {
        "ERROR" => Color32::RED,
        "WARN " => Color32::YELLOW,
        "INFO " => Color32::DARK_GRAY,
        "DEBUG" => Color32::BLUE,
        _ => Color32::WHITE,
    }
}
