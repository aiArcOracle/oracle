use log::{info, error};

pub fn init_logging() {
    env_logger::init();
    info!("Logging initialized for ArcOracle backend");
}

pub fn log_error(message: &str) {
    error!("{}", message);
}
