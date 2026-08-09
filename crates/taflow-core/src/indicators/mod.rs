//! Canonical persistent technical indicators.

mod cumulative_count;
#[cfg(test)]
mod cumulative_count_test;
mod even_better_sinewave;
#[cfg(test)]
mod even_better_sinewave_test;
mod klinger_volume_oscillator;
#[cfg(test)]
mod klinger_volume_oscillator_test;
mod opening_range;
#[cfg(test)]
mod opening_range_test;
mod parabolic_moving_average_stop;
#[cfg(test)]
mod parabolic_moving_average_stop_test;
mod pivot_points;
#[cfg(test)]
mod pivot_points_test;
mod premium_discount;
#[cfg(test)]
mod premium_discount_test;
mod session_volume_levels;
#[cfg(test)]
mod session_volume_levels_test;
mod smoothed_trend_channel;
#[cfg(test)]
mod smoothed_trend_channel_test;
mod tom_de_mark_sequential;
#[cfg(test)]
mod tom_de_mark_sequential_test;

use crate::error::TaError;

pub(crate) fn invalid_period(name: &'static str, period: usize, minimum: usize) -> TaError {
    TaError::InvalidParameter {
        name,
        value: period.to_string(),
        reason: if minimum == 1 {
            "must be >= 1"
        } else {
            "must be >= 2"
        },
    }
}

pub use cumulative_count::CumulativeCount;
pub use even_better_sinewave::EvenBetterSinewave;
pub use klinger_volume_oscillator::{KlingerVolumeOscillator, KlingerVolumeOscillatorValue};
pub use opening_range::{OpeningRange, OpeningRangeValue};
pub use parabolic_moving_average_stop::{
    ParabolicMovingAverageStop, ParabolicMovingAverageStopValue,
};
pub use pivot_points::{PivotPoints, PivotPointsValue};
pub use premium_discount::{PremiumDiscount, PremiumDiscountValue};
pub use session_volume_levels::{SessionVolumeLevels, SessionVolumeLevelsValue};
pub use smoothed_trend_channel::SmoothedTrendChannel;
pub use tom_de_mark_sequential::{TomDeMarkSequential, TomDeMarkSequentialValue};
