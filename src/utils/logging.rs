use env_logger;
use log::LevelFilter;

/// Enable info level logging for the imlet crate only.
pub fn init_info() {
    env_logger::Builder::new()
        .filter_module("imlet_engine", LevelFilter::Info)
        .init();
}

/// Enable debug level logging for the imlet crate only.
pub fn init_debug() {
    env_logger::Builder::new()
        .filter_module("imlet_engine", LevelFilter::Debug)
        .init();
}
