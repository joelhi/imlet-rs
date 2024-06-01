use env_logger;
use log::LevelFilter;

pub fn init(){
    env_logger::Builder::new()
        .filter_module("implicit", LevelFilter::Info)
        .init();
}