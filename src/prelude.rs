#[cfg(feature = "log")]
pub use {
    color_eyre::Result,
    tracing::{debug, error, info, trace, warn},
};

#[cfg(feature = "assert")]
pub use {
    crate::tests,
    pretty_assertions::{assert_eq, assert_ne, assert_str_eq},
};

#[cfg(feature = "green-threads")]
pub use rayon::prelude::*;
