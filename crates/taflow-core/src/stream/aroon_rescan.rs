//! Whole-slice latest-extrema rescan shared by the Aroon states.

#[inline]
pub(crate) fn aroon_rescan<F>(
    high: &[f64],
    low: &[f64],
    period: usize,
    inverse_period: f64,
    mut emit: F,
) where
    F: FnMut(usize, f64, f64),
{
    debug_assert_eq!(high.len(), low.len());
    debug_assert!(high.len() > period);
    let mut highest = high[0];
    let mut highest_index = 0_usize;
    let mut lowest = low[0];
    let mut lowest_index = 0_usize;
    for index in 1..=period {
        if high[index] >= highest {
            highest = high[index];
            highest_index = index;
        }
        if low[index] <= lowest {
            lowest = low[index];
            lowest_index = index;
        }
    }
    emit(
        period,
        lowest_index as f64 * inverse_period,
        highest_index as f64 * inverse_period,
    );

    let mut trailing = 1_usize;
    for today in period + 1..high.len() {
        if highest_index < trailing {
            highest = high[trailing];
            highest_index = trailing;
            for (offset, &value) in high[trailing + 1..=today].iter().enumerate() {
                if value >= highest {
                    highest = value;
                    highest_index = trailing + 1 + offset;
                }
            }
        } else if high[today] >= highest {
            highest = high[today];
            highest_index = today;
        }
        if lowest_index < trailing {
            lowest = low[trailing];
            lowest_index = trailing;
            for (offset, &value) in low[trailing + 1..=today].iter().enumerate() {
                if value <= lowest {
                    lowest = value;
                    lowest_index = trailing + 1 + offset;
                }
            }
        } else if low[today] <= lowest {
            lowest = low[today];
            lowest_index = today;
        }
        emit(
            today,
            (period - (today - lowest_index)) as f64 * inverse_period,
            (period - (today - highest_index)) as f64 * inverse_period,
        );
        trailing += 1;
    }
}
