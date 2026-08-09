//! Persistent technical indicators for bulk history and realtime continuation.
//!
//! Each TA implementation lives in its own module and retains only the bounded
//! recurrence state required to process newly appended bars.

#[cfg(test)]
pub(crate) mod tests_extrema_support {
    //! Shared adversarial datasets for the extrema-family bit-exactness tests.

    /// Random, monotonic increasing/decreasing, constant, and quantized
    /// (repeated equal extremes) series of the requested length.
    pub(crate) fn datasets(len: usize) -> Vec<Vec<f64>> {
        let mut state = 0x9E3779B97F4A7C15_u64;
        let random: Vec<f64> = (0..len)
            .map(|_| {
                state = state
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                ((state >> 33) % 997) as f64 / 7.0
            })
            .collect();
        let increasing: Vec<f64> = (0..len).map(|i| i as f64 * 0.5).collect();
        let decreasing: Vec<f64> = (0..len).map(|i| (len as f64) - i as f64 * 0.5).collect();
        let constant = vec![13.25_f64; len];
        let quantized: Vec<f64> = (0..len).map(|i| ((i * 7) % 5) as f64).collect();
        vec![random, increasing, decreasing, constant, quantized]
    }

    /// Periods 2/5/30/200 crossed with lengths 0, 1, p-1, p, p+1, 10_000.
    pub(crate) fn periods_and_lengths() -> Vec<(usize, usize)> {
        let mut cases = Vec::new();
        for &period in &[2usize, 5, 30, 200] {
            for &len in &[0usize, 1, period - 1, period, period + 1, 10_000] {
                cases.push((period, len));
            }
        }
        cases
    }
}

pub(crate) mod accumulation_distribution_helper;
pub(crate) mod aroon_rescan;
pub use crate::indicators::{
    ChaikinMoneyFlow, ChandeMomentumOscillator, CloseToCloseSigma, CommodityChannelIndex, Cross,
    Crossover, Crossunder, CumulativeMaximum, CumulativeMinimum, CumulativeSumControlChart,
    DetrendedPriceOscillator, EqualHighsLows, Falling, FractalDimension,
    HilbertTransformDominantCyclePhase, KaufmanAdaptiveMovingAverage, Rising,
    Amihud, AverageDailyDollarValue, HedgeRatio, KnowSureThing, MassIndex,
};
pub(crate) mod cycle;
pub(crate) mod directional;
mod double_exponential_moving_average;
#[cfg(test)]
mod double_exponential_moving_average_test;
mod exponential_moving_average;
#[cfg(test)]
mod exponential_moving_average_test;
mod hilbert_transform_trendline;
#[cfg(test)]
mod hilbert_transform_trendline_test;
mod indicator;
mod math_operator;
mod minus_directional_indicator;
#[cfg(test)]
mod minus_directional_indicator_test;
mod minus_directional_movement;
#[cfg(test)]
mod minus_directional_movement_test;
mod money_flow_index;
#[cfg(test)]
mod money_flow_index_test;
mod moving_average;
mod moving_average_convergence_divergence;
mod moving_average_convergence_divergence_extended;
#[cfg(test)]
mod moving_average_convergence_divergence_extended_test;
mod moving_average_convergence_divergence_fixed;
#[cfg(test)]
mod moving_average_convergence_divergence_fixed_test;
mod moving_average_convergence_divergence_helpers;
#[cfg(test)]
mod moving_average_convergence_divergence_test;
pub(crate) mod moving_average_dispatcher;
#[cfg(test)]
mod moving_average_test;
pub(crate) mod pattern;
mod percentage_price_oscillator;
#[cfg(test)]
mod percentage_price_oscillator_test;
mod plus_directional_indicator;
#[cfg(test)]
mod plus_directional_indicator_test;
mod plus_directional_movement;
#[cfg(test)]
mod plus_directional_movement_test;
pub(crate) mod price_transform;
pub(crate) mod regression;
mod relative_strength_index;
#[cfg(test)]
mod relative_strength_index_test;
pub(crate) mod rolling_extrema;
mod rolling_price;
pub(crate) mod rolling_statistics;
mod session_flags;
pub(crate) mod sorted_ring;
pub(crate) mod statistic;
pub(crate) mod vhgw;
pub use session_flags::session_flags;
mod fast_stochastic_oscillator;
#[cfg(test)]
mod fast_stochastic_oscillator_test;
mod fibonacci_retracement;
#[cfg(test)]
mod fibonacci_retracement_test;
mod helpers;
pub(crate) mod lagged_common;
pub(crate) mod operator_states;
mod relative_momentum_index;
#[cfg(test)]
mod relative_momentum_index_test;
mod simple_moving_average;
#[cfg(test)]
mod simple_moving_average_test;
mod stochastic_oscillator;
#[cfg(test)]
mod stochastic_oscillator_test;
mod stochastic_relative_strength_index;
#[cfg(test)]
mod stochastic_relative_strength_index_test;
mod triangular_moving_average;
#[cfg(test)]
mod triangular_moving_average_test;
mod triple_exponential_average;
#[cfg(test)]
mod triple_exponential_average_test;
mod triple_exponential_moving_average;
#[cfg(test)]
mod triple_exponential_moving_average_test;
mod triple_exponential_rate_of_change;
#[cfg(test)]
mod triple_exponential_rate_of_change_test;
mod ultimate_oscillator;
#[cfg(test)]
mod ultimate_oscillator_test;
mod variable_index_dynamic_average;
#[cfg(test)]
mod variable_index_dynamic_average_test;
mod window;
#[allow(unused_imports)]
pub(crate) use helpers::invalid_period;
pub(crate) use operator_states::{ewm_alpha, validate_period, validate_quantile};
pub(crate) use operator_states::{weighted_mean_slice, ContiguousWindow};
mod weighted_moving_average;
#[cfg(test)]
mod weighted_moving_average_test;

