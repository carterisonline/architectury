#[cfg(feature = "base-logging")]
pub use {
    color_eyre::Result,
    tracing::{debug, error, info, trace, warn},
};

#[cfg(feature = "util-testing")]
pub use {
    crate::tests,
    pretty_assertions::{assert_eq, assert_ne, assert_str_eq},
};
