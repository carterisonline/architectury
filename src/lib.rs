#[cfg(feature = "green-threads")]
pub mod gt;

#[cfg(feature = "rng")]
pub mod rng;

#[cfg(feature = "log")]
pub use color_eyre as log;

#[cfg(feature = "log")]
pub mod tracing {
    pub use tracing::*;
    pub use tracing_error as error;
    pub use tracing_subscriber as subscriber;
}

pub mod prelude;

#[macro_export]
macro_rules! tests {
    ($(use $import: path);*; $($fn: ident $block: block)+) => {
        #[cfg(test)]
        pub mod tests {
            use architectury::prelude::*;
            use architectury::prelude::{assert_eq, assert_ne, assert_str_eq};
            $(
                use $import;
            );*
            $(
                #[test]
                fn $fn() {
                    architectury::init();
                    $block
                }
            )+
        }
    };

    ($($fn: ident $block: block)+) => {
        tests!{; $($fn $block)+}
    }

}

#[cfg(feature = "log")]
#[macro_export]
macro_rules! debug {
    ($($tt:tt)*) => {
        log
    };
}

/// Initializes the Architectury runtime.
pub fn init() {
    #[cfg(feature = "log")]
    {
        use tracing_error::ErrorLayer;
        use tracing_subscriber::prelude::*;
        use tracing_subscriber::{fmt, EnvFilter};

        let fmt_layer = fmt::layer().with_target(false);
        let filter_layer = EnvFilter::try_from_default_env()
            .or_else(|_| EnvFilter::try_new("info"))
            .unwrap();

        tracing_subscriber::registry()
            .with(filter_layer)
            .with(fmt_layer)
            .with(ErrorLayer::default())
            .init();

        color_eyre::install().expect("Failed to initialize color_eyre");
    }
}
