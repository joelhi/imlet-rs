use env_logger;
use log::LevelFilter;

pub fn init_info(){
    env_logger::Builder::new()
        .filter_module("imlet_engine", LevelFilter::Info)
        .init();
}

pub fn init_debug(){
    env_logger::Builder::new()
        .filter_module("imlet_engine", LevelFilter::Debug)
        .init();
}