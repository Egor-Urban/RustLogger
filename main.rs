// Usage

pub mod logger; // logger.rs
use crate::logger::LOGGER;

fn main() {
    LOGGER.info("Info msg");
    LOGGER.debug("Debug msg");
    LOGGER.error("Error msg");
    LOGGER.warn("Warn msg");
}
