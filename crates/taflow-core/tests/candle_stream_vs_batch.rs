//! Bitwise equality between batch candle functions and streaming states.
//!
//! Feeds seeded pseudo-random OHLC bars (LCG, no external crates) through the
//! batch function and the streaming state bar-by-bar and asserts identical
//! output sequences, including warm-up zeros. Streaming `None` during warm-up
//! must correspond to a batch `0`. After the first pass the state is `reset()`
//! and replayed to prove the in-place reset restores a pristine state.

use taflow::stream as s;

/// Minimal 64-bit LCG (Knuth MMIX constants).
struct Lcg(u64);

impl Lcg {
    fn next_f64(&mut self) -> f64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
}

/// Seeded pseudo-random OHLC with regime variety (dojis, long bodies, gaps,
/// varying shadows) so pattern conditions actually fire. Guarantees
/// high >= max(open, close) and low <= min(open, close).
fn gen_ohlc(seed: u64, n: usize) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
    let mut rng = Lcg(seed);
    let mut price = 100.0f64;
    let mut open = Vec::with_capacity(n);
    let mut high = Vec::with_capacity(n);
    let mut low = Vec::with_capacity(n);
    let mut close = Vec::with_capacity(n);
    // Directed-run state: while > 0, emit same-color candles whose opens sit
    // inside the previous body (three-crows/soldiers/mat-hold territory).
    let mut run_left = 0usize;
    let mut run_dir = 1.0f64;
    let mut prev_open = price;
    let mut prev_close = price;
    for i in 0..n {
        if run_left == 0 && rng.next_f64() < 0.08 {
            run_left = 3 + (rng.next_f64() * 6.0) as usize;
            run_dir = if rng.next_f64() < 0.5 { 1.0 } else { -1.0 };
        }
        let (o, c, h, l);
        if run_left > 0 && i > 0 {
            run_left -= 1;
            // Open within the previous real body, close beyond the previous
            // close in the run direction; mostly tiny shadows.
            let body_lo = prev_open.min(prev_close);
            let body_hi = prev_open.max(prev_close);
            o = body_lo + rng.next_f64() * (body_hi - body_lo);
            c = prev_close + run_dir * (0.1 + rng.next_f64() * 1.5);
            let shadow_scale = if rng.next_f64() < 0.7 { 0.03 } else { 1.0 };
            h = o.max(c) + rng.next_f64() * shadow_scale;
            l = o.min(c) - rng.next_f64() * shadow_scale;
        } else {
            let regime = rng.next_f64();
            // Mix tiny bodies (doji-like), normal, and very long bodies.
            let body_scale = if regime < 0.3 {
                0.05
            } else if regime < 0.8 {
                1.0
            } else {
                3.0
            };
            // Occasional gaps relative to the previous close.
            let gap = (rng.next_f64() - 0.5) * 2.0;
            o = price + gap;
            c = o + (rng.next_f64() - 0.5) * 2.0 * body_scale;
            // Shadows: sometimes near-zero, sometimes long.
            let shadow_scale = if rng.next_f64() < 0.4 { 0.02 } else { 1.5 };
            h = o.max(c) + rng.next_f64() * shadow_scale;
            l = o.min(c) - rng.next_f64() * shadow_scale;
        }
        open.push(o);
        high.push(h);
        low.push(l);
        close.push(c);
        prev_open = o;
        prev_close = c;
        price = c;
    }
    splice_scenarios(&mut open, &mut high, &mut low, &mut close);
    (open, high, low, close)
}

