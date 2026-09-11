//! Future WebAssembly surface for Mostra.
//!
//! No WebAssembly API is frozen in the bootstrap release.

#![forbid(unsafe_code)]

/// Returns the version of the underlying Mostra core.
#[must_use]
pub const fn core_version() -> &'static str {
    mostra::version()
}
