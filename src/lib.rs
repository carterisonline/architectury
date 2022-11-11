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

pub fn init() {
    #[cfg(feature = "base-logging")]
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
