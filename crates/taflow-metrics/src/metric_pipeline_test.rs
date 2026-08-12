use super::{MetricInputKind, MetricPipeline, MetricPipelineConfiguration, NanPolicy};
use crate::metrics::{MaximumDrawdown, SharpeRatio, TotalReturn};

#[test]
fn pnl_is_normalized_once_and_matches_standalone_return_metrics() {
    let pnl = [10.0, -5.0, 7.0, -2.0];
    let names = vec![
        "TotalReturn".to_owned(),
        "SharpeRatio".to_owned(),
        "MaximumDrawdown".to_owned(),
    ];
    let mut pipeline = MetricPipeline::new(
        MetricInputKind::PeriodPnl {
            initial_equity: 100.0,
        },
        &names,
        MetricPipelineConfiguration::default(),
        NanPolicy::Omit,
    )
    .unwrap();
    pipeline.extend(&pnl).unwrap();

    let mut total = TotalReturn::new(
        MetricInputKind::PeriodPnl {
            initial_equity: 100.0,
        },
        NanPolicy::Omit,
    )
    .unwrap();
    let mut sharpe = SharpeRatio::new(
        MetricInputKind::PeriodPnl {
            initial_equity: 100.0,
        },
        252.0,
        0.0,
        NanPolicy::Omit,
    )
    .unwrap();
    let mut drawdown = MaximumDrawdown::new(
        MetricInputKind::PeriodPnl {
            initial_equity: 100.0,
        },
        NanPolicy::Omit,
    )
    .unwrap();
    total.extend(&pnl).unwrap();
    sharpe.extend(&pnl).unwrap();
    drawdown.extend(&pnl).unwrap();
    let values = pipeline.compute();
    assert_eq!(values[0].1, total.compute());
    assert_eq!(values[1].1, sharpe.compute());
    assert_eq!(values[2].1, drawdown.compute());
}

#[test]
fn reset_replay_and_selection_order_are_invariant() {
    let returns = [0.01, -0.02, 0.03, 0.0, 0.04];
    let names = vec!["WinRate".to_owned(), "TotalReturn".to_owned()];
    let mut pipeline = MetricPipeline::new(
        MetricInputKind::Returns,
        &names,
        MetricPipelineConfiguration::default(),
        NanPolicy::Omit,
    )
    .unwrap();
    pipeline.extend(&returns).unwrap();
    let first = pipeline.compute();
    assert_eq!(pipeline.metric_names(), vec!["WinRate", "TotalReturn"]);
    pipeline.reset();
    assert_eq!(pipeline.len(), 0);
    pipeline.extend(&returns).unwrap();
    assert_eq!(pipeline.compute(), first);
}

#[test]
fn rejects_duplicate_and_unknown_metrics_before_processing() {
    let duplicate = vec!["TotalReturn".to_owned(), "TotalReturn".to_owned()];
    assert!(MetricPipeline::new(
        MetricInputKind::Returns,
        &duplicate,
        MetricPipelineConfiguration::default(),
        NanPolicy::Omit
    )
    .is_err());
    let unknown = vec!["NotAMetric".to_owned()];
    assert!(MetricPipeline::new(
        MetricInputKind::Returns,
        &unknown,
        MetricPipelineConfiguration::default(),
        NanPolicy::Omit
    )
    .is_err());
}
