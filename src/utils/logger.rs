
use log::LevelFilter;
use env_logger::Builder;

#[allow(dead_code)]
pub fn init_logger() {
    Builder::new()
        .filter(None, LevelFilter::Info)
        .init();
}
