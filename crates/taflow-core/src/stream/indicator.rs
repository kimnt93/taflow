//! Shared lifecycle for incremental single-input technical indicators.

/// Common interface for scalar indicators.
/// Common lifecycle contract for a persistent streaming indicator.
///
/// Implementations consume one chronological observation at a time and
/// expose resettable state for continuation across batches.
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

    /// Extends the state without exposing `Option` values at the bulk boundary.
    ///
    /// Warm-up samples are represented as `NaN`, which is the representation
    /// used by the batch and Python APIs.  Keeping that conversion in the core
    /// loop avoids allocating an intermediate `Vec<Option<_>>` and a second
    /// mapped vector for every bulk call.
    fn extend_into<I>(&mut self, inputs: I) -> Vec<f64>
    where
        I: IntoIterator<Item = f64>,
        Self::Output: Into<f64>,
    {
        let iter = inputs.into_iter();
        let (lower, upper) = iter.size_hint();
        let mut output = Vec::with_capacity(upper.unwrap_or(lower));
        output.extend(iter.map(|input| self.append(input).map(Into::into).unwrap_or(f64::NAN)));
        output
    }

    /// Extends from a borrowed slice directly into a caller-owned output.
    /// Implementations may override this with a specialized bulk kernel; the
    /// default preserves streaming semantics and chunk invariance.
    fn extend_slice_into(&mut self, inputs: &[f64], output: &mut Vec<f64>)
    where
        Self::Output: Into<f64>,
    {
        output.reserve(inputs.len());
        output.extend(
            inputs
                .iter()
                .copied()
                .map(|input| self.append(input).map(Into::into).unwrap_or(f64::NAN)),
        );
    }
}
