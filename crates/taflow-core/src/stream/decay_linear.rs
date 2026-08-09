//! Batch implementation for `decay_linear`.

/// WorldQuant Alpha101 linear-decay weighted moving average.
///
/// This canonical state delegates to the shared weighted-moving-average recurrence.
pub type DecayLinear = super::WeightedMovingAverage;
