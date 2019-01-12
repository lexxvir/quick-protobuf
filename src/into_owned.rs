//! IntoOwned module

/// IntoOwned trait
pub trait IntoOwned<T> {
    /// Converts self such that all fields with `Cow` type become Cow::Owned variant
    fn into_owned(self) -> T;
}