#[allow(unused_imports)]
#[allow(unused_imports)]
pub use double_exponential_moving_average::DoubleExponentialMovingAverage;
pub use exponential_moving_average::ExponentialMovingAverage;
pub use fibonacci_retracement::{FibonacciRetracement, FibonacciRetracementValue};
pub use hilbert_transform_trendline::HilbertTransformTrendline;
pub use indicator::StreamingIndicator;
#[allow(unused_imports)]
#[allow(unused_imports)]
pub use minus_directional_indicator::MinusDirectionalIndicator;
pub use minus_directional_movement::MinusDirectionalMovement;
pub use money_flow_index::MoneyFlowIndex;
pub use moving_average::MovingAverage;
pub use moving_average_convergence_divergence::{
    MovingAverageConvergenceDivergence, MovingAverageConvergenceDivergenceValue,
};
pub use moving_average_convergence_divergence_extended::MovingAverageConvergenceDivergenceExtended;
pub use moving_average_convergence_divergence_fixed::MovingAverageConvergenceDivergenceFixed;
#[allow(unused_imports)]
pub use percentage_price_oscillator::PercentagePriceOscillator;
pub use plus_directional_indicator::PlusDirectionalIndicator;
pub use plus_directional_movement::PlusDirectionalMovement;
pub use relative_momentum_index::RelativeMomentumIndex;
#[allow(unused_imports)]
pub(crate) use rolling_extrema::{MonotonicMax, MonotonicMin, RollingExtrema};

