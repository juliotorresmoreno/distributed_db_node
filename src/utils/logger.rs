
use log::LevelFilter;
use env_logger::Builder;

pub fn init_logger() {
    Builder::new()
        .filter(None, LevelFilter::Info)
        .init();
}
