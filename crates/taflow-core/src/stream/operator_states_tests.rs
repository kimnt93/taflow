#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::*;
    use crate::stream::{RollingMedian, RollingMode};

    fn bulk_lcg_series(n: usize, mut state: u64) -> Vec<f64> {
        (0..n)
            .map(|_| {
                state = state
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                90.0 + (state >> 11) as f64 / (1u64 << 53) as f64 * 20.0
            })
            .collect()
    }

    fn assert_same_bits(actual: &[f64], expected: &[f64], label: &str) {
        assert_eq!(actual.len(), expected.len(), "{label}: length");
        for (i, (a, b)) in actual.iter().zip(expected).enumerate() {
            assert_eq!(a.to_bits(), b.to_bits(), "{label}: bar {i}");
        }
    }

    #[test]
    fn know_sure_thing_bulk_is_bitwise_identical_to_per_bar_append() {
        let input = bulk_lcg_series(5_000, 0x5EED_0357);
        let tail = bulk_lcg_series(128, 0x7A11_0357);
        let combos = [
            (
                10usize, 15usize, 20usize, 30usize, 10usize, 10usize, 10usize, 15usize, 9usize,
            ),
            (1, 1, 1, 1, 1, 1, 1, 1, 1),
            (2, 3, 4, 5, 2, 2, 2, 2, 3),
            (5, 5, 5, 5, 8, 8, 8, 8, 2),
        ];
        for (r1, r2, r3, r4, s1, s2, s3, s4, nsig) in combos {
            let mut per_bar = KnowSureThing::new(r1, r2, r3, r4, s1, s2, s3, s4, nsig).unwrap();
            let mut ref_kst = Vec::new();
            let mut ref_signal = Vec::new();
            for &x in &input {
                let value = per_bar.append(x);
                ref_kst.push(value.kst);
                ref_signal.push(value.signal);
            }
            let mut tail_kst = Vec::new();
            let mut tail_signal = Vec::new();
            for &x in &tail {
                let value = per_bar.append(x);
                tail_kst.push(value.kst);
                tail_signal.push(value.signal);
            }

            for chunk in [usize::MAX, 1, 7, 97] {
                let mut state = KnowSureThing::new(r1, r2, r3, r4, s1, s2, s3, s4, nsig).unwrap();
                let (mut kst_out, mut signal_out) = (Vec::new(), Vec::new());
                for piece in input.chunks(chunk.min(input.len())) {
                    state.extend_slices_into(piece, &mut kst_out, &mut signal_out);
                }
                let label = format!("kst {r1}/{s1}/{nsig} chunk {chunk}");
                assert_same_bits(&kst_out, &ref_kst, &label);
                assert_same_bits(&signal_out, &ref_signal, &label);
                let (mut tk, mut ts) = (Vec::new(), Vec::new());
                for &x in &tail {
                    let value = state.append(x);
                    tk.push(value.kst);
                    ts.push(value.signal);
                }
                assert_same_bits(&tk, &tail_kst, &format!("{label} tail"));
                assert_same_bits(&ts, &tail_signal, &format!("{label} tail"));
            }
        }
    }

    #[test]
    fn schaff_trend_cycle_bulk_is_bitwise_identical_to_per_bar_append() {
        let input = bulk_lcg_series(5_000, 0x5EED_057C);
        let tail = bulk_lcg_series(128, 0x7A11_057C);
        let combos = [
            (10usize, 23usize, 50usize, 0.5),
            (1, 2, 2, 0.5),
            (3, 5, 5, 1.0),
            (2, 2, 30, 0.25),
        ];
        for (tclength, fast, slow, factor) in combos {
            let mut per_bar = SchaffTrendCycle::new(tclength, fast, slow, factor).unwrap();
            let (mut ref_stc, mut ref_macd, mut ref_stoch) = (Vec::new(), Vec::new(), Vec::new());
            for &x in &input {
                let value = per_bar.append(x);
                ref_stc.push(value.stc);
                ref_macd.push(value.macd);
                ref_stoch.push(value.stoch);
            }
            let (mut tail_stc, mut tail_macd, mut tail_stoch) =
                (Vec::new(), Vec::new(), Vec::new());
            for &x in &tail {
                let value = per_bar.append(x);
                tail_stc.push(value.stc);
                tail_macd.push(value.macd);
                tail_stoch.push(value.stoch);
            }

            for chunk in [usize::MAX, 1, 7, 97] {
                let mut state = SchaffTrendCycle::new(tclength, fast, slow, factor).unwrap();
                let (mut s, mut m, mut t) = (Vec::new(), Vec::new(), Vec::new());
                for piece in input.chunks(chunk.min(input.len())) {
                    state.extend_slices_into(piece, &mut s, &mut m, &mut t);
                }
                let label = format!("stc {tclength}/{fast}/{slow} chunk {chunk}");
                assert_same_bits(&s, &ref_stc, &label);
                assert_same_bits(&m, &ref_macd, &label);
                assert_same_bits(&t, &ref_stoch, &label);
                let (mut xs, mut xm, mut xt) = (Vec::new(), Vec::new(), Vec::new());
                for &x in &tail {
                    let value = state.append(x);
                    xs.push(value.stc);
                    xm.push(value.macd);
                    xt.push(value.stoch);
                }
                assert_same_bits(&xs, &tail_stc, &format!("{label} tail"));
                assert_same_bits(&xm, &tail_macd, &format!("{label} tail"));
                assert_same_bits(&xt, &tail_stoch, &format!("{label} tail"));
            }
        }
    }

    #[test]
    fn drawdown_batch_matches_definition() {
        let input = [2.0, 4.0, 1.0, 8.0, 2.0];
        let mut drawdown_state = Drawdown::new();
        let drawdown_values: Vec<f64> = input.iter().map(|&value| drawdown_state.append(value)).collect();
        assert_eq!(drawdown_values, vec![0.0, 0.0, -0.75, 0.0, -0.75]);
    }

    #[test]
    fn rolling_statistics_match_batch_and_reset() {
        let input = vec![1.0, 4.0, 2.0, 2.0, 9.0, 4.0];
        let mut median_state = RollingMedian::new(3).unwrap();
        let mut median = Vec::new();
        median_state.extend_slice_into(&input, &mut median);
        let mut mode_state = RollingMode::new(3).unwrap();
        let mut mode = Vec::new();
        mode_state.extend_slice_into(&input, &mut mode);
        assert!(median[0].is_nan() && median[1].is_nan());
        assert_eq!(&median[2..], &[2.0, 2.0, 2.0, 4.0]);
        assert!(mode[0].is_nan() && mode[1].is_nan());
        assert_eq!(&mode[2..], &[1.0, 2.0, 2.0, 2.0]);

        let mut state = RollingMedian::new(3).unwrap();
        for &value in &input {
            state.append(value);
        }
        state.reset();
        assert!(state.append(7.0).is_none());
    }

    #[test]
    fn rolling_distribution_operators_match_definitions() {
        let input = vec![1.0, 4.0, 2.0, 8.0];
        let mut quantile = RollingQuantile::new(3, 0.5).unwrap();
        let quantile_values: Vec<f64> = input.iter().map(|&value| quantile.append(value).unwrap_or(f64::NAN)).collect();
        assert_eq!(quantile_values[2..], [2.0, 4.0]);
        let mut percentile = RollingPercentile::new(3, 50.0).unwrap();
        let percentile_values: Vec<f64> = input.iter().map(|&value| percentile.append(value).unwrap_or(f64::NAN)).collect();
        assert_eq!(percentile_values[2..], [2.0, 4.0]);
        let mut rank = RollingRank::new(3).unwrap();
        let rank_values: Vec<f64> = input.iter().map(|&value| rank.append(value).unwrap_or(f64::NAN)).collect();
        assert_eq!(rank_values[2..], [2.0 / 3.0, 1.0]);
        let mut zscore = RollingZScore::new(3).unwrap();
        let zscore_values: Vec<f64> = input.iter().map(|&value| zscore.append(value).unwrap_or(f64::NAN)).collect();
        assert!((zscore_values[2] - (-0.2672612419)).abs() < 1e-9);
        let mut iqr = RollingInterquartileRange::new(3).unwrap();
        let iqr_values: Vec<f64> = input.iter().map(|&value| iqr.append(value).unwrap_or(f64::NAN)).collect();
        assert_eq!(iqr_values[2], 1.5);
        let mut covariance = RollingCovariance::new(3).unwrap();
        let covariance_value = input
            .iter()
            .zip([2.0, 8.0, 4.0, 16.0])
            .map(|(&x, y)| covariance.append(x, y).unwrap_or(f64::NAN))
            .collect::<Vec<_>>()[2];
        assert!((covariance_value - 28.0 / 9.0).abs() < 1e-12);
        let mut winsorize = RollingWinsorize::new(3, 0.0, 0.5).unwrap();
        let winsorized = input.iter().map(|&value| winsorize.append(value).unwrap_or(f64::NAN)).collect::<Vec<_>>();
        assert_eq!(winsorized[2], 2.0);
        let mut variance = ExponentiallyWeightedVariance::new(2).unwrap();
        let mut standard_deviation = ExponentiallyWeightedStandardDeviation::new(2).unwrap();
        assert_eq!(variance.append(input[0]), 0.0);
        assert_eq!(standard_deviation.append(input[0]), 0.0);
    }

    #[test]
    fn quant_family_batch_and_stream_match() {
        let close = vec![100.0, 102.0, 101.0, 105.0, 107.0, 106.0];
        let volume = vec![1000.0, 1100.0, 900.0, 1200.0, 1300.0, 950.0];

        assert_eq!(
            time_series_rank(&close, 3)
                .unwrap()
                .iter()
                .map(|&x| x.to_bits())
                .collect::<Vec<_>>(),
            {
                let mut state = RollingRank::new(3).unwrap();
                close.iter().map(|&value| state.append(value).unwrap_or(f64::NAN).to_bits()).collect::<Vec<_>>()
            }
        );
        assert_eq!(
            decay_linear(&close, 3)
                .unwrap()
                .iter()
                .map(|&x| x.to_bits())
                .collect::<Vec<_>>(),
            {
                let mut state = crate::stream::WeightedMovingAverage::new(3).unwrap();
                let mut output = Vec::new();
                state.extend_slice_into(&close, &mut output);
                output
            }
                .iter()
                .map(|&x| x.to_bits())
                .collect::<Vec<_>>()
        );
        assert_eq!(signed_power(&[2.0, -3.0, 0.5], 2.0), vec![4.0, -9.0, 0.25]);

        let mut adv_state = AverageDailyDollarValue::new(3).unwrap();
        for (close, volume) in close.iter().zip(&volume) {
            assert_eq!(
                adv_state.append(*close, *volume).map(f64::to_bits),
                adv_state.value().map(f64::to_bits)
            );
        }

        let amihud_batch = amihud(&close, &volume, 3).unwrap();
        let mut amihud_state = Amihud::new(3).unwrap();
        for ((close, volume), expected) in close.iter().zip(&volume).zip(&amihud_batch) {
            assert_eq!(
                amihud_state.append(*close, *volume).map(f64::to_bits),
                (!expected.is_nan()).then_some(expected.to_bits())
            );
        }

        let spread_batch = roll_spread(&close, 3).unwrap();
        let mut spread_state = RollSpread::new(3).unwrap();
        for (price, expected) in close.iter().zip(&spread_batch) {
            assert_eq!(
                spread_state.append(*price).map(f64::to_bits),
                (!expected.is_nan()).then_some(expected.to_bits())
            );
        }

        let hl_batch = ornstein_uhlenbeck_half_life(&close, 3).unwrap();
        let mut hl_state = OrnsteinUhlenbeckHalfLife::new(3).unwrap();
        for (price, expected) in close.iter().zip(&hl_batch) {
            assert_eq!(
                hl_state.append(*price).map(f64::to_bits),
                (!expected.is_nan()).then_some(expected.to_bits())
            );
        }

        let cusum_batch = cumulative_sum_control_chart(&[0.5, -0.5, 2.0, -1.0], 1.0).unwrap();
        assert_eq!(cusum_batch, vec![0.0, 0.0, 1.0, 0.0]);

    }

    #[test]
    fn spread_zscore_matches_hedge_ratio_composition() {
        let x = vec![10.0, 11.0, 9.0, 12.0, 13.0, 11.5];
        let y = vec![20.0, 22.0, 18.5, 23.0, 25.0, 22.0];
        let period = 4;

        let mut z_state = SpreadZScore::new(period).unwrap();
        let z: Vec<f64> = x
            .iter()
            .zip(&y)
            .map(|(&x, &y)| z_state.append(x, y).unwrap_or(f64::NAN))
            .collect();
        assert!(z[..period - 1].iter().all(|&value| value.is_nan()));

        let mut hedge_state = HedgeRatio::new(period).unwrap();
        let beta: Vec<f64> = x
            .iter()
            .zip(&y)
            .map(|(&x, &y)| hedge_state.append(x, y).unwrap_or(f64::NAN))
            .collect();
        for i in period - 1..x.len() {
            let window_x = &x[i + 1 - period..=i];
            let window_y = &y[i + 1 - period..=i];
            let spreads: Vec<f64> = window_x
                .iter()
                .zip(window_y)
                .map(|(&x, &y)| y - beta[i] * x)
                .collect();
            let mean = spreads.iter().sum::<f64>() / period as f64;
            let variance = spreads.iter().map(|&s| (s - mean).powi(2)).sum::<f64>() / period as f64;
            let expected = if variance > 0.0 {
                (spreads[period - 1] - mean) / variance.sqrt()
            } else {
                0.0
            };
            assert!((z[i] - expected).abs() < 1e-9, "index {i}");
        }

        let mut state = SpreadZScore::new(period).unwrap();
        let mut replayed = Vec::new();
        for (&x, &y) in x.iter().zip(&y) {
            replayed.push(state.append(x, y).unwrap_or(f64::NAN));
        }
        assert_eq!(
            replayed.iter().map(|&v| v.to_bits()).collect::<Vec<_>>(),
            z.iter().map(|&v| v.to_bits()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn frac_diff_matches_reference_weights() {
        let d = 0.5;
        let threshold = 1e-3;
        let mut weights = vec![1.0];
        let mut k = 1usize;
        loop {
            let wk = -weights[k - 1] * (d - k as f64 + 1.0) / k as f64;
            if wk.abs() < threshold {
                break;
            }
            weights.push(wk);
            k += 1;
        }
        assert!(
            weights.len() > 2,
            "truncation should retain several weights"
        );

        let input: Vec<f64> = (1..=200).map(|x| x as f64).collect();
        let output = frac_diff(&input, d, threshold).unwrap();
        let w = weights.len();
        assert!(output[..w - 1].iter().all(|&v| v.is_nan()));
        for i in w - 1..input.len() {
            let mut expected = 0.0;
            for (j, &weight) in weights.iter().enumerate() {
                expected += weight * input[i - j];
            }
            assert!((output[i] - expected).abs() < 1e-9, "index {i}");
        }

        let mut state = FracDiff::new(d, threshold).unwrap();
        let replayed: Vec<f64> = input
            .iter()
            .map(|&v| state.append(v).unwrap_or(f64::NAN))
            .collect();
        assert_eq!(
            replayed.iter().map(|&v| v.to_bits()).collect::<Vec<_>>(),
            output.iter().map(|&v| v.to_bits()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn frac_diff_rejects_bad_params() {
        assert!(FracDiff::new(0.0, 1e-5).is_err());
        assert!(FracDiff::new(0.5, 0.0).is_err());
        assert!(FracDiff::new(-1.0, 1e-5).is_err());
    }

    #[test]
    fn kalman_hedge_ratio_tracks_synthetic_beta() {
        let true_beta = 2.0;
        let x: Vec<f64> = (0..200).map(|i| i as f64 / 10.0).collect();
        let y: Vec<f64> = x.iter().map(|&x| 1.0 + true_beta * x).collect();

        let delta = 1e-4;
        let observation_variance = 1e-3;
        let beta = kalman_hedge_ratio(&x, &y, delta, observation_variance).unwrap();
        assert_eq!(beta.len(), x.len());
        assert!((beta[0] - 1.0).abs() < 1e-9);
        assert!(
            (beta[beta.len() - 1] - true_beta).abs() < 0.1,
            "final beta {}",
            beta[beta.len() - 1]
        );

        let mut state = KalmanHedgeRatio::new(delta, observation_variance).unwrap();
        let replayed: Vec<f64> = x
            .iter()
            .zip(&y)
            .map(|(&x, &y)| state.append(x, y).unwrap_or(f64::NAN))
            .collect();
        assert_eq!(
            replayed.iter().map(|&v| v.to_bits()).collect::<Vec<_>>(),
            beta.iter().map(|&v| v.to_bits()).collect::<Vec<_>>()
        );
        assert!(state.alpha().unwrap().abs() < 2.0);
        assert!(state.innovation().is_some());
        assert!(state.std().unwrap() > 0.0);

        state.reset();
        assert!(state.append(1.0, 3.0).is_some());
        assert!(state.value().unwrap() > 1.0);
    }

    #[test]
    fn kalman_hedge_ratio_rejects_bad_params() {
        assert!(KalmanHedgeRatio::new(-0.1, 1.0).is_err());
        assert!(KalmanHedgeRatio::new(0.0, 0.0).is_err());
        assert_eq!(
            kalman_hedge_ratio(&[1.0, 2.0], &[1.0], 1e-4, 1e-3),
            Err(TaError::LengthMismatch {
                expected: 2,
                got: 1
            })
        );
    }

    #[test]
    fn quant_family_rejects_bad_periods() {
        assert!(AverageDailyDollarValue::new(0).is_err());
        assert!(Amihud::new(0).is_err());
        assert!(RollSpread::new(0).is_err());
        assert!(OrnsteinUhlenbeckHalfLife::new(0).is_err());
        assert!(CumulativeSumControlChart::new(-1.0).is_err());
    }

    #[test]
    fn supertrend_batch_and_stream_match() {
        let high: Vec<f64> = (0..200)
            .map(|i| 52.0 + (i as f64 * 0.3).sin() * 5.0 + (i as f64 * 0.01).cos())
            .collect();
        let low: Vec<f64> = high.iter().map(|&h| h - 2.0).collect();
        let close: Vec<f64> = high
            .iter()
            .enumerate()
            .map(|(i, &h)| h - 1.0 + (i as f64 * 0.05).sin())
            .collect();

        let (trend, direction, long, short) = supertrend(&high, &low, &close, 7, 3.0).unwrap();
        assert!(trend[..6].iter().all(|&value| value.is_nan()));
        assert!(trend[6..].iter().all(|&value| value.is_finite()));
        assert!(direction[6..]
            .iter()
            .all(|&value| value == 1.0 || value == -1.0));

        let mut state = Supertrend::new(7, 3.0).unwrap();
        let replayed: Vec<f64> = high
            .iter()
            .zip(&low)
            .zip(&close)
            .map(|((&h, &l), &c)| state.append(h, l, c).map_or(f64::NAN, |v| v.trend))
            .collect();
        assert_eq!(
            replayed.iter().map(|&v| v.to_bits()).collect::<Vec<_>>(),
            trend.iter().map(|&v| v.to_bits()).collect::<Vec<_>>()
        );

        let mut flipped = 0;
        for pair in direction.windows(2) {
            if pair[0] != pair[1] {
                flipped += 1;
            }
        }
        assert!(
            flipped >= 2,
            "expected direction flips on the synthetic series"
        );
    }

    #[test]
    fn supertrend_rejects_bad_params() {
        assert!(Supertrend::new(0, 3.0).is_err());
        assert!(Supertrend::new(7, 0.0).is_err());
        assert!(Supertrend::new(7, -1.0).is_err());
        assert_eq!(
            supertrend(&[1.0, 2.0], &[1.0], &[1.0, 2.0], 7, 3.0),
            Err(TaError::LengthMismatch {
                expected: 2,
                got: 1
            })
        );
    }

    #[test]
    fn ichimoku_batch_and_stream_match() {
        let high: Vec<f64> = (0..200)
            .map(|i| 52.0 + (i as f64 * 0.3).sin() * 5.0)
            .collect();
        let low: Vec<f64> = high.iter().map(|&h| h - 2.0).collect();
        let close: Vec<f64> = high
            .iter()
            .enumerate()
            .map(|(i, &h)| h - 1.0 + (i as f64 * 0.02).sin())
            .collect();

        let (tenkan, kijun, span_a, span_b, chikou) =
            ichimoku(&high, &low, &close, 9, 26, 52).unwrap();
        assert!(tenkan[..8].iter().all(|&v| v.is_nan()));
        assert!(kijun[..25].iter().all(|&v| v.is_nan()));
        assert!(span_a[..25].iter().all(|&v| v.is_nan()));
        assert!(span_b[..51].iter().all(|&v| v.is_nan()));
        assert!(tenkan[8..].iter().all(|&v| v.is_finite()));
        assert!(span_b[51..].iter().all(|&v| v.is_finite()));

        // span_a = 0.5 * (tenkan + kijun); chikou = current close (causal).
        for i in 25..close.len() {
            assert!((span_a[i] - 0.5 * (tenkan[i] + kijun[i])).abs() < 1e-12);
            assert_eq!(chikou[i], close[i]);
        }

        let mut state = Ichimoku::new(9, 26, 52).unwrap();
        let replayed: Vec<f64> = high
            .iter()
            .zip(&low)
            .zip(&close)
            .map(|((&h, &l), &c)| state.append(h, l, c).span_b)
            .collect();
        assert_eq!(
            replayed.iter().map(|&v| v.to_bits()).collect::<Vec<_>>(),
            span_b.iter().map(|&v| v.to_bits()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn ichimoku_rejects_bad_params() {
        assert!(Ichimoku::new(0, 26, 52).is_err());
        assert!(Ichimoku::new(9, 0, 52).is_err());
        assert!(Ichimoku::new(9, 26, 0).is_err());
        assert_eq!(
            ichimoku(&[1.0], &[1.0], &[1.0, 2.0], 9, 26, 52),
            Err(TaError::LengthMismatch {
                expected: 1,
                got: 1
            })
        );
    }

    #[test]
    fn squeeze_batch_and_stream_match() {
        let high: Vec<f64> = (0..240)
            .map(|i| 52.0 + (i as f64 * 0.31).sin() * 6.0 + (i as f64 * 0.015).cos())
            .collect();
        let low: Vec<f64> = high.iter().map(|&h| h - 3.0).collect();
        let close: Vec<f64> = high
            .iter()
            .enumerate()
            .map(|(i, &h)| h - 1.5 + (i as f64 * 0.07).sin())
            .collect();

        let (squeeze, on, off, no) = squeeze(&high, &low, &close, 20, 2.0, 20, 1.5, 12, 6).unwrap();
        assert!(squeeze[..16].iter().all(|&v| v.is_nan()));
        assert!(squeeze[17..].iter().all(|&v| v.is_finite()));
        assert!(on[..19].iter().all(|&v| v == 0.0));
        assert!(off[..19].iter().all(|&v| v == 0.0));
        assert!(no[..19].iter().all(|&v| v == 1.0));
        assert!(on[19..].iter().all(|&v| v == 0.0 || v == 1.0));
        assert!(off[19..].iter().all(|&v| v == 0.0 || v == 1.0));
        assert!(no[19..].iter().all(|&v| v == 0.0 || v == 1.0));
        for i in 19..close.len() {
            assert_eq!(on[i] + off[i] + no[i], 1.0);
        }

        let mut state = Squeeze::new(20, 2.0, 20, 1.5, 12, 6).unwrap();
        let replayed: Vec<f64> = high
            .iter()
            .zip(&low)
            .zip(&close)
            .map(|((&h, &l), &c)| state.append(h, l, c).squeeze)
            .collect();
        assert_eq!(
            replayed.iter().map(|&v| v.to_bits()).collect::<Vec<_>>(),
            squeeze.iter().map(|&v| v.to_bits()).collect::<Vec<_>>()
        );

        let mut state = Squeeze::new(20, 2.0, 20, 1.5, 12, 6).unwrap();
        let replayed_on: Vec<f64> = high
            .iter()
            .zip(&low)
            .zip(&close)
            .map(|((&h, &l), &c)| state.append(h, l, c).on)
            .collect();
        assert_eq!(
            replayed_on.iter().map(|&v| v.to_bits()).collect::<Vec<_>>(),
            on.iter().map(|&v| v.to_bits()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn squeeze_rejects_bad_params() {
        assert!(Squeeze::new(0, 2.0, 20, 1.5, 12, 6).is_err());
        assert!(Squeeze::new(20, 2.0, 0, 1.5, 12, 6).is_err());
        assert!(Squeeze::new(20, 2.0, 20, 0.0, 12, 6).is_err());
        assert!(Squeeze::new(20, 0.0, 20, 1.5, 12, 6).is_err());
        assert_eq!(
            squeeze(&[1.0, 2.0], &[1.0], &[1.0, 2.0], 20, 2.0, 20, 1.5, 12, 6),
            Err(TaError::LengthMismatch {
                expected: 2,
                got: 1
            })
        );
    }

    #[test]
    fn squeeze_pro_batch_and_stream_match() {
        let high: Vec<f64> = (0..240)
            .map(|i| 52.0 + (i as f64 * 0.31).sin() * 6.0 + (i as f64 * 0.015).cos())
            .collect();
        let low: Vec<f64> = high.iter().map(|&h| h - 3.0).collect();
        let close: Vec<f64> = high
            .iter()
            .enumerate()
            .map(|(i, &h)| h - 1.5 + (i as f64 * 0.07).sin())
            .collect();

        let (sq, on_wide, on_normal, on_narrow, off, no) =
            squeeze_pro(&high, &low, &close, 20, 2.0, 20, 2.0, 1.5, 1.0, 12, 6).unwrap();
        assert!(sq[..16].iter().all(|&v| v.is_nan()));
        assert!(sq[17..].iter().all(|&v| v.is_finite()));
        for column in [&on_wide, &on_normal, &on_narrow, &off, &no] {
            assert!(column[19..].iter().all(|&v| v == 0.0 || v == 1.0));
        }

        let mut state = SqueezePro::new(20, 2.0, 20, 2.0, 1.5, 1.0, 12, 6).unwrap();
        let replayed: Vec<f64> = high
            .iter()
            .zip(&low)
            .zip(&close)
            .map(|((&h, &l), &c)| state.append(h, l, c).on_narrow)
            .collect();
        assert_eq!(
            replayed.iter().map(|&v| v.to_bits()).collect::<Vec<_>>(),
            on_narrow.iter().map(|&v| v.to_bits()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn squeeze_pro_rejects_bad_params() {
        assert!(SqueezePro::new(0, 2.0, 20, 2.0, 1.5, 1.0, 12, 6).is_err());
        assert!(SqueezePro::new(20, 2.0, 20, 1.5, 1.5, 1.0, 12, 6).is_err());
        assert!(SqueezePro::new(20, 2.0, 20, 2.0, 1.5, 2.0, 12, 6).is_err());
        assert!(SqueezePro::new(20, 2.0, 20, 2.0, 1.5, 0.0, 12, 6).is_err());
        assert_eq!(
            squeeze_pro(
                &[1.0, 2.0],
                &[1.0],
                &[1.0, 2.0],
                20,
                2.0,
                20,
                2.0,
                1.5,
                1.0,
                12,
                6
            ),
            Err(TaError::LengthMismatch {
                expected: 2,
                got: 1
            })
        );
    }

    #[test]
    fn stc_batch_and_stream_match() {
        let close: Vec<f64> = (0..300)
            .map(|i| 100.0 + (i as f64 * 0.07).sin() * 8.0 + (i as f64 * 0.013) * 2.0)
            .collect();

        let mut batch_state = SchaffTrendCycle::new(10, 12, 26, 0.5).unwrap();
        let values: Vec<SchaffTrendCycleValue> = close.iter().map(|&value| batch_state.append(value)).collect();
        let stc: Vec<f64> = values.iter().map(|value| value.stc).collect();
        let macd: Vec<f64> = values.iter().map(|value| value.macd).collect();
        let stoch: Vec<f64> = values.iter().map(|value| value.stoch).collect();
        assert_eq!(stc[0], 0.0);
        assert_eq!(stoch[0], 0.0);
        assert!(macd[..24].iter().all(|&v| v.is_nan()));
        assert!(macd[25..].iter().all(|&v| v.is_finite()));
        assert!(stc
            .iter()
            .all(|&v| v.is_finite() && (0.0..=100.0).contains(&v)));
        assert!(stoch
            .iter()
            .all(|&v| v.is_finite() && (0.0..=100.0).contains(&v)));

        let mut state = SchaffTrendCycle::new(10, 12, 26, 0.5).unwrap();
        let replayed: Vec<f64> = close.iter().map(|&c| state.append(c).stc).collect();
        assert_eq!(
            replayed.iter().map(|&v| v.to_bits()).collect::<Vec<_>>(),
            stc.iter().map(|&v| v.to_bits()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn stc_swaps_fast_slow_and_rejects_bad_params() {
        let close: Vec<f64> = (0..200).map(|i| 100.0 + (i as f64 * 0.03).cos()).collect();
        let mut state_a = SchaffTrendCycle::new(10, 12, 26, 0.5).unwrap();
        let mut state_b = SchaffTrendCycle::new(10, 26, 12, 0.5).unwrap();
        let a: Vec<f64> = close.iter().map(|&value| state_a.append(value).stc).collect();
        let b: Vec<f64> = close.iter().map(|&value| state_b.append(value).stc).collect();
        assert_eq!(a, b);

        assert!(SchaffTrendCycle::new(0, 12, 26, 0.5).is_err());
        assert!(SchaffTrendCycle::new(10, 0, 26, 0.5).is_err());
        assert!(SchaffTrendCycle::new(10, 12, 26, 0.0).is_err());
    }

    #[test]
    fn vortex_batch_and_stream_match() {
        let high: Vec<f64> = (0..240)
            .map(|i| 52.0 + (i as f64 * 0.31).sin() * 5.0)
            .collect();
        let low: Vec<f64> = high.iter().map(|&h| h - 2.5).collect();
        let close: Vec<f64> = high
            .iter()
            .enumerate()
            .map(|(i, &h)| h - 1.2 + (i as f64 * 0.05).sin())
            .collect();

        let (vp, vn) = vortex(&high, &low, &close, 14).unwrap();
        assert!(vp[..13].iter().all(|&v| v.is_nan()));
        assert!(vn[..13].iter().all(|&v| v.is_nan()));
        assert!(vp[14..].iter().all(|&v| v.is_finite() && v >= 0.0));
        assert!(vn[14..].iter().all(|&v| v.is_finite() && v >= 0.0));

        let mut state = Vortex::new(14).unwrap();
        let replayed: Vec<f64> = high
            .iter()
            .zip(&low)
            .zip(&close)
            .map(|((&h, &l), &c)| state.append(h, l, c).vp)
            .collect();
        assert_eq!(
            replayed.iter().map(|&v| v.to_bits()).collect::<Vec<_>>(),
            vp.iter().map(|&v| v.to_bits()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn vortex_rejects_bad_params() {
        assert!(Vortex::new(0).is_err());
        assert_eq!(
            vortex(&[1.0, 2.0], &[1.0], &[1.0, 2.0], 14),
            Err(TaError::LengthMismatch {
                expected: 2,
                got: 1
            })
        );
    }

    #[test]
    fn kst_batch_and_stream_match() {
        let close: Vec<f64> = (0..400)
            .map(|i| 100.0 + (i as f64 * 0.05).sin() * 6.0 + i as f64 * 0.01)
            .collect();

        let (kst, signal) = know_sure_thing(&close, 10, 15, 20, 30, 10, 10, 10, 15, 9).unwrap();
        assert!(kst[..43].iter().all(|&v| v.is_nan()));
        assert!(signal[..43].iter().all(|&v| v.is_nan()));
        assert!(kst[44..].iter().all(|&v| v.is_finite()));
        assert!(signal[52..].iter().all(|&v| v.is_finite()));

        let mut state = KnowSureThing::new(10, 15, 20, 30, 10, 10, 10, 15, 9).unwrap();
        let replayed: Vec<f64> = close.iter().map(|&c| state.append(c).kst).collect();
        assert_eq!(
            replayed.iter().map(|&v| v.to_bits()).collect::<Vec<_>>(),
            kst.iter().map(|&v| v.to_bits()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn kst_rejects_bad_params() {
        assert!(KnowSureThing::new(0, 15, 20, 30, 10, 10, 10, 15, 9).is_err());
        assert!(KnowSureThing::new(10, 15, 20, 30, 10, 10, 10, 15, 0).is_err());
    }

    #[test]
    fn mass_index_batch_and_stream_match() {
        let high: Vec<f64> = (0..200)
            .map(|i| 100.0 + i as f64 * 0.2 + (i as f64 * 0.13).sin())
            .collect();
        let low: Vec<f64> = high.iter().map(|value| value - 2.0).collect();
        let batch = mass_index(&high, &low, 9, 25).unwrap();
        let mut state = MassIndex::new(9, 25).unwrap();
        let replayed: Vec<f64> = high
            .iter()
            .zip(&low)
            .map(|(&high, &low)| state.append(high, low).unwrap_or(f64::NAN))
            .collect();
        assert_eq!(
            batch.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
            replayed.iter().map(|v| v.to_bits()).collect::<Vec<_>>()
        );
        assert!(batch[..40].iter().all(|value| value.is_nan()));
        assert!(batch[40..].iter().all(|value| value.is_finite()));
    }

    #[test]
    fn dpo_batch_and_stream_match() {
        let input: Vec<f64> = (0..100)
            .map(|i| i as f64 + (i as f64 * 0.2).sin())
            .collect();
        let mut batch_state = DetrendedPriceOscillator::new(20).unwrap();
        let batch: Vec<f64> = input
            .iter()
            .map(|&value| batch_state.append(value).unwrap_or(f64::NAN))
            .collect();
        let mut state = DetrendedPriceOscillator::new(20).unwrap();
        let replayed: Vec<f64> = input
            .iter()
            .map(|&value| state.append(value).unwrap_or(f64::NAN))
            .collect();
        assert_eq!(
            batch.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
            replayed.iter().map(|v| v.to_bits()).collect::<Vec<_>>()
        );
        assert!(batch[..30].iter().all(|value| value.is_nan()));
        assert!(batch[30..].iter().all(|value| value.is_finite()));
    }

    #[test]
    fn cmf_batch_and_stream_match() {
        let close: Vec<f64> = (0..100).map(|i| 100.0 + i as f64 * 0.1).collect();
        let high: Vec<f64> = close.iter().map(|value| value + 1.0).collect();
        let low: Vec<f64> = close.iter().map(|value| value - 1.0).collect();
        let volume: Vec<f64> = (1..=100).map(|value| value as f64).collect();
        let batch = chaikin_money_flow(&high, &low, &close, &volume, 20).unwrap();
        let mut state = ChaikinMoneyFlow::new(20).unwrap();
        let replayed: Vec<f64> = high
            .iter()
            .zip(&low)
            .zip(&close)
            .zip(&volume)
            .map(|(((&h, &l), &c), &v)| state.append(h, l, c, v).unwrap_or(f64::NAN))
            .collect();
        assert_eq!(
            batch.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
            replayed.iter().map(|v| v.to_bits()).collect::<Vec<_>>()
        );
        assert!(batch[..19].iter().all(|value| value.is_nan()));
        assert!(batch[19..].iter().all(|value| value.is_finite()));
    }

    #[test]
    fn vpt_batch_and_stream_match() {
        let close: Vec<f64> = (1..=100).map(|value| value as f64).collect();
        let volume: Vec<f64> = (1..=100).map(|value| value as f64).collect();
        let batch = volume_price_trend(&close, &volume).unwrap();
        let mut state = VolumePriceTrend::new();
        let replayed: Vec<f64> = close
            .iter()
            .zip(&volume)
            .map(|(&close, &volume)| state.append(close, volume).unwrap_or(f64::NAN))
            .collect();
        assert_eq!(
            batch.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
            replayed.iter().map(|v| v.to_bits()).collect::<Vec<_>>()
        );
        assert!(batch[0].is_nan());
        assert!(batch[1..].iter().all(|value| value.is_finite()));
    }
}

#[cfg(test)]
mod donchian_bulk_tests {
    use crate::stream::Donchian;

    fn lcg_series(len: usize, seed: u64) -> Vec<f64> {
        let mut state = seed;
        (0..len)
            .map(|_| {
                state = state
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                ((state >> 33) % 100_003) as f64 / 101.0
            })
            .collect()
    }

    #[test]
    fn donchian_bulk_matches_append_bitwise() {
        let base = lcg_series(5_000, 0x00DC_1A11_2233_4455);
        let high: Vec<f64> = base.iter().map(|v| v + 0.5).collect();
        let low: Vec<f64> = base.iter().map(|v| v - 0.5).collect();
        for period in [2usize, 5, 14, 30, 200] {
            let mut reference = Donchian::new(period).unwrap();
            let expected: Vec<(f64, f64, f64)> = (0..base.len())
                .map(|i| match reference.append(high[i], low[i]) {
                    Some(value) => (value.upper, value.lower, value.middle),
                    None => (f64::NAN, f64::NAN, f64::NAN),
                })
                .collect();
            for chunk in [1usize, 7, 97, base.len()] {
                let mut state = Donchian::new(period).unwrap();
                let (mut upper, mut lower, mut middle) = (Vec::new(), Vec::new(), Vec::new());
                let mut offset = 0;
                while offset < base.len() {
                    let end = (offset + chunk).min(base.len());
                    state
                        .extend_slices_into(
                            &high[offset..end],
                            &low[offset..end],
                            &mut upper,
                            &mut lower,
                            &mut middle,
                        )
                        .unwrap();
                    offset = end;
                }
                assert_eq!(upper.len(), base.len());
                for (i, (eu, el, em)) in expected.iter().enumerate() {
                    assert_eq!(
                        eu.to_bits(),
                        upper[i].to_bits(),
                        "upper p={period} c={chunk} i={i}"
                    );
                    assert_eq!(
                        el.to_bits(),
                        lower[i].to_bits(),
                        "lower p={period} c={chunk} i={i}"
                    );
                    assert_eq!(
                        em.to_bits(),
                        middle[i].to_bits(),
                        "middle p={period} c={chunk} i={i}"
                    );
                }
                let mut follow = reference.clone();
                for i in 0..256 {
                    assert_eq!(
                        follow.append(high[i], low[i]),
                        state.append(high[i], low[i]),
                        "continue p={period} c={chunk}"
                    );
                }
            }
        }
    }

    #[test]
    fn donchian_bulk_validates_lengths() {
        let mut state = Donchian::new(3).unwrap();
        let (mut u, mut l, mut m) = (Vec::new(), Vec::new(), Vec::new());
        assert!(state
            .extend_slices_into(&[1.0, 2.0], &[1.0], &mut u, &mut l, &mut m)
            .is_err());
    }
}

#[cfg(test)]
mod rolling_zscore_tests {
    use crate::stream::RollingZScore;

    fn lcg_series(n: usize, mut state: u64) -> Vec<f64> {
        (0..n)
            .map(|_| {
                state = state
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                90.0 + (state >> 11) as f64 / (1u64 << 53) as f64 * 20.0
            })
            .collect()
    }

    /// Fresh two-pass z-score over `input[end + 1 - period..=end]`.
    fn exact_zscore(input: &[f64], end: usize, period: usize) -> f64 {
        let window = &input[end + 1 - period..=end];
        let period_f = period as f64;
        let mut sum = 0.0;
        for &value in window {
            sum += value;
        }
        let mean = sum / period_f;
        let mut variance = 0.0;
        for &value in window {
            variance += (value - mean) * (value - mean);
        }
        variance /= period_f;
        if variance > 0.0 {
            (input[end] - mean) / variance.sqrt()
        } else {
            0.0
        }
    }

    /// `RollingZScore` slides no accumulator, so it carries no drift at all:
    /// every bar is already a fresh window recomputation. This test pins that
    /// property so a future "optimisation" to O(1) sliding sums cannot silently
    /// introduce the drift the other rolling-moment states have to manage.
    #[test]
    fn streaming_has_zero_drift_over_1m_bars() {
        let input = lcg_series(1_000_000, 0x2500_D21F);
        for period in [14usize, 30] {
            let mut state = RollingZScore::new(period).unwrap();
            for i in 0..input.len() {
                let Some(value) = state.append(input[i]) else {
                    continue;
                };
                if (i + 1) % 50_000 != 0 && i + 1 != input.len() {
                    continue;
                }
                let exact = exact_zscore(&input, i, period);
                let drift = (value - exact).abs();
                assert!(
                    drift < 1e-12,
                    "RollingZScore p{period} bar {i}: drift {drift:e} vs a fresh window"
                );
            }
        }
    }

    /// Chunked `append` replay stays bitwise identical (there is no bulk kernel
    /// to diverge, but the state must not depend on where a run is split).
    #[test]
    fn chunked_replay_is_bitwise_identical() {
        let input = lcg_series(5_000, 0x2500_5EED);
        for period in [2usize, 14, 200] {
            let mut reference_state = RollingZScore::new(period).unwrap();
            let reference: Vec<f64> = input
                .iter()
                .map(|&x| reference_state.append(x).unwrap_or(f64::NAN))
                .collect();
            for chunk in [1usize, 7, 10, 97, 1000] {
                let mut state = RollingZScore::new(period).unwrap();
                let mut actual = Vec::new();
                for piece in input.chunks(chunk) {
                    for &x in piece {
                        actual.push(state.append(x).unwrap_or(f64::NAN));
                    }
                }
                for (i, (a, b)) in actual.iter().zip(&reference).enumerate() {
                    assert_eq!(
                        a.to_bits(),
                        b.to_bits(),
                        "zscore p{period} c{chunk} bar {i}"
                    );
                }
            }
        }
    }
}
