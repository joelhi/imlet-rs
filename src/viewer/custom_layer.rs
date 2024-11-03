use std::{
    sync::{Arc, Mutex},
    time::SystemTime,
};

use bevy::{
    log::tracing_subscriber::Layer,
    prelude::Resource,
    utils::tracing::{self, Subscriber},
};
use chrono::{DateTime, Utc};

#[derive(Default, Resource)]
pub struct LogMessages {
    pub messages: Arc<Mutex<Vec<String>>>,
}

pub struct CustomLayer {
    pub log_messages: Arc<Mutex<Vec<String>>>,
}

const MAX_SIZE: usize = 100;

impl<S: Subscriber> Layer<S> for CustomLayer {
    fn on_event(
        &self,
        event: &bevy::utils::tracing::Event<'_>,
        _ctx: bevy::log::tracing_subscriber::layer::Context<'_, S>,
    ) {
        let mut visitor = EventVisitor::new(event.metadata().level().to_string());
        event.record(&mut visitor);

        if !visitor.message.is_empty() {
            let mut log_messages = self.log_messages.lock().unwrap();

            if log_messages.len() >= MAX_SIZE {
                log_messages.remove(0);
            }

            let system_time = SystemTime::now();
            let datetime: DateTime<Utc> = system_time.into();
            let timestamp = datetime.format("%Y%m%d %H:%M:%S").to_string();

            let formatted = format!("{} | {:<5} | {}", timestamp, visitor.level, visitor.message,);

            log_messages.push(formatted);
        }
    }
}

struct EventVisitor {
    message: String,
    level: String,
}

impl EventVisitor {
    fn new(level: String) -> Self {
        EventVisitor {
            message: String::new(),
            level,
        }
    }
}

impl tracing::field::Visit for EventVisitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            self.message.push_str(&format!("{:?}", value));
        }
    }
}
