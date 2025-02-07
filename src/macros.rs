//! Useful macros

/// Creates a filter-map clause that maps any event of type `$source` into `Some(T)`
#[macro_export]
macro_rules! where_into {
    ($($source:ty),+ => $target:ty) => {{
        // Create mapper function
        fn convert(event: &dyn ::std::any::Any) -> ::std::option::Option<$target> {
            // Try to downcast events
            $(
                // Downcast event if possible
                if let Some(event) = event.downcast_ref::<$source>() {
                    let cloned = event.clone();
                    return Some(cloned.into());
                }
            )+

            // No downcast successful
            return None;
        }

        // Return mapper function
        convert
    }};
}

/// Creates a filter-map clause that tries to map any event of type `$source` into `Some(T)`
#[macro_export]
macro_rules! where_try_into {
    ($($source:ty),+ => $target:ty) => {{
        // Create mapper function
        fn convert(event: &dyn ::std::any::Any) -> ::std::option::Option<$target> {
            // Try to downcast events
            $(
                // Downcast and map event if possible
                if let Some(event) = event.downcast_ref::<$source>() {
                    let cloned = event.clone();
                    if let Ok(mapped) = cloned.try_into() {
                        // Return mapped event
                        return Some(mapped);
                    }
                }
            )+

            // No downcast+mapping successful
            return None;
        }

        // Return mapper function
        convert
    }};
}

/// Implements `From` to convert `$source` into `$enum::$variant($source)`
#[macro_export]
macro_rules! enum_from {
    ($($source:ty => $enum:tt :: $variant:tt),+) => {
        $(
            // Enum conversion
            impl ::std::convert::From<$source> for $enum {
                fn from(source: $source) -> Self {
                    // Do conversion
                    <$enum>::$variant(source)
                }
            }
        )+
    };
}
