use env_logger::Builder;
use log::LevelFilter;

pub fn init() {
    Builder::new()
        .filter(None, LevelFilter::Trace)
        .init();
}
