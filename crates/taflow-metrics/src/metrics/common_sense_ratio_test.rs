use super::common_sense_ratio::CommonSenseRatio;
use crate::{MetricInputKind, NanPolicy};
use approx::assert_relative_eq;

fn linear_quantile(mut values: Vec<f64>, cutoff: f64) -> f64 {
    values.sort_by(f64::total_cmp);
    let index = (values.len() - 1) as f64 * cutoff;
    let lower = index.floor() as usize;
    let upper = index.ceil() as usize;
    values[lower] + (values[upper] - values[lower]) * (index - lower as f64)
}

fn component_reference(values: &[f64]) -> f64 {
    let gains: f64 = values.iter().copied().filter(|value| *value > 0.0).sum();
    let losses: f64 = values.iter().copied().filter(|value| *value < 0.0).sum();
    let lower = linear_quantile(values.to_vec(), 0.05);
    let upper = linear_quantile(values.to_vec(), 0.95);
    gains / -losses * (upper.abs() / lower.abs())
}

#[test]
fn matches_profit_factor_times_exact_tail_ratio() {
    let values = [-0.10, -0.04, -0.01, 0.0, 0.02, 0.05, 0.12];
    let mut state = CommonSenseRatio::new(MetricInputKind::Returns, NanPolicy::Omit).unwrap();
    state.extend(&values).unwrap();
    assert_relative_eq!(
        state.compute().unwrap(),
        component_reference(&values),
        epsilon = 1e-15
    );
}

#[test]
fn freezes_zero_denominator_and_loss_only_edges() {
    let mut positive = CommonSenseRatio::new(MetricInputKind::Returns, NanPolicy::Omit).unwrap();
    positive.extend(&[0.01, 0.02, 0.03]).unwrap();
    assert_eq!(positive.value(), None);

    let mut losses = CommonSenseRatio::new(MetricInputKind::Returns, NanPolicy::Omit).unwrap();
    losses.extend(&[-0.01, -0.02, -0.03]).unwrap();
    assert_eq!(losses.value(), Some(0.0));

    let mut zero = CommonSenseRatio::new(MetricInputKind::Returns, NanPolicy::Omit).unwrap();
    zero.extend(&[0.0, 0.0]).unwrap();
    assert_eq!(zero.value(), None);
}

#[test]
fn lifecycle_chunking_omission_and_reset_are_invariant() {
    let values = [-0.10, f64::NAN, -0.04, -0.01, 0.0, 0.02, 0.05, 0.12];
    let mut batch = CommonSenseRatio::new(MetricInputKind::Returns, NanPolicy::Omit).unwrap();
    batch.extend(&values).unwrap();
    assert_eq!(batch.len(), 7);
    let expected = batch.value().unwrap();

    let mut streamed = CommonSenseRatio::new(MetricInputKind::Returns, NanPolicy::Omit).unwrap();
    for value in values {
        streamed.append(value).unwrap();
    }
    assert_relative_eq!(streamed.compute().unwrap(), expected, epsilon = 1e-15);
    streamed.reset();
    assert!(streamed.is_empty());
    assert_eq!(streamed.value(), None);
    streamed.extend(&values).unwrap();
    assert_relative_eq!(streamed.compute().unwrap(), expected, epsilon = 1e-15);
}

#[test]
fn rejects_every_non_return_domain() {
    for input_kind in [
        MetricInputKind::LogReturns,
        MetricInputKind::Equity,
        MetricInputKind::RawPnl,
        MetricInputKind::Trades,
        MetricInputKind::PeriodPnl {
            initial_equity: 100.0,
        },
    ] {
        assert!(CommonSenseRatio::new(input_kind, NanPolicy::Omit).is_err());
    }
}