/// Handcrafted (o, h, l, c) blocks that trigger the rarest patterns
/// (mat hold, rise/fall three methods, three line strike, three stars in
/// south, concealing baby swallow), each preceded by 12 calm context bars so
/// the rolling body/shadow averages are controlled. Spliced at fixed offsets
/// over the random series so the equality test has teeth for every pattern.
fn splice_scenarios(open: &mut [f64], high: &mut [f64], low: &mut [f64], close: &mut [f64]) {
    const CALM: (f64, f64, f64, f64) = (100.0, 100.3, 99.9, 100.2);
    let blocks: [&[(f64, f64, f64, f64)]; 5] = [
        // Three stars in the south (bullish)
        &[
            (101.0, 101.1, 96.0, 99.0),
            (100.5, 100.6, 95.5, 99.5),
            (99.8, 99.85, 99.55, 99.6),
        ],
        // Mat hold (bullish, penetration 0.5)
        &[
            (100.0, 102.05, 99.95, 102.0),
            (102.5, 102.6, 102.1, 102.2),
            (101.9, 101.95, 101.65, 101.7),
            (101.6, 101.65, 101.35, 101.4),
            (101.5, 103.05, 101.45, 103.0),
        ],
        // Rising three methods (bullish)
        &[
            (100.0, 102.1, 99.9, 102.0),
            (101.9, 101.95, 101.5, 101.6),
            (101.5, 101.55, 101.1, 101.2),
            (101.1, 101.15, 100.7, 100.8),
            (100.9, 102.55, 100.85, 102.5),
        ],
        // Three line strike (bullish)
        &[
            (100.0, 100.9, 99.95, 100.8),
            (100.5, 101.4, 100.45, 101.3),
            (101.0, 101.9, 100.95, 101.8),
            (101.9, 101.95, 99.7, 99.8),
        ],
        // Concealing baby swallow (bullish)
        &[
            (100.5, 100.51, 99.49, 99.5),
            (99.4, 99.41, 98.39, 98.4),
            (98.3, 98.5, 97.95, 98.0),
            (98.6, 98.65, 97.75, 97.8),
        ],
    ];
    let mut at = 60usize;
    for block in blocks {
        for k in 0..12 {
            let (o, h, l, c) = CALM;
            open[at + k] = o;
            high[at + k] = h;
            low[at + k] = l;
            close[at + k] = c;
        }
        for (k, &(o, h, l, c)) in block.iter().enumerate() {
            open[at + 12 + k] = o;
            high[at + 12 + k] = h;
            low[at + 12 + k] = l;
            close[at + 12 + k] = c;
        }
        at += 60;
    }
}

/// Compares two score sequences and reports the first divergence rather than
/// dumping two thousand-element vectors.
#[track_caller]
fn assert_scores_eq(got: &[i32], want: &[i32], what: &str) {
    assert_eq!(got.len(), want.len(), "{what}: length differs");
    if let Some(i) = (0..got.len()).find(|&i| got[i] != want[i]) {
        panic!(
            "{what}: first divergence at bar {i}: got {}, want {}",
            got[i], want[i]
        );
    }
}

const BARS: usize = 2_000;
const SEEDS: [u64; 3] = [1, 0x5EED, 987654321];
/// Bars appended one-at-a-time after a bulk pass, to prove the rebuilt state
/// keeps producing what an uninterrupted per-bar run would have produced.
const TAIL: usize = 64;
/// Bulk chunk sizes. `BARS` is the single-shot (from-empty) fast path; the rest
/// force the "state already warm" fallback from the second chunk onward, and 1
/// makes every call after the first a single-bar fallback.
const CHUNKS: [usize; 8] = [BARS, 1, 7, 10, 97, 251, 1000, 1013];
/// Mirrors `pattern::BULK_REPLAY_BARS`, the tail a route (a) bulk pass
/// replays to rebuild its state. Not public, so it is restated here.
const REPLAY: usize = 64;