#[allow(unused_imports)]
pub use fast_stochastic_oscillator::{FastStochasticOscillator, FastStochasticOscillatorValue};
#[allow(unused_imports)]
pub use relative_strength_index::RelativeStrengthIndex;
pub use simple_moving_average::SimpleMovingAverage;
#[allow(unused_imports)]
pub use stochastic_oscillator::{StochasticOscillator, StochasticOscillatorValue};
#[allow(unused_imports)]
pub use stochastic_relative_strength_index::{
    StochasticRelativeStrengthIndex, StochasticRelativeStrengthIndexValue,
};
pub use triangular_moving_average::TriangularMovingAverage;
#[allow(unused_imports)]
pub use triple_exponential_average::TripleExponentialAverage;
#[allow(unused_imports)]
pub use triple_exponential_moving_average::TripleExponentialMovingAverage;
pub use triple_exponential_rate_of_change::TripleExponentialRateOfChange;
#[allow(unused_imports)]
pub use ultimate_oscillator::UltimateOscillator;
pub use variable_index_dynamic_average::VariableIndexDynamicAverage;
pub use weighted_moving_average::WeightedMovingAverage;
pub use window::Window;

#[cfg(test)]
mod bars_since_test;
#[cfg(test)]
#[cfg(test)]
mod donchian_test;
#[cfg(test)]
#[cfg(test)]
mod entry_exit_test;
#[cfg(test)]
#[cfg(test)]
#[cfg(test)]
mod exponentially_weighted_standard_deviation_test;
#[cfg(test)]
#[cfg(test)]
mod exponentially_weighted_variance_test;
#[cfg(test)]
mod gap_down_test;
#[cfg(test)]
mod gap_up_test;
#[cfg(test)]
mod highest_since_test;
#[cfg(test)]
mod keltner_channels_test;
#[cfg(test)]
mod lowest_since_test;
#[cfg(test)]
mod outside_bar_test;
#[cfg(test)]
mod position_hold_test;
#[cfg(test)]
mod signal_delay_test;
#[cfg(test)]
#[cfg(test)]
mod value_when_test;
#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::indicators::{
        MedianPrice, RollingAverageDeviation, RollingBeta, RollingCorrelation,
        RollingLinearRegression, RollingLinearRegressionAngle, RollingLinearRegressionIntercept,
        RollingLinearRegressionSlope, RollingMaximum, RollingMaximumIndex, RollingMidpoint,
        RollingMidprice, RollingMinMax, RollingMinMaxIndex, RollingMinimum, RollingMinimumIndex,
        RollingStandardDeviation, RollingSum, RollingTimeSeriesForecast, RollingVariance,
        TypicalPrice, WeightedClose,
    };

    fn assert_optional_eq(actual: Option<f64>, expected: f64) {
        if expected.is_nan() {
            assert_eq!(actual, None);
        } else {
            assert!((actual.expect("expected a warm value") - expected).abs() < 1e-12);
        }
    }

    #[test]
    fn window_evicts_without_growing() {
        let mut window = Window::new(2).unwrap();
        assert_eq!(window.push(1.0), None);
        assert_eq!(window.push(2.0), None);
        assert_eq!(window.push(3.0), Some(1.0));
        assert_eq!(window.len(), 2);
    }

    #[test]
    fn scalar_states_match_batch_for_every_bar() {
        let input: Vec<f64> = (0..80)
            .map(|i| 100.0 + (i as f64 * 0.37).sin() * 8.0 + i as f64 * 0.05)
            .collect();
        let mut sma_batch_state = SimpleMovingAverage::new(7).unwrap();
        let mut sma_batch = Vec::new();
        sma_batch_state.extend_slice_into(&input, &mut sma_batch);
        let mut ema_batch_state = ExponentialMovingAverage::new(7).unwrap();
        let mut ema_batch = Vec::new();
        ema_batch_state.extend_slice_into(&input, &mut ema_batch);
        let mut wma_batch_state = WeightedMovingAverage::new(7).unwrap();
        let mut wma_batch = Vec::new();
        wma_batch_state.extend_slice_into(&input, &mut wma_batch);
        let mut dema_batch_state = DoubleExponentialMovingAverage::new(7).unwrap();
        let mut dema_batch = Vec::new();
        dema_batch_state.extend_slice_into(&input, &mut dema_batch);
        let mut tema_batch_state = TripleExponentialMovingAverage::new(7).unwrap();
        let mut tema_batch = Vec::new();
        tema_batch_state.extend_slice_into(&input, &mut tema_batch);
        let mut trima_batch_state = TriangularMovingAverage::new(7).unwrap();
        let mut trima_batch = Vec::new();
        trima_batch_state.extend_slice_into(&input, &mut trima_batch);
        let mut kama_batch_state = KaufmanAdaptiveMovingAverage::new(7).unwrap();
        let mut kama_batch = Vec::new();
        kama_batch_state.extend_slice_into(&input, &mut kama_batch);
        let mut midpoint_batch_state = RollingMidpoint::new(7).unwrap();
        let mut midpoint_batch = Vec::new();
        midpoint_batch_state.extend_slice_into(&input, &mut midpoint_batch);
        let mut cmo_batch_state = ChandeMomentumOscillator::new(14).unwrap();
        let mut cmo_batch = Vec::new();
        cmo_batch_state.extend_slice_into(&input, &mut cmo_batch);
        let mut sma = SimpleMovingAverage::new(7).unwrap();
        let mut ema = ExponentialMovingAverage::new(7).unwrap();
        let mut wma = WeightedMovingAverage::new(7).unwrap();
        let mut dema = DoubleExponentialMovingAverage::new(7).unwrap();
        let mut tema = TripleExponentialMovingAverage::new(7).unwrap();
        let mut trima = TriangularMovingAverage::new(7).unwrap();
        let mut kama = KaufmanAdaptiveMovingAverage::new(7).unwrap();
        let mut midpoint = RollingMidpoint::new(7).unwrap();
        let mut cmo = ChandeMomentumOscillator::new(14).unwrap();

        for index in 0..input.len() {
            let value = input[index];
            assert_optional_eq(sma.append(value), sma_batch[index]);
            assert_optional_eq(ema.append(value), ema_batch[index]);
            assert_optional_eq(wma.append(value), wma_batch[index]);
            assert_optional_eq(dema.append(value), dema_batch[index]);
            assert_optional_eq(tema.append(value), tema_batch[index]);
            assert_optional_eq(trima.append(value), trima_batch[index]);
            assert_optional_eq(kama.append(value), kama_batch[index]);
            assert_optional_eq(midpoint.append(value), midpoint_batch[index]);
            assert_optional_eq(cmo.append(value), cmo_batch[index]);
        }
    }

    #[test]
    fn midprice_matches_batch_for_every_bar() {
        let close: Vec<f64> = (0..80)
            .map(|i| 100.0 + (i as f64 * 0.31).sin() * 5.0)
            .collect();
        let high: Vec<f64> = close
            .iter()
            .enumerate()
            .map(|(i, value)| value + 1.0 + (i % 3) as f64 * 0.2)
            .collect();
        let low: Vec<f64> = close
            .iter()
            .enumerate()
            .map(|(i, value)| value - 0.8 - (i % 4) as f64 * 0.15)
            .collect();
        let mut batch_state = RollingMidprice::new(7).unwrap();
        let mut expected = Vec::new();
        batch_state
            .extend_slices_into(&high, &low, &mut expected)
            .unwrap();
        let mut state = RollingMidprice::new(7).unwrap();
        for index in 0..close.len() {
            assert_optional_eq(state.append(high[index], low[index]), expected[index]);
        }
    }

    #[test]
    fn rolling_math_states_match_batch_including_ties() {
        let input = vec![
            4.0, 2.0, 2.0, 5.0, 3.0, 3.0, 5.0, 1.0, 1.0, 4.0, 4.0, 2.0, 6.0, 6.0, 0.0, 0.0, 5.0,
        ];
        let period = 4;
        let mut max_expected_state = RollingMaximum::new(period).unwrap();
        let mut max_expected = Vec::new();
        max_expected_state.extend_slice_into(&input, &mut max_expected);
        let mut min_expected_state = RollingMinimum::new(period).unwrap();
        let mut min_expected = Vec::new();
        min_expected_state.extend_slice_into(&input, &mut min_expected);
        let mut sum_batch_state = RollingSum::new(period).unwrap();
        let mut sum_expected = Vec::new();
        sum_batch_state.extend_slice_into(&input, &mut sum_expected);
        let mut maxindex_expected_state = RollingMaximumIndex::new(period).unwrap();
        let mut maxindex_expected = Vec::new();
        maxindex_expected_state.extend_slice_into(&input, &mut maxindex_expected);
        let mut minindex_expected_state = RollingMinimumIndex::new(period).unwrap();
        let mut minindex_expected = Vec::new();
        minindex_expected_state.extend_slice_into(&input, &mut minindex_expected);
        let mut minmax_expected_state = RollingMinMax::new(period).unwrap();
        let mut minmax_min = Vec::new();
        let mut minmax_max = Vec::new();
        minmax_expected_state.extend_slices_into(&input, &mut minmax_min, &mut minmax_max);
        let mut minmaxindex_expected_state = RollingMinMaxIndex::new(period).unwrap();
        let mut minidx = Vec::new();
        let mut maxidx = Vec::new();
        minmaxindex_expected_state.extend_slices_into(&input, &mut minidx, &mut maxidx);
        let mut max = RollingMaximum::new(period).unwrap();
        let mut min = RollingMinimum::new(period).unwrap();
        let mut sum = RollingSum::new(period).unwrap();
        let mut maxindex = RollingMaximumIndex::new(period).unwrap();
        let mut minindex = RollingMinimumIndex::new(period).unwrap();
        let mut minmax = RollingMinMax::new(period).unwrap();
        let mut minmaxindex = RollingMinMaxIndex::new(period).unwrap();

        for index in 0..input.len() {
            assert_optional_eq(max.append(input[index]), max_expected[index]);
            assert_optional_eq(min.append(input[index]), min_expected[index]);
            assert_optional_eq(sum.append(input[index]), sum_expected[index]);
            assert_optional_eq(maxindex.append(input[index]), maxindex_expected[index]);
            assert_optional_eq(minindex.append(input[index]), minindex_expected[index]);
            match minmax.append(input[index]) {
                Some(value) => {
                    assert_eq!(value.minimum, minmax_min[index]);
                    assert_eq!(value.maximum, minmax_max[index]);
                }
                None => {
                    assert!(minmax_min[index].is_nan());
                    assert!(minmax_max[index].is_nan());
                }
            }
            let indices = minmaxindex.append(input[index]);
            assert_eq!(indices.minimum as f64, minidx[index]);
            assert_eq!(indices.maximum as f64, maxidx[index]);
        }
    }

    #[test]
    fn rolling_statistic_states_match_batch_for_every_bar() {
        let input: Vec<f64> = (0..96)
            .map(|index| {
                1_000_000.0 + (index as f64 * 0.23).sin() * 11.0 + (index % 5) as f64 * 0.125
            })
            .collect();
        let period = 12;
        let mut avgdev_batch = RollingAverageDeviation::new(period).unwrap();
        let mut avgdev_expected = Vec::new();
        avgdev_batch.extend_slice_into(&input, &mut avgdev_expected);
        let mut var_batch = RollingVariance::new(period, 2.0).unwrap();
        let mut var_expected = Vec::new();
        var_batch.extend_slice_into(&input, &mut var_expected);
        let mut stddev_batch = RollingStandardDeviation::new(period, 2.0).unwrap();
        let mut stddev_expected = Vec::new();
        stddev_batch.extend_slice_into(&input, &mut stddev_expected);
        let mut avgdev = RollingAverageDeviation::new(period).unwrap();
        let mut var = RollingVariance::new(period, 2.0).unwrap();
        let mut stddev = RollingStandardDeviation::new(period, 2.0).unwrap();
        for index in 0..input.len() {
            assert_optional_eq(avgdev.append(input[index]), avgdev_expected[index]);
            assert_optional_eq(var.append(input[index]), var_expected[index]);
            assert_optional_eq(stddev.append(input[index]), stddev_expected[index]);
        }

        let constant = vec![42.0; 30];
        let mut expected_state = RollingStandardDeviation::new(5, 3.0).unwrap();
        let mut expected = Vec::new();
        expected_state.extend_slice_into(&constant, &mut expected);
        let mut state = RollingStandardDeviation::new(5, 3.0).unwrap();
        for (input, expected) in constant.into_iter().zip(expected) {
            assert_optional_eq(state.append(input), expected);
        }
    }

    #[test]
    fn bivariate_statistic_states_match_batch_for_every_bar() {
        let market: Vec<f64> = (0..100)
            .map(|index| 80.0 + index as f64 * 0.08 + (index as f64 * 0.17).sin() * 3.0)
            .collect();
        let asset: Vec<f64> = market
            .iter()
            .enumerate()
            .map(|(index, market)| market * 1.3 + (index as f64 * 0.29).cos() * 2.0)
            .collect();
        let period = 10;
        let mut beta_expected_state = RollingBeta::new(period).unwrap();
        let mut beta_expected = Vec::new();
        beta_expected_state
            .extend_slices_into(&market, &asset, &mut beta_expected)
            .unwrap();
        let mut correl_expected_state = RollingCorrelation::new(period).unwrap();
        let mut correl_expected = Vec::new();
        correl_expected_state
            .extend_slices_into(&market, &asset, &mut correl_expected)
            .unwrap();
        let mut beta = RollingBeta::new(period).unwrap();
        let mut correl = RollingCorrelation::new(period).unwrap();
        for index in 0..market.len() {
            let beta_actual = beta.append(market[index], asset[index]);
            let correl_actual = correl.append(market[index], asset[index]);
            if beta_expected[index].is_nan() {
                assert_eq!(beta_actual, None);
            } else {
                assert!(
                    (beta_actual.unwrap() - beta_expected[index]).abs() < 1e-12,
                    "BETA differs at {index}: {:?} vs {}",
                    beta_actual,
                    beta_expected[index]
                );
            }
            if correl_expected[index].is_nan() {
                assert_eq!(correl_actual, None);
            } else {
                assert!(
                    (correl_actual.unwrap() - correl_expected[index]).abs() < 1e-12,
                    "CORREL differs at {index}: {:?} vs {}",
                    correl_actual,
                    correl_expected[index]
                );
            }
        }
    }

    #[test]
    fn regression_states_match_batch_for_every_bar() {
        let input: Vec<f64> = (0..100)
            .map(|index| 200.0 + index as f64 * 0.4 + (index as f64 * 0.19).sin() * 7.0)
            .collect();
        let period = 14;
        let mut linearreg_expected = Vec::new();
        RollingLinearRegression::new(period)
            .unwrap()
            .extend_slice_into(&input, &mut linearreg_expected);
        let mut slope_expected = Vec::new();
        RollingLinearRegressionSlope::new(period)
            .unwrap()
            .extend_slice_into(&input, &mut slope_expected);
        let mut intercept_expected = Vec::new();
        RollingLinearRegressionIntercept::new(period)
            .unwrap()
            .extend_slice_into(&input, &mut intercept_expected);
        let mut angle_expected = Vec::new();
        RollingLinearRegressionAngle::new(period)
            .unwrap()
            .extend_slice_into(&input, &mut angle_expected);
        let mut tsf_expected = Vec::new();
        RollingTimeSeriesForecast::new(period)
            .unwrap()
            .extend_slice_into(&input, &mut tsf_expected);
        let mut linearreg = RollingLinearRegression::new(period).unwrap();
        let mut slope = RollingLinearRegressionSlope::new(period).unwrap();
        let mut intercept = RollingLinearRegressionIntercept::new(period).unwrap();
        let mut angle = RollingLinearRegressionAngle::new(period).unwrap();
        let mut tsf = RollingTimeSeriesForecast::new(period).unwrap();
        for index in 0..input.len() {
            assert_optional_eq(linearreg.append(input[index]), linearreg_expected[index]);
            assert_optional_eq(slope.append(input[index]), slope_expected[index]);
            assert_optional_eq(intercept.append(input[index]), intercept_expected[index]);
            assert_optional_eq(angle.append(input[index]), angle_expected[index]);
            assert_optional_eq(tsf.append(input[index]), tsf_expected[index]);
        }
    }
}
mod chaikin_volatility;
#[cfg(test)]
mod chaikin_volatility_test;
#[allow(unused_imports)]
mod ease_of_movement;
#[cfg(test)]
mod ease_of_movement_test;
mod fair_value_gap;
#[cfg(test)]
mod fair_value_gap_test;
mod force_index;
#[cfg(test)]
mod force_index_test;
#[allow(unused_imports)]
#[allow(unused_imports)]
mod order_block;
#[cfg(test)]
mod order_block_test;
mod parkinson;
#[cfg(test)]
mod parkinson_test;
mod previous_high_low;
#[cfg(test)]
mod previous_high_low_test;
mod retracements;
#[cfg(test)]
mod retracements_test;
mod rogers_satchell;
#[cfg(test)]
mod rogers_satchell_test;
mod rolling_alpha;
#[cfg(test)]
mod rolling_alpha_test;
mod rolling_autocorr;
#[cfg(test)]
mod rolling_autocorr_test;
mod rolling_information_ratio;
#[cfg(test)]
mod rolling_information_ratio_test;
mod rolling_volume_weighted_average_price;
#[cfg(test)]
mod rolling_volume_weighted_average_price_test;
mod session_extrema;
#[cfg(test)]
mod session_extrema_test;
mod sessions;
#[cfg(test)]
mod sessions_test;
mod time_series_rank;
#[cfg(test)]
mod time_series_rank_test;
mod ulcer_index;
#[cfg(test)]
mod ulcer_index_test;
mod yang_zhang;
#[cfg(test)]
mod yang_zhang_test;
pub use time_series_rank::TimeSeriesRank;
mod signed_power;
#[cfg(test)]
mod signed_power_test;
pub use signed_power::SignedPower;
mod decay_linear;
#[cfg(test)]
mod decay_linear_test;
pub use decay_linear::DecayLinear;
mod frac_diff;
#[cfg(test)]
mod frac_diff_test;
mod ichimoku;
#[cfg(test)]
mod ichimoku_test;
mod ornstein_uhlenbeck_half_life;
#[cfg(test)]
mod ornstein_uhlenbeck_half_life_test;
mod roll_spread;
#[cfg(test)]
mod roll_spread_test;
mod schaff_trend_cycle;
#[cfg(test)]
mod schaff_trend_cycle_test;
mod spread_z_score;
#[cfg(test)]
mod spread_z_score_test;
mod squeeze;
mod squeeze_pro;
#[cfg(test)]
mod squeeze_pro_test;
#[cfg(test)]
mod squeeze_test;
mod supertrend;
#[cfg(test)]
mod supertrend_test;
mod swing_high_low;
#[cfg(test)]
mod swing_high_low_test;
mod swing_highs_lows;
mod vortex;
#[cfg(test)]
mod vortex_test;
pub use swing_high_low::{SwingHighLow, SwingValue};
pub use swing_highs_lows::SwingHighsLows;
mod swing_high;
#[cfg(test)]
mod swing_high_test;
mod swing_highs_lows_indicator;
#[cfg(test)]
mod swing_highs_lows_indicator_test;
#[cfg(test)]
pub(crate) mod swing_highs_lows_test;
pub use swing_high::SwingHigh;
mod swing_low;
#[cfg(test)]
mod swing_low_test;
pub use swing_low::SwingLow;
mod exponentially_weighted_standard_deviation;
mod exponentially_weighted_variance;
mod negative_volume_index;
#[cfg(test)]
mod negative_volume_index_test;
mod positive_volume_index;
#[cfg(test)]
mod positive_volume_index_test;
mod rolling_z_score;
#[cfg(test)]
mod rolling_z_score_test;
#[allow(unused_imports)]
mod volume_price_trend;
#[cfg(test)]
mod volume_price_trend_test;

