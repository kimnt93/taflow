use super::downside_deviation::DownsideDeviation;
use crate::{MetricInputKind, NanPolicy};

fn assert_close(actual: f64, expected: f64) {
    assert!((actual - expected).abs() <= 1e-13, "{actual} != {expected}");
}

#[test]
fn computes_lower_partial_moment_over_all_observations() {
    let returns = [-0.02, 0.01, -0.01, 0.03];
    let expected = ((0.02_f64.powi(2) + 0.01_f64.powi(2)) / 4.0).sqrt() * 252.0_f64.sqrt();
    let mut state =
        DownsideDeviation::new(MetricInputKind::Returns, 252.0, 0.0, NanPolicy::Omit).unwrap();

    assert_eq!(state.value(), None);
    assert_close(
        state.append(returns[0]).unwrap().unwrap(),
        0.02 * 252.0_f64.sqrt(),
    );
    state.extend(&returns[1..]).unwrap();
    assert_close(state.value().unwrap(), expected);
    assert_close(state.compute().unwrap(), expected);
    assert_eq!(state.len(), returns.len());

    state.reset();
    assert!(state.is_empty());
    assert_eq!(state.value(), None);
    assert_close(state.extend(&returns).unwrap().unwrap(), expected);
}

#[test]
fn converts_annual_effective_required_return_to_each_period() {
    let annual_required_return: f64 = 0.12682503013196977;
    let periods_per_year: f64 = 12.0;
    let period_required_return = (annual_required_return.ln_1p() / periods_per_year).exp_m1();
    let returns: [f64; 4] = [0.0, 0.02, -0.01, 0.03];
    let squared_shortfall_sum = returns
        .iter()
        .map(|value| (*value - period_required_return).min(0.0).powi(2))
        .sum::<f64>();
    let expected = (squared_shortfall_sum / returns.len() as f64).sqrt() * periods_per_year.sqrt();
    let mut state = DownsideDeviation::new(
        MetricInputKind::Returns,
        periods_per_year,
        annual_required_return,
        NanPolicy::Omit,
    )
    .unwrap();

    assert_close(state.extend(&returns).unwrap().unwrap(), expected);
}

#[test]
fn input_modes_are_equivalent_and_missing_values_are_omitted() {
    let returns = [0.10, -0.20, 0.05];
    let mut direct =
        DownsideDeviation::new(MetricInputKind::Returns, 12.0, 0.03, NanPolicy::Omit).unwrap();
    let expected = direct.extend(&returns).unwrap().unwrap();

    let mut equity =
        DownsideDeviation::new(MetricInputKind::Equity, 12.0, 0.03, NanPolicy::Omit).unwrap();
    assert_close(
        equity.extend(&[100.0, 110.0, 88.0, 92.4]).unwrap().unwrap(),
        expected,
    );

    let mut pnl = DownsideDeviation::new(
        MetricInputKind::PeriodPnl {
            initial_equity: 100.0,
        },
        12.0,
        0.03,
        NanPolicy::Omit,
    )
    .unwrap();
    assert_close(pnl.extend(&[10.0, -22.0, 4.4]).unwrap().unwrap(), expected);

    let mut logarithmic =
        DownsideDeviation::new(MetricInputKind::LogReturns, 12.0, 0.03, NanPolicy::Omit).unwrap();
    assert_close(
        logarithmic
            .extend(&returns.map(f64::ln_1p))
            .unwrap()
            .unwrap(),
        expected,
    );

    let mut missing =
        DownsideDeviation::new(MetricInputKind::Returns, 252.0, 0.0, NanPolicy::Omit).unwrap();
    missing.extend(&[f64::NAN, -0.01, f64::NAN]).unwrap();
    assert_eq!(missing.len(), 1);
    assert_close(missing.value().unwrap(), 0.01 * 252.0_f64.sqrt());
}

#[test]
fn validates_configuration() {
    for periods_per_year in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        assert!(DownsideDeviation::new(
            MetricInputKind::Returns,
            periods_per_year,
            0.0,
            NanPolicy::Omit,
        )
        .is_err());
    }
    for annual_required_return in [-1.0, -2.0, f64::NAN, f64::INFINITY] {
        assert!(DownsideDeviation::new(
            MetricInputKind::Returns,
            252.0,
            annual_required_return,
            NanPolicy::Omit,
        )
        .is_err());
    }
    assert!(DownsideDeviation::new(MetricInputKind::RawPnl, 252.0, 0.0, NanPolicy::Omit,).is_err());
    assert!(DownsideDeviation::new(MetricInputKind::Trades, 252.0, 0.0, NanPolicy::Omit,).is_err());
}
