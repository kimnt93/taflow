//! Shared van Herk–Gil–Werman (vHGW) sliding-window extrema kernels.
//!
//! Comparison-only bulk kernels for fixed-width window maxima/minima and the
//! TA-Lib latest-wins index variants. Roughly three comparisons per element,
//! independent of the window length, and bit-exact safe: the emitted values
//! are always elements of the input, never re-derived arithmetic.
//!
//! Layout convention: `out[k]` describes the window `input[k..k + period]`,
//! so `out` must have length `input.len() - period + 1`. Callers keep their
//! own warm-up fills (NaN or `0.0`) and alignment.
//!
//! Cache blocking: the naive formulation allocates a full-length suffix
//! scratch buffer (plus a full-length index buffer for the indexed variants),
//! which at a million bars streams 8–16 MB through the cache and turns the
//! kernel memory-bound. Instead the input is processed in tiles whose scratch
//! working set fits in L2 ([`SCRATCH_BYTE_BUDGET`]). A tile is always a whole
//! number of `period`-aligned vHGW blocks, so every block boundary lands
//! exactly where the untiled kernel would put it and the outputs are
//! bit-identical. One scratch allocation is reused across all tiles.

/// Scratch working-set target per kernel call. Sized to stay resident in a
/// typical 256 KB–1 MB L2 alongside the streamed input and output.
const SCRATCH_BYTE_BUDGET: usize = 128 * 1024;

/// Window starts handled per tile: the largest whole number of `period`-sized
/// vHGW blocks whose scratch span (`tile + period - 1` elements) fits the
/// byte budget. Degenerates to a single block when `period` alone exceeds it.
#[inline]
fn tile_len(period: usize, bytes_per_element: usize) -> usize {
    debug_assert!(period >= 1);
    let budget = SCRATCH_BYTE_BUDGET / bytes_per_element;
    if budget + 1 <= period {
        return period;
    }
    let blocks = (budget + 1 - period) / period;
    if blocks == 0 {
        period
    } else {
        blocks * period
    }
}

/// Writes the maximum of every full window into `out`.
pub(crate) fn sliding_max_into(input: &[f64], period: usize, out: &mut [f64]) {
    sliding_extremum_tiled(
        input,
        period,
        out,
        tile_len(period, 8),
        |candidate, best| candidate > best,
    );
}

/// Writes the minimum of every full window into `out`.
pub(crate) fn sliding_min_into(input: &[f64], period: usize, out: &mut [f64]) {
    sliding_extremum_tiled(
        input,
        period,
        out,
        tile_len(period, 8),
        |candidate, best| candidate < best,
    );
}

/// Writes the latest-wins index of the window maximum into `out`.
///
/// Tie semantics: when several window elements share the maximum value the
/// LARGEST index wins, matching TA-Lib's all-`>=` Aroon tracker and the
/// newest-wins monotonic deque used by the streaming states.
pub(crate) fn sliding_argmax_latest_into(input: &[f64], period: usize, out: &mut [usize]) {
    sliding_arg_extremum_tiled(input, period, out, tile_len(period, 16), true);
}

/// Writes the latest-wins index of the window minimum into `out`.
pub(crate) fn sliding_argmin_latest_into(input: &[f64], period: usize, out: &mut [usize]) {
    sliding_arg_extremum_tiled(input, period, out, tile_len(period, 16), false);
}

/// Value-only vHGW driven over `period`-aligned tiles of `tile` window starts.
fn sliding_extremum_tiled(
    input: &[f64],
    period: usize,
    out: &mut [f64],
    tile: usize,
    replaces: impl Fn(f64, f64) -> bool + Copy,
) {
    debug_assert!(period >= 1);
    debug_assert!(input.len() >= period);
    debug_assert_eq!(out.len(), input.len() - period + 1);
    debug_assert!(tile >= period && tile % period == 0);
    if period == 1 {
        out.copy_from_slice(input);
        return;
    }
    let len = input.len();
    let span = (tile + period - 1).min(len);
    let mut suffix = vec![0.0_f64; span];

    let mut start = 0usize;
    while start + period <= len {
        let sub_len = (tile + period - 1).min(len - start);
        let sub = &input[start..start + sub_len];
        let scratch = &mut suffix[..sub_len];
        fill_block_suffix(sub, period, scratch, replaces);

        let out_len = sub_len - period + 1;
        let tile_out = &mut out[start..start + out_len];
        let mut prefix = sub[0];
        let mut position_in_block = 0usize;
        for (i, &value) in sub.iter().enumerate() {
            if position_in_block == 0 {
                prefix = value;
            } else if replaces(value, prefix) {
                prefix = value;
            }
            position_in_block += 1;
            if position_in_block == period {
                position_in_block = 0;
            }
            if i + 1 >= period {
                let k = i + 1 - period;
                let tail = scratch[k];
                tile_out[k] = if replaces(prefix, tail) { prefix } else { tail };
            }
        }
        start += tile;
    }
}