mod bar_relation;
mod bars_since;
mod gap_down;
mod gap_up;
mod higher_high;
#[cfg(test)]
mod higher_high_test;
mod highest_since;
mod inside_bar;
#[cfg(test)]
mod inside_bar_test;
mod lower_low;
#[cfg(test)]
mod lower_low_test;
mod lowest_since;
mod outside_bar;
mod value_when;
pub use bars_since::BarsSince;
pub use exponentially_weighted_standard_deviation::ExponentiallyWeightedStandardDeviation;
pub use exponentially_weighted_variance::ExponentiallyWeightedVariance;
pub use fair_value_gap::{FairValueGap, FairValueGapValue};
pub use frac_diff::FracDiff;
pub use gap_down::GapDown;
pub use gap_up::GapUp;
pub use higher_high::HigherHigh;
pub use highest_since::HighestSince;
pub use ichimoku::{Ichimoku, IchimokuValue};
pub use inside_bar::InsideBar;
pub use lower_low::LowerLow;
pub use lowest_since::LowestSince;
pub use order_block::{OrderBlock, OrderBlockValue};
pub use ornstein_uhlenbeck_half_life::OrnsteinUhlenbeckHalfLife;
pub use outside_bar::OutsideBar;
pub use parkinson::Parkinson;
pub use previous_high_low::{PreviousHighLow, PreviousHighLowValue};
pub use retracements::{Retracements, RetracementsValue};
pub use rogers_satchell::RogersSatchell;
pub use roll_spread::RollSpread;
pub use rolling_alpha::RollingAlpha;
pub use rolling_autocorr::RollingAutocorr;
pub use rolling_information_ratio::RollingInformationRatio;
pub use rolling_z_score::RollingZScore;
pub use schaff_trend_cycle::{SchaffTrendCycle, SchaffTrendCycleValue};
pub use session_extrema::{SessionExtrema, SessionExtremaValue};
pub use sessions::{Sessions, SessionsValue};
pub use spread_z_score::SpreadZScore;
pub use squeeze::{Squeeze, SqueezeValue};
pub use squeeze_pro::{SqueezePro, SqueezeProValue};
pub use supertrend::{Supertrend, SupertrendValue};
pub use value_when::ValueWhen;
pub use vortex::{Vortex, VortexValue};
pub use yang_zhang::YangZhang;
mod donchian;
pub use donchian::{Donchian, DonchianValue};
mod donchian_channels;
#[cfg(test)]
mod donchian_channels_test;
pub use donchian_channels::DonchianChannels;
pub use ulcer_index::UlcerIndex;
mod keltner_channels;
pub use chaikin_volatility::ChaikinVolatility;
pub use ease_of_movement::EaseOfMovement;
pub use force_index::ForceIndex;
pub use keltner_channels::{KeltnerChannels, KeltnerValue};
pub use rolling_volume_weighted_average_price::RollingVolumeWeightedAveragePrice;
mod signal_delay;
pub use signal_delay::SignalDelay;
mod position_hold;
pub use position_hold::PositionHold;
mod entry_exit;
pub use entry_exit::EntryExit;
pub use negative_volume_index::NegativeVolumeIndex;
pub use positive_volume_index::PositiveVolumeIndex;
pub use volume_price_trend::VolumePriceTrend;
