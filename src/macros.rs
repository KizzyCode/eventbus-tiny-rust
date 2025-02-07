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

/// Helper macro to quickly create an aggregated enum
#[macro_export]
macro_rules! aggregate_enum {
    ($visibility:vis $name:ident { $($variant:tt),+ }) => {
        /// A basic aggregate enum
        #[derive(Debug, Clone)]
        $visibility enum $name {
            // Enum variants
            $($variant($variant),)+
        }
        impl $name {
            /// The filter-map function to create `Self` from an event if possible
            pub fn try_from_event(event: &dyn ::std::any::Any) -> ::std::option::Option<Self> {
                // Try to downcast events
                $(
                    // Downcast event if possible
                    if let Some(event) = event.downcast_ref::<$variant>() {
                        // Init self
                        let this = <$name>::$variant(event.clone());
                        return Some(this);
                    }
                )+

                // No downcast successful
                return None;
            }
        }
    };
}
