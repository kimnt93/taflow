mod metric_input_state;
mod paired_metric_input_state;

pub use metric_input_state::{MetricInputKind, MetricInputState, NanPolicy};
pub use paired_metric_input_state::PairedMetricInputState;

#[cfg(test)]
mod metric_input_state_test;
#[cfg(test)]
mod paired_metric_input_state_test;
