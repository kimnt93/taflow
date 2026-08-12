use super::{MetricPipeline, NanPolicy};
use crate::metrics::{SharpeRatio, SortinoRatio};

#[test]
fn configured_metrics_are_computed_under_caller_names() {
    let returns = [0.01, -0.02, 0.03, 0.0, 0.04];
    let mut pipeline = MetricPipeline::new();
    pipeline
        .add(
            "sharpe",
            SharpeRatio::new(252.0, 0.0, NanPolicy::Omit).unwrap(),
        )
        .unwrap()
        .add(
            "sor",
            SortinoRatio::new(252.0, 0.0, NanPolicy::Omit).unwrap(),
        )
        .unwrap()
        .from_returns(&returns)
        .unwrap();

    let values = pipeline.compute();
    assert_eq!(values[0].0, "sharpe");
    assert_eq!(values[1].0, "sor");
    assert!(values[0].1.is_some());
    assert!(values[1].1.is_some());
}

#[test]
fn scalar_chunk_reset_and_pnl_lifecycle_are_invariant() {
    let returns = [0.10, -0.20, 0.05];
    let mut batch = MetricPipeline::new();
    batch
        .add(
            "sharpe",
            SharpeRatio::new(12.0, 0.04, NanPolicy::Omit).unwrap(),
        )
        .unwrap()
        .from_returns(&returns)
        .unwrap();
    let expected = batch.compute()[0].1;

    let mut scalar = MetricPipeline::new();
    scalar
        .add(
            "sharpe",
            SharpeRatio::new(12.0, 0.04, NanPolicy::Omit).unwrap(),
        )
        .unwrap()
        .from_returns(&[])
        .unwrap()
        .append(returns[0])
        .unwrap()
        .extend(&returns[1..])
        .unwrap();
    assert_eq!(scalar.compute()[0].1, expected);
    scalar.reset().extend(&returns).unwrap();
    assert_eq!(scalar.compute()[0].1, expected);

    let mut pnl = MetricPipeline::new();
    pnl.add(
        "sharpe",
        SharpeRatio::new(12.0, 0.04, NanPolicy::Omit).unwrap(),
    )
    .unwrap()
    .from_pnl(&[10.0, -22.0, 4.4], 100.0)
    .unwrap();
    assert_eq!(pnl.compute()[0].1, expected);
}

#[test]
fn validates_names_order_and_domain_selection() {
    let mut pipeline = MetricPipeline::new();
    pipeline
        .add(
            "sharpe",
            SharpeRatio::new(252.0, 0.0, NanPolicy::Omit).unwrap(),
        )
        .unwrap();
    assert_eq!(pipeline.metric_names(), ["sharpe"]);
    assert!(pipeline
        .add(
            "sharpe",
            SortinoRatio::new(252.0, 0.0, NanPolicy::Omit).unwrap(),
        )
        .is_err());
    assert!(pipeline.append(0.01).is_err());
    pipeline.from_returns(&[0.01]).unwrap();
    assert!(pipeline.from_equity(&[100.0]).is_err());
    assert!(pipeline
        .add(
            "late",
            SortinoRatio::new(252.0, 0.0, NanPolicy::Omit).unwrap(),
        )
        .is_err());
}
