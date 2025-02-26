use log::{info, error, LevelFilter};
use env_logger::Builder;

pub fn init_logging() {
    Builder::new()
        .filter(None, LevelFilter::Info)
        .init();
    info!("Logging initialized for ArcOracle backend");
}

pub fn log_error(message: &str) {
    error!("{}", message);
}

pub fn log_info(message: &str) {
    info!("{}", message);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logging() {
        init_logging();
        log_info("Test info message");
        log_error("Test error message");
    }
}