//! Batch implementation for `order_block`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes the causal order block series.
/// Parameters: aligned input slices followed by indicator parameters.
/// Returns: an aligned series, with NaN during warm-up, or a parameter error.
pub fn order_block(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    volume: &[f64],
    swing_length: usize,
    internal_length: usize,
    atr_period: usize,
    threshold: f64,
) -> TaResult<(Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>)> {
    if high.len() != low.len() || low.len() != close.len() || close.len() != volume.len() {
        return Err(TaError::LengthMismatch {
            expected: high.len(),
            got: low.len().max(close.len()).max(volume.len()),
        });
    }
    let mut state = OrderBlock::new(swing_length, internal_length, atr_period, threshold)?;
    let mut ob_out = Vec::with_capacity(high.len());
    let mut top = Vec::with_capacity(high.len());
    let mut bottom = Vec::with_capacity(high.len());
    let mut ob_volume = Vec::with_capacity(high.len());
    let mut mitigated = Vec::with_capacity(high.len());
    for ((((&high, &low), &close), &volume), _) in high
        .iter()
        .zip(low)
        .zip(close)
        .zip(volume)
        .zip(std::iter::repeat(()))
    {
        let value = state.append(high, low, close, volume);
        ob_out.push(value.ob);
        top.push(value.top);
        bottom.push(value.bottom);
        ob_volume.push(value.ob_volume);
        mitigated.push(value.mitigated);
    }
    Ok((ob_out, top, bottom, ob_volume, mitigated))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::{AverageTrueRange, SwingHighLow};

    #[derive(Debug, Clone, Copy)]
    struct OracleZone {
        direction: f64,
        top: f64,
        bottom: f64,
    }

    /// The pre-optimisation `OrderBlock::append` body, kept verbatim.
    struct Oracle {
        atr: AverageTrueRange,
        internal: SwingHighLow,
        structure: SwingHighLow,
        internal_low: Option<(f64, f64, bool)>,
        internal_high: Option<(f64, f64, bool)>,
        structure_low: Option<f64>,
        structure_high: Option<f64>,
        threshold: f64,
        zones: Vec<OracleZone>,
    }

    impl Oracle {
        fn new(
            swing_length: usize,
            internal_length: usize,
            atr_period: usize,
            threshold: f64,
        ) -> Self {
            Self {
                atr: AverageTrueRange::new(atr_period).unwrap(),
                internal: SwingHighLow::new(internal_length).unwrap(),
                structure: SwingHighLow::new(swing_length).unwrap(),
                internal_low: None,
                internal_high: None,
                structure_low: None,
                structure_high: None,
                threshold,
                zones: Vec::new(),
            }
        }

        fn append(&mut self, high: f64, low: f64, close: f64, volume: f64) -> [f64; 5] {
            let atr = self.atr.append(high, low, close);
            let volatile = atr.is_some_and(|atr| high - low >= self.threshold * atr);

            let mut ob = f64::NAN;
            let mut top = f64::NAN;
            let mut bottom = f64::NAN;
            let mut ob_volume = f64::NAN;

            if let Some(internal_swing) = self.internal.append(high, low) {
                match internal_swing.signal {
                    signal if signal > 0.0 => {
                        self.internal_high = Some((internal_swing.level, volume, volatile));
                        if let Some(structure_high) = self.structure_high {
                            if internal_swing.level > structure_high
                                && self.internal_low.is_some_and(|(_, _, volatile)| !volatile)
                            {
                                let (low_level, low_volume, _) =
                                    self.internal_low.expect("internal low is set");
                                ob = 1.0;
                                top = internal_swing.level;
                                bottom = low_level;
                                ob_volume = low_volume;
                                self.zones.push(OracleZone {
                                    direction: ob,
                                    top,
                                    bottom,
                                });
                                self.structure_high = Some(internal_swing.level);
                            }
                        }
                    }
                    signal if signal < 0.0 => {
                        self.internal_low = Some((internal_swing.level, volume, volatile));
                        if let Some(structure_low) = self.structure_low {
                            if internal_swing.level < structure_low
                                && self.internal_high.is_some_and(|(_, _, volatile)| !volatile)
                            {
                                let (high_level, high_volume, _) =
                                    self.internal_high.expect("internal high is set");
                                ob = -1.0;
                                top = high_level;
                                bottom = internal_swing.level;
                                ob_volume = high_volume;
                                self.zones.push(OracleZone {
                                    direction: ob,
                                    top,
                                    bottom,
                                });
                                self.structure_low = Some(internal_swing.level);
                            }
                        }
                    }
                    _ => {}
                }
            }

            if let Some(structure_swing) = self.structure.append(high, low) {
                match structure_swing.signal {
                    signal if signal > 0.0 => self.structure_high = Some(structure_swing.level),
                    signal if signal < 0.0 => self.structure_low = Some(structure_swing.level),
                    _ => {}
                }
            }

            let mut mitigated = f64::NAN;
            self.zones.retain(|zone| {
                let filled = (zone.direction > 0.0 && low <= zone.bottom)
                    || (zone.direction < 0.0 && high >= zone.top);
                if filled {
                    mitigated = zone.direction;
                }
                !filled
            });

            [ob, top, bottom, ob_volume, mitigated]
        }
    }

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

    fn ohlcv(n: usize, seed: u64) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
        let base = lcg_series(n, seed);
        let spread = lcg_series(n, seed ^ 0xABCD);
        let raw_volume = lcg_series(n, seed ^ 0x1234);
        let mut high = Vec::with_capacity(n);
        let mut low = Vec::with_capacity(n);
        let mut close = Vec::with_capacity(n);
        let mut volume = Vec::with_capacity(n);
        for bar in 0..n {
            let half = (spread[bar] - 90.0) / 20.0 * 0.8 + 0.05;
            high.push(base[bar] + half);
            low.push(base[bar] - half);
            close.push(base[bar]);
            volume.push(raw_volume[bar] * 100.0);
        }
        (high, low, close, volume)
    }

    #[test]
    fn streaming_matches_the_previous_detector_bitwise() {
        let (high, low, close, volume) = ohlcv(5_000, 0x0B10_0001);
        // `threshold = 0` makes every bar volatile (the excluded-block path);
        // a large threshold makes none volatile (every block emitted).
        for (swing_length, internal_length, atr_period, threshold) in [
            (1usize, 1usize, 1usize, 2.0f64),
            (5, 2, 14, 0.0),
            (5, 5, 14, 2.0),
            (50, 5, 200, 2.0),
            (50, 5, 200, 1_000.0),
            (10, 3, 5, 1.5),
        ] {
            let mut state =
                OrderBlock::new(swing_length, internal_length, atr_period, threshold).unwrap();
            let mut oracle = Oracle::new(swing_length, internal_length, atr_period, threshold);
            for bar in 0..high.len() {
                let actual = state.append(high[bar], low[bar], close[bar], volume[bar]);
                let expected = oracle.append(high[bar], low[bar], close[bar], volume[bar]);
                let label =
                    format!("{swing_length}/{internal_length}/{atr_period}/{threshold} bar {bar}");
                assert_eq!(actual.ob.to_bits(), expected[0].to_bits(), "{label} ob");
                assert_eq!(actual.top.to_bits(), expected[1].to_bits(), "{label} top");
                assert_eq!(
                    actual.bottom.to_bits(),
                    expected[2].to_bits(),
                    "{label} bottom"
                );
                assert_eq!(
                    actual.ob_volume.to_bits(),
                    expected[3].to_bits(),
                    "{label} volume"
                );
                assert_eq!(
                    actual.mitigated.to_bits(),
                    expected[4].to_bits(),
                    "{label} mitigated"
                );
            }
        }
    }

    #[test]
    fn batch_matches_per_bar_streaming_bitwise() {
        let (high, low, close, volume) = ohlcv(5_000, 0x0B10_0002);
        let (ob, top, bottom, ob_volume, mitigated) =
            order_block(&high, &low, &close, &volume, 50, 5, 200, 2.0).unwrap();
        let mut state = OrderBlock::new(50, 5, 200, 2.0).unwrap();
        for bar in 0..high.len() {
            let expected = state.append(high[bar], low[bar], close[bar], volume[bar]);
            assert_eq!(ob[bar].to_bits(), expected.ob.to_bits(), "bar {bar}");
            assert_eq!(top[bar].to_bits(), expected.top.to_bits(), "bar {bar}");
            assert_eq!(
                bottom[bar].to_bits(),
                expected.bottom.to_bits(),
                "bar {bar}"
            );
            assert_eq!(
                ob_volume[bar].to_bits(),
                expected.ob_volume.to_bits(),
                "bar {bar}"
            );
            assert_eq!(
                mitigated[bar].to_bits(),
                expected.mitigated.to_bits(),
                "bar {bar}"
            );
        }
    }

    #[test]
    fn reset_restores_a_fresh_state() {
        let (high, low, close, volume) = ohlcv(1_000, 0x0B10_0003);
        let mut state = OrderBlock::new(50, 5, 200, 2.0).unwrap();
        for bar in 0..high.len() {
            state.append(high[bar], low[bar], close[bar], volume[bar]);
        }
        state.reset();
        assert!(state.value().is_none());
        let mut fresh = OrderBlock::new(50, 5, 200, 2.0).unwrap();
        let (high, low, close, volume) = ohlcv(1_000, 0x0B10_0004);
        for bar in 0..high.len() {
            let after_reset = state.append(high[bar], low[bar], close[bar], volume[bar]);
            let from_fresh = fresh.append(high[bar], low[bar], close[bar], volume[bar]);
            assert_eq!(after_reset.ob.to_bits(), from_fresh.ob.to_bits());
            assert_eq!(after_reset.top.to_bits(), from_fresh.top.to_bits());
            assert_eq!(after_reset.bottom.to_bits(), from_fresh.bottom.to_bits());
            assert_eq!(
                after_reset.ob_volume.to_bits(),
                from_fresh.ob_volume.to_bits()
            );
            assert_eq!(
                after_reset.mitigated.to_bits(),
                from_fresh.mitigated.to_bits()
            );
        }
    }
}
