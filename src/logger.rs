use env_logger::Builder;
use log::LevelFilter;

#[inline]
pub fn init() {
    Builder::new()
        .filter(None, LevelFilter::Trace)
        .init();
}