/// Latest-wins index vHGW driven over `period`-aligned tiles.
///
/// `maximum` selects `>`/`>=` (max) versus `<`/`<=` (min); the strict form
/// drives the right-to-left suffix scan (keeping the rightmost index on ties)
/// and the non-strict form resolves prefix-versus-suffix, where every prefix
/// index is greater than every suffix index for the same window.
fn sliding_arg_extremum_tiled(
    input: &[f64],
    period: usize,
    out: &mut [usize],
    tile: usize,
    maximum: bool,
) {
    debug_assert!(period >= 1);
    debug_assert!(input.len() >= period);
    debug_assert_eq!(out.len(), input.len() - period + 1);
    debug_assert!(tile >= period && tile % period == 0);
    if period == 1 {
        for (k, slot) in out.iter_mut().enumerate() {
            *slot = k;
        }
        return;
    }
    let strict = |candidate: f64, best: f64| {
        if maximum {
            candidate > best
        } else {
            candidate < best
        }
    };
    let wins = |candidate: f64, best: f64| {
        if maximum {
            candidate >= best
        } else {
            candidate <= best
        }
    };

    let len = input.len();
    let span = (tile + period - 1).min(len);
    let mut suffix_value = vec![0.0_f64; span];
    let mut suffix_index = vec![0usize; span];

    let mut start = 0usize;
    while start + period <= len {
        let sub_len = (tile + period - 1).min(len - start);
        let sub = &input[start..start + sub_len];
        let scratch_value = &mut suffix_value[..sub_len];
        let scratch_index = &mut suffix_index[..sub_len];
        fill_block_suffix_indexed(sub, period, scratch_value, scratch_index, strict);

        let out_len = sub_len - period + 1;
        let tile_out = &mut out[start..start + out_len];
        let mut prefix_value = sub[0];
        let mut prefix_index = 0usize;
        let mut position_in_block = 0usize;
        for (i, &value) in sub.iter().enumerate() {
            if position_in_block == 0 || wins(value, prefix_value) {
                // Latest index wins on equal values.
                prefix_value = value;
                prefix_index = i;
            }
            position_in_block += 1;
            if position_in_block == period {
                position_in_block = 0;
            }
            if i + 1 >= period {
                let k = i + 1 - period;
                let local = if wins(prefix_value, scratch_value[k]) {
                    prefix_index
                } else {
                    scratch_index[k]
                };
                tile_out[k] = start + local;
            }
        }
        start += tile;
    }
}

/// Right-to-left running extremum inside each `period`-aligned block.
#[inline]
fn fill_block_suffix(
    input: &[f64],
    period: usize,
    suffix: &mut [f64],
    replaces: impl Fn(f64, f64) -> bool,
) {
    let len = input.len();
    let mut block_start = 0usize;
    while block_start < len {
        let block_end = (block_start + period).min(len);
        let mut running = input[block_end - 1];
        suffix[block_end - 1] = running;
        for j in (block_start..block_end - 1).rev() {
            let value = input[j];
            if replaces(value, running) {
                running = value;
            }
            suffix[j] = running;
        }
        block_start = block_end;
    }
}

