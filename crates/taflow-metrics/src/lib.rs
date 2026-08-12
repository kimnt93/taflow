//! Persistent portfolio, strategy-performance, and risk metrics for TAFlow.

pub mod error;
pub mod input;
mod metric_pipeline;
pub mod metrics;
pub mod primitives;

pub use error::{MetricError, MetricResult};
pub use input::{MetricInputKind, MetricInputState, NanPolicy, PairedMetricInputState};
pub use metric_pipeline::{MetricPipeline, MetricPipelineInputKind, PipelineMetric};

#[cfg(test)]
mod metric_pipeline_test;
