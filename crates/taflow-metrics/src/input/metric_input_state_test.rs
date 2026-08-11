use super::{MetricInputKind, MetricInputState, NanPolicy};
use approx::assert_relative_eq;

#[test]
fn converts_every_supported_domain_and_tracks_usable_length() {
    let returns = [0.02, -0.02, 0.05];
    let equity = [100.0, 102.0, 99.96, 104.958];
    let pnl = [2.0, -2.04, 4.998];
    let logs = returns.map(f64::ln_1p);

    let mut observed = Vec::new();
    for (kind, values) in [
        (MetricInputKind::Returns, returns.as_slice()),
        (MetricInputKind::LogReturns, logs.as_slice()),
        (MetricInputKind::Equity, equity.as_slice()),
        (
            MetricInputKind::PeriodPnl {
                initial_equity: 100.0,
            },
            pnl.as_slice(),
        ),
    ] {
        let mut state = MetricInputState::new(kind, NanPolicy::Omit).unwrap();
        let converted: Vec<_> = values
            .iter()
            .filter_map(|&value| state.append(value).unwrap())
            .collect();
        assert_eq!(state.len(), 3);
        observed.push(converted);
    }
    for converted in observed {
        for (actual, expected) in converted.iter().zip(returns) {
            assert_relative_eq!(*actual, expected, epsilon = 1e-14);
        }
    }

    for kind in [MetricInputKind::RawPnl, MetricInputKind::Trades] {
        let mut state = MetricInputState::new(kind, NanPolicy::Omit).unwrap();
        assert_eq!(state.append(7.5).unwrap(), Some(7.5));
    }
}

#[test]
fn omission_and_reset_preserve_converter_semantics() {
    let mut state = MetricInputState::new(MetricInputKind::Equity, NanPolicy::Omit).unwrap();
    assert_eq!(state.append(100.0).unwrap(), None);
    assert_eq!(state.append(f64::NAN).unwrap(), None);
    assert_relative_eq!(state.append(110.0).unwrap().unwrap(), 0.1);
    assert_eq!(state.len(), 1);
    state.reset();
    assert!(state.is_empty());
    assert_eq!(state.append(100.0).unwrap(), None);
    assert_relative_eq!(state.append(110.0).unwrap().unwrap(), 0.1);
}

#[test]
fn failures_do_not_mutate_conversion_state() {
    let mut equity = MetricInputState::new(MetricInputKind::Equity, NanPolicy::Raise).unwrap();
    assert_eq!(equity.append(100.0).unwrap(), None);
    assert!(equity.append(f64::NAN).is_err());
    assert_relative_eq!(equity.append(110.0).unwrap().unwrap(), 0.1);

    let mut pnl = MetricInputState::new(
        MetricInputKind::PeriodPnl {
            initial_equity: 100.0,
        },
        NanPolicy::Omit,
    )
    .unwrap();
    assert!(pnl.append(-101.0).is_err());
    assert_eq!(pnl.append(10.0).unwrap(), Some(0.1));
    assert_eq!(pnl.append(-110.0).unwrap(), Some(-1.0));
    assert!(pnl.append(0.0).is_err());
}

#[test]
fn validates_configuration_and_nonfinite_values() {
    assert!(MetricInputState::new(
        MetricInputKind::PeriodPnl {
            initial_equity: 0.0,
        },
        NanPolicy::Omit,
    )
    .is_err());
    assert!(NanPolicy::try_from("propagate").is_err());
    let mut state = MetricInputState::new(MetricInputKind::Returns, NanPolicy::Omit).unwrap();
    assert!(state.append(f64::INFINITY).is_err());
    assert!(state.append(-1.01).is_err());
    assert!(state.is_empty());
    assert_eq!(state.append(-1.0).unwrap(), Some(-1.0));
}
