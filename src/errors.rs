macro_rules! create_error_wrapper {
    ($enum:ident, $($variant:ident($type:ty)),* $(,)?) => {
        // Define the enum with the specified variants
        #[derive(Debug)]
        pub enum $enum {
            $(
                $variant($type),
            )*
        }

        // Implement From trait for each variant
        $(
            impl From<$type> for $enum {
                fn from(err: $type) -> Self {
                    $enum::$variant(err)
                }
            }
        )*
    };
}

create_error_wrapper!(
    DeviceFlowError,
    // user errors
    Other(String),
    // wrapped user errors
    IO(std::io::Error),
    Http(attohttpc::Error),
    Keyring(keyring::Error),
);
