//! Mostra.
//!
//! Local-first engine for personal data exposure analysis.
//!
//! The initial release intentionally exposes only a minimal stable
//! surface while the domain model is designed.

#![forbid(unsafe_code)]

/// Returns the current Mostra crate version.
#[must_use]
pub const fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::version;

    #[test]
    fn version_matches_package_version() {
        assert_eq!(version(), env!("CARGO_PKG_VERSION"));
    }
}
