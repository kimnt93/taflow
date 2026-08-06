//! Shared lifecycle for incremental single-input technical indicators.

/// Common interface for scalar indicators.
pub trait StreamingIndicator {
    type Output: Copy;

    /// Adds one observation and returns a value once the indicator is warm.
    fn append(&mut self, input: f64) -> Option<Self::Output>;

    /// Returns the most recently produced value, if warm.
    fn value(&self) -> Option<Self::Output>;

    /// Restores the post-construction state while retaining allocated buffers.
    fn reset(&mut self);

    fn extend<I>(&mut self, inputs: I) -> Vec<Option<Self::Output>>
    where
        I: IntoIterator<Item = f64>,
    {
        inputs.into_iter().map(|input| self.append(input)).collect()
    }
}
