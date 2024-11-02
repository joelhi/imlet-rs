
use std::sync::{Arc, Mutex};

use bevy::{log::tracing_subscriber::Layer, prelude::Resource, utils::tracing::{self, Subscriber}};

#[derive(Default, Resource)]
pub struct LogMessages {
    pub messages: Arc<Mutex<Vec<String>>>
}

pub struct CustomLayer{
    pub log_messages: Arc<Mutex<Vec<String>>>,
}

impl<S: Subscriber> Layer<S> for CustomLayer {
    fn on_event(
        &self,
        event: &bevy::utils::tracing::Event<'_>,
        _ctx: bevy::log::tracing_subscriber::layer::Context<'_, S>,
    ) {
        let mut visitor = EventVisitor::new();
        event.record(&mut visitor);
        
        // Store the log message in the shared log_messages resource
        if !visitor.message.is_empty() {
            let mut log_messages = self.log_messages.lock().unwrap();
            log_messages.push(visitor.message.clone());
        }
    }
}

struct EventVisitor {
    message: String,
}

impl EventVisitor {
    fn new() -> Self {
        EventVisitor {
            message: String::new(),
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