/// Right-to-left running extremum with carried index inside each block.
///
/// Uses a strict comparison so equal values keep the RIGHTMOST (latest)
/// index seen so far in the scan direction.
#[inline]
fn fill_block_suffix_indexed(
    input: &[f64],
    period: usize,
    suffix_value: &mut [f64],
    suffix_index: &mut [usize],
    replaces: impl Fn(f64, f64) -> bool,
) {
    let len = input.len();
    let mut block_start = 0usize;
    while block_start < len {
        let block_end = (block_start + period).min(len);
        let mut running_value = input[block_end - 1];
        let mut running_index = block_end - 1;
        suffix_value[block_end - 1] = running_value;
        suffix_index[block_end - 1] = running_index;
        for j in (block_start..block_end - 1).rev() {
            let value = input[j];
            if replaces(value, running_value) {
                running_value = value;
                running_index = j;
            }
            suffix_value[j] = running_value;
            suffix_index[j] = running_index;
        }
        block_start = block_end;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn windows() -> Vec<(Vec<f64>, usize)> {
        let mut cases = Vec::new();
        for &period in &[1usize, 2, 3, 5, 30, 200] {
            for len in [period, period + 1, period + 7, 512] {
                let mut state = 0x2545F4914F6CDD1D_u64;
                let random: Vec<f64> = (0..len)
                    .map(|_| {
                        state = state
                            .wrapping_mul(6364136223846793005)
                            .wrapping_add(1442695040888963407);
                        ((state >> 33) % 1000) as f64 / 10.0
                    })
                    .collect();
                let increasing: Vec<f64> = (0..len).map(|i| i as f64).collect();
                let decreasing: Vec<f64> = (0..len).map(|i| (len - i) as f64).collect();
                let constant = vec![42.5_f64; len];
                let quantized: Vec<f64> = (0..len).map(|i| ((i * 7) % 5) as f64).collect();
                for data in [random, increasing, decreasing, constant, quantized] {
                    cases.push((data, period));
                }
            }
        }
        cases
    }

    #[test]
    fn matches_naive_window_scans() {
        for (data, period) in windows() {
            let out_len = data.len() - period + 1;
            let mut max_out = vec![0.0; out_len];
            let mut min_out = vec![0.0; out_len];
            let mut argmax_out = vec![0usize; out_len];
            let mut argmin_out = vec![0usize; out_len];
            sliding_max_into(&data, period, &mut max_out);
            sliding_min_into(&data, period, &mut min_out);
            sliding_argmax_latest_into(&data, period, &mut argmax_out);
            sliding_argmin_latest_into(&data, period, &mut argmin_out);
            for k in 0..out_len {
                let window = &data[k..k + period];
                let mut best_max = window[0];
                let mut best_max_idx = 0usize;
                let mut best_min = window[0];
                let mut best_min_idx = 0usize;
                for (j, &value) in window.iter().enumerate() {
                    if value >= best_max {
                        best_max = value;
                        best_max_idx = j;
                    }
                    if value <= best_min {
                        best_min = value;
                        best_min_idx = j;
                    }
                }
                assert_eq!(
                    max_out[k].to_bits(),
                    best_max.to_bits(),
                    "max k={k} p={period}"
                );
                assert_eq!(
                    min_out[k].to_bits(),
                    best_min.to_bits(),
                    "min k={k} p={period}"
                );
                assert_eq!(argmax_out[k], k + best_max_idx, "argmax k={k} p={period}");
                assert_eq!(argmin_out[k], k + best_min_idx, "argmin k={k} p={period}");
            }
        }
    }

    /// Cheap LCG series with heavy value repetition so tie-breaking is
    /// exercised at every tile boundary.
    fn lcg_series(len: usize, modulus: u64) -> Vec<f64> {
        let mut state = 0x9E3779B97F4A7C15_u64;
        (0..len)
            .map(|_| {
                state = state
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                ((state >> 33) % modulus) as f64 / 8.0
            })
            .collect()
    }

    /// Untiled driver: one tile covering the whole input.
    fn whole_input_tile(len: usize, period: usize) -> usize {
        len.div_ceil(period) * period
    }

    #[test]
    fn tiled_matches_untiled_bitwise() {
        const LEN: usize = 300_000;
        for &modulus in &[7_u64, 100_000] {
            let data = lcg_series(LEN, modulus);
            for &period in &[2usize, 14, 200, 4096] {
                let out_len = LEN - period + 1;
                let untiled = whole_input_tile(LEN, period);
                assert!(tile_len(period, 8) < untiled, "tiling must actually split");

                let mut tiled_max = vec![0.0; out_len];
                let mut reference_max = vec![0.0; out_len];
                sliding_max_into(&data, period, &mut tiled_max);
                sliding_extremum_tiled(&data, period, &mut reference_max, untiled, |c, b| c > b);

                let mut tiled_min = vec![0.0; out_len];
                let mut reference_min = vec![0.0; out_len];
                sliding_min_into(&data, period, &mut tiled_min);
                sliding_extremum_tiled(&data, period, &mut reference_min, untiled, |c, b| c < b);

                let mut tiled_argmax = vec![0usize; out_len];
                let mut reference_argmax = vec![0usize; out_len];
                sliding_argmax_latest_into(&data, period, &mut tiled_argmax);
                sliding_arg_extremum_tiled(&data, period, &mut reference_argmax, untiled, true);

                let mut tiled_argmin = vec![0usize; out_len];
                let mut reference_argmin = vec![0usize; out_len];
                sliding_argmin_latest_into(&data, period, &mut tiled_argmin);
                sliding_arg_extremum_tiled(&data, period, &mut reference_argmin, untiled, false);

                for k in 0..out_len {
                    assert_eq!(
                        tiled_max[k].to_bits(),
                        reference_max[k].to_bits(),
                        "max k={k} p={period} m={modulus}"
                    );
                    assert_eq!(
                        tiled_min[k].to_bits(),
                        reference_min[k].to_bits(),
                        "min k={k} p={period} m={modulus}"
                    );
                    assert_eq!(
                        tiled_argmax[k], reference_argmax[k],
                        "argmax k={k} p={period} m={modulus}"
                    );
                    assert_eq!(
                        tiled_argmin[k], reference_argmin[k],
                        "argmin k={k} p={period} m={modulus}"
                    );
                }
            }
        }
    }

    #[test]
    fn tile_len_respects_budget() {
        for &period in &[1usize, 2, 14, 200, 4096, 20_000] {
            for &bytes in &[8usize, 16] {
                let tile = tile_len(period, bytes);
                assert_eq!(tile % period, 0, "p={period}");
                assert!(tile >= period, "p={period}");
                let scratch = (tile + period - 1) * bytes;
                assert!(
                    scratch <= SCRATCH_BYTE_BUDGET || tile == period,
                    "p={period} b={bytes} scratch={scratch}"
                );
            }
        }
    }
}
