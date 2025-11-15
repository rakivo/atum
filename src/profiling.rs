/// Creates a Tracy profiling span when the `tracy` feature is enabled.
///
/// # Overview
/// This macro wraps `tracy_client::span!` behind a Cargo feature gate.
/// When the `tracy` feature is enabled, it expands to:
///
/// ```ignore
/// tracy_client::span!(...)
/// ```
///
/// When the `tracy` feature is *not* enabled, the macro expands to
/// a no-op and generates zero code.
///
/// # Feature Flags
/// - Requires: `tracy` feature
///
/// ---
#[cfg(feature = "tracy")]
#[macro_export]
macro_rules! tracy_span {
    ($($tt: tt)*) => { tracy_client::span!($($tt)*) };
}

/// No-op version of `tracy_span!` used when the `tracy` feature is disabled.
///
/// See the documentation on the enabled version for details.
#[cfg(not(feature = "tracy"))]
#[macro_export]
macro_rules! tracy_span {
    ($($tt: tt)*) => {
        ()
    };
}