macro_rules! check_pattern {
    ($test:ident, $batch:ident, $state:ident, $route:ident $(, $pen:expr)?) => {
        #[test]
        fn $test() {
            for seed in SEEDS {
                let (o, h, l, c) = gen_ohlc(seed, BARS);
                let mut expected_state = s::$state::new();
                let expected: Vec<i32> = (0..BARS)
                    .map(|i| expected_state.append(o[i], h[i], l[i], c[i]).unwrap_or(0))
                    .collect();
                assert_eq!(expected.len(), BARS);
                let mut state = s::$state::new();
                for pass in 0..2 {
                    for i in 0..BARS {
                        match state.append(o[i], h[i], l[i], c[i]) {
                            Some(v) => assert_eq!(
                                v, expected[i],
                                "{} seed {} pass {} bar {}",
                                stringify!($state), seed, pass, i
                            ),
                            None => assert_eq!(
                                expected[i], 0,
                                "{} seed {} pass {} bar {}: batch nonzero during stream warm-up",
                                stringify!($state), seed, pass, i
                            ),
                        }
                    }
                    // Replay after an in-place reset: must be bit-identical.
                    state.reset();
                }

                // ---- window-boundedness leg -----------------------------
                // Route (a)'s fast path rebuilds the state by replaying only
                // the trailing REPLAY bars, so it is sound exactly when the
                // state is a function of a bounded window of recent bars. Prove
                // that directly: a state warm-started REPLAY bars before a cut
                // must be indistinguishable from one fed the whole history.
                // Route (b) states carry unbounded history and are exempt.
                if stringify!($route) == "a" {
                    const CUT: usize = 1_000;
                    let mut whole = s::$state::new();
                    let mut warm = s::$state::new();
                    for i in 0..CUT {
                        whole.append(o[i], h[i], l[i], c[i]);
                        if i >= CUT - REPLAY {
                            warm.append(o[i], h[i], l[i], c[i]);
                        }
                    }
                    for i in CUT..BARS {
                        assert_eq!(
                            whole.append(o[i], h[i], l[i], c[i]),
                            warm.append(o[i], h[i], l[i], c[i]),
                            "{} seed {} bar {}: state is not bounded by the last {} bars, \
                             so the route (a) tail replay cannot reconstruct it",
                            stringify!($state), seed, i, REPLAY
                        );
                    }
                }

                // ---- bulk leg -------------------------------------------
                // Reference: an uninterrupted per-bar run over BARS + TAIL
                // bars, split into the part a bulk call would cover and the
                // per-bar continuation that follows it.
                let (o, h, l, c) = gen_ohlc(seed, BARS + TAIL);
                let mut reference = s::$state::new();
                let per_bar: Vec<i32> = (0..BARS)
                    .map(|i| reference.append(o[i], h[i], l[i], c[i]).unwrap_or(0))
                    .collect();
                let after_bulk: Vec<i32> = (BARS..BARS + TAIL)
                    .map(|i| reference.append(o[i], h[i], l[i], c[i]).unwrap_or(0))
                    .collect();
                let reference_value = reference.value();

                for chunk in CHUNKS {
                    let mut state = s::$state::new();
                    let mut got = Vec::new();
                    for start in (0..BARS).step_by(chunk) {
                        let end = (start + chunk).min(BARS);
                        state
                            .extend_slices_into(
                                &o[start..end],
                                &h[start..end],
                                &l[start..end],
                                &c[start..end],
                                &mut got,
                            )
                            .unwrap();
                    }
                    assert_scores_eq(&got, &per_bar, &format!(
                        "{} seed {} chunk {}: bulk output vs per-bar append",
                        stringify!($state), seed, chunk
                    ));

                    // The state left behind must be indistinguishable from the
                    // one a per-bar run would have left.
                    let continued: Vec<i32> = (BARS..BARS + TAIL)
                        .map(|i| state.append(o[i], h[i], l[i], c[i]).unwrap_or(0))
                        .collect();
                    assert_scores_eq(&continued, &after_bulk, &format!(
                        "{} seed {} chunk {}: continuation after bulk",
                        stringify!($state), seed, chunk
                    ));
                    assert_eq!(
                        state.value(), reference_value,
                        "{} seed {} chunk {}: value() after bulk diverges",
                        stringify!($state), seed, chunk
                    );
                }

                // A bulk pass over an empty slice is a no-op, and `reset()`
                // must restore the pristine state the fast path keys off.
                let mut state = s::$state::new();
                let mut got = Vec::new();
                state.extend_slices_into(&[], &[], &[], &[], &mut got).unwrap();
                assert!(got.is_empty(), "{} seed {}: empty bulk emitted output", stringify!($state), seed);
                state.extend_slices_into(&o, &h, &l, &c, &mut got).unwrap();
                state.reset();
                got.clear();
                state.extend_slices_into(&o[..BARS], &h[..BARS], &l[..BARS], &c[..BARS], &mut got).unwrap();
                assert_scores_eq(&got, &per_bar, &format!(
                    "{} seed {}: bulk after reset vs per-bar append",
                    stringify!($state), seed
                ));
            }
        }
    };
}

