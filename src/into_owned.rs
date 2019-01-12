//! IntoOwned module

/// IntoOwned trait
///
/// Example:
/// struct M<'a> {
///     bytes: Cow<'a, [u8],
/// }
///
/// impl IntoOwned for M {
///     fn into_owned(&mut self) {
///         self.bytes = self.bytes.into_owned();
///     }
/// }
pub trait IntoOwned {
    /// Turns all fields of self from Cow::Borrowed into Cow::Owned
    fn into_owned(&mut self);
}
