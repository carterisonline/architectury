#[cfg(feature = "base-logging")]
pub use {
    color_eyre::Result,
    tracing::{debug, error, info, trace, warn},
};