check_pattern!(two_crows, candle_two_crows, CandleTwoCrows, a);
check_pattern!(
    three_black_crows,
    candle_three_black_crows,
    CandleThreeBlackCrows,
    b
);
check_pattern!(three_inside, candle_three_inside, CandleThreeInside, a);
check_pattern!(
    three_line_strike,
    candle_three_line_strike,
    CandleThreeLineStrike,
    a
);
check_pattern!(three_outside, candle_three_outside, CandleThreeOutside, a);
check_pattern!(
    three_stars_in_south,
    candle_three_stars_in_south,
    CandleThreeStarsInSouth,
    a
);
check_pattern!(
    three_white_soldiers,
    candle_three_white_soldiers,
    CandleThreeWhiteSoldiers,
    a
);
check_pattern!(
    abandoned_baby,
    candle_abandoned_baby,
    CandleAbandonedBaby,
    a,
    0.3
);
check_pattern!(advance_block, candle_advance_block, CandleAdvanceBlock, a);
check_pattern!(belt_hold, candle_belt_hold, CandleBeltHold, a);
check_pattern!(breakaway, candle_breakaway, CandleBreakaway, a);
check_pattern!(
    closing_marubozu,
    candle_closing_marubozu,
    CandleClosingMarubozu,
    a
);
check_pattern!(
    conceal_baby_swall,
    candle_conceal_baby_swall,
    CandleConcealBabySwall,
    a
);
check_pattern!(counterattack, candle_counterattack, CandleCounterAttack, a);
check_pattern!(
    dark_cloud_cover,
    candle_dark_cloud_cover,
    CandleDarkCloudCover,
    a,
    0.5
);
check_pattern!(doji, candle_doji, CandleDoji, a);
check_pattern!(doji_star, candle_doji_star, CandleDojiStar, a);
check_pattern!(
    dragonfly_doji,
    candle_dragonfly_doji,
    CandleDragonflyDoji,
    a
);
check_pattern!(engulfing, candle_engulfing, CandleEngulfing, a);
check_pattern!(
    evening_doji_star,
    candle_evening_doji_star,
    CandleEveningDojiStar,
    a,
    0.3
);
check_pattern!(evening_star, candle_evening_star, CandleEveningStar, a, 0.3);
check_pattern!(
    gap_side_side_white,
    candle_gap_side_side_white,
    CandleGapSideSideWhite,
    a
);
check_pattern!(
    gravestone_doji,
    candle_gravestone_doji,
    CandleGravestoneDoji,
    a
);
check_pattern!(hammer, candle_hammer, CandleHammer, a);
check_pattern!(hanging_man, candle_hanging_man, CandleHangingMan, a);
check_pattern!(harami_cross, candle_harami_cross, CandleHaramiCross, a);
check_pattern!(harami, candle_harami, CandleHarami, a);
check_pattern!(high_wave, candle_high_wave, CandleHighWave, a);
check_pattern!(
    hikkake_modified,
    candle_hikkake_modified,
    CandleHikkakeModified,
    b
);
check_pattern!(hikkake, candle_hikkake, CandleHikkake, b);
check_pattern!(homing_pigeon, candle_homing_pigeon, CandleHomingPigeon, a);
check_pattern!(
    identical_three_crows,
    candle_identical_three_crows,
    CandleIdenticalThreeCrows,
    a
);
check_pattern!(in_neck, candle_in_neck, CandleInNeck, a);
check_pattern!(
    inverted_hammer,
    candle_inverted_hammer,
    CandleInvertedHammer,
    a
);
check_pattern!(
    kicking_by_length,
    candle_kicking_by_length,
    CandleKickingByLength,
    a
);
check_pattern!(kicking, candle_kicking, CandleKicking, a);
check_pattern!(ladder_bottom, candle_ladder_bottom, CandleLadderBottom, a);
check_pattern!(
    long_legged_doji,
    candle_long_legged_doji,
    CandleLongLeggedDoji,
    a
);
check_pattern!(long_line, candle_long_line, CandleLongLine, a);
check_pattern!(marubozu, candle_marubozu, CandleMarubozu, a);
check_pattern!(matching_low, candle_matching_low, CandleMatchingLow, a);
check_pattern!(mat_hold, candle_mat_hold, CandleMatHold, a, 0.5);
check_pattern!(
    morning_doji_star,
    candle_morning_doji_star,
    CandleMorningDojiStar,
    a,
    0.3
);
check_pattern!(morning_star, candle_morning_star, CandleMorningStar, a, 0.3);
check_pattern!(on_neck, candle_on_neck, CandleOnNeck, a);
check_pattern!(piercing, candle_piercing, CandlePiercing, a);
check_pattern!(rickshawman, candle_rickshawman, CandleRickshawman, a);
check_pattern!(
    rise_fall_three_methods,
    candle_rise_fall_three_methods,
    CandleRiseFallThreeMethods,
    a
);
check_pattern!(
    separating_lines,
    candle_separating_lines,
    CandleSeparatingLines,
    a
);
check_pattern!(shooting_star, candle_shooting_star, CandleShootingStar, a);
check_pattern!(short_line, candle_short_line, CandleShortLine, a);
check_pattern!(spinningtop, candle_spinningtop, CandleSpinningTop, a);
check_pattern!(
    stalled_pattern,
    candle_stalled_pattern,
    CandleStalledPattern,
    a
);
check_pattern!(
    stick_sandwich,
    candle_stick_sandwich,
    CandleStickSandwich,
    a
);
check_pattern!(takuri, candle_takuri, CandleTakuri, a);
check_pattern!(tasuki_gap, candle_tasuki_gap, CandleTasukiGap, a);
check_pattern!(thrusting, candle_thrusting, CandleThrusting, a);
check_pattern!(tri_star, candle_tri_star, CandleTriStar, a);
check_pattern!(
    unique_three_river,
    candle_unique_three_river,
    CandleUniqueThreeRiver,
    a
);
check_pattern!(
    upside_gap_two_crows,
    candle_upside_gap_two_crows,
    CandleUpsideGapTwoCrows,
    a
);
check_pattern!(
    xside_gap_three_methods,
    candle_xside_gap_three_methods,
    CandleUpDownSideGapThreeMethods,
    a
);
