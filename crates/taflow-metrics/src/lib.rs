//! Persistent portfolio, strategy-performance, and risk metrics for TAFlow.

pub mod error;
pub mod input;
pub mod metrics;
pub mod primitives;

pub use error::{MetricError, MetricResult};
pub use input::{MetricInputKind, MetricInputState, NanPolicy, PairedMetricInputState};
