use super::system_quality_number::SystemQualityNumber;
use crate::{MetricInputKind, NanPolicy};

fn expected_system_quality_number(trades: &[f64]) -> f64 {
    let count = trades.len() as f64;
    let mean = trades.iter().sum::<f64>() / count;
    let squared_deviation_sum = trades
        .iter()
        .map(|trade| {
            let deviation = trade - mean;
            deviation * deviation
        })
        .sum::<f64>();
    let sample_standard_deviation = (squared_deviation_sum / (count - 1.0)).sqrt();
    count.sqrt() * mean / sample_standard_deviation
}

fn assert_close(actual: f64, expected: f64) {
    assert!((actual - expected).abs() <= 1e-12, "{actual} != {expected}");
}

#[test]
fn computes_vectorbt_formula_and_preserves_lifecycle() {
    let trades = [100.0, -40.0, 20.0, -10.0, 80.0];
    let expected = expected_system_quality_number(&trades);
    let mut state = SystemQualityNumber::new(MetricInputKind::Trades, NanPolicy::Omit).unwrap();

    assert_eq!(state.value(), None);
    assert_eq!(state.append(trades[0]).unwrap(), None);
    assert!(state.append(trades[1]).unwrap().is_some());
    state.extend(&trades[2..4]).unwrap();
    assert_close(state.append(trades[4]).unwrap().unwrap(), expected);
    assert_close(state.compute().unwrap(), expected);
    assert_eq!(state.len(), trades.len());

    state.reset();
    assert!(state.is_empty());
    assert_eq!(state.value(), None);
    assert_close(state.extend(&trades).unwrap().unwrap(), expected);
}

#[test]
fn retains_the_sign_of_mean_trade_pnl() {
    let losing = [-100.0, 20.0, -50.0, 10.0];
    let mut state = SystemQualityNumber::new(MetricInputKind::Trades, NanPolicy::Omit).unwrap();
    assert_close(
        state.extend(&losing).unwrap().unwrap(),
        expected_system_quality_number(&losing),
    );

    let neutral = [-10.0, 10.0];
    state.reset();
    assert_eq!(state.extend(&neutral).unwrap(), Some(0.0));
}

#[test]
fn insufficient_and_constant_samples_are_undefined() {
    let mut state = SystemQualityNumber::new(MetricInputKind::Trades, NanPolicy::Omit).unwrap();
    assert_eq!(state.append(10.0).unwrap(), None);
    assert_eq!(state.append(10.0).unwrap(), None);
    assert_eq!(state.append(10.0).unwrap(), None);
}

#[test]
fn missing_and_invalid_values_follow_the_input_contract() {
    let mut omit = SystemQualityNumber::new(MetricInputKind::Trades, NanPolicy::Omit).unwrap();
    omit.extend(&[f64::NAN, -10.0, 20.0]).unwrap();
    assert_eq!(omit.len(), 2);
    assert_close(
        omit.value().unwrap(),
        expected_system_quality_number(&[-10.0, 20.0]),
    );

    let mut raise = SystemQualityNumber::new(MetricInputKind::Trades, NanPolicy::Raise).unwrap();
    raise.extend(&[-10.0, 20.0]).unwrap();
    assert!(raise.append(f64::NAN).is_err());
    assert_eq!(raise.len(), 2);

    assert!(omit.append(f64::INFINITY).is_err());
    assert_eq!(omit.len(), 2);
}

#[test]
fn rejects_every_non_trade_domain() {
    assert!(SystemQualityNumber::new(MetricInputKind::Returns, NanPolicy::Omit).is_err());
    assert!(SystemQualityNumber::new(MetricInputKind::LogReturns, NanPolicy::Omit).is_err());
    assert!(SystemQualityNumber::new(MetricInputKind::Equity, NanPolicy::Omit).is_err());
    assert!(SystemQualityNumber::new(MetricInputKind::RawPnl, NanPolicy::Omit).is_err());
    assert!(SystemQualityNumber::new(
        MetricInputKind::PeriodPnl {
            initial_equity: 100.0,
        },
        NanPolicy::Omit,
    )
    .is_err());
}
