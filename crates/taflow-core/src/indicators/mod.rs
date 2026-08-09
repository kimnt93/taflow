//! Canonical persistent technical indicators.

mod arnaud_legoux_moving_average;
#[cfg(test)]
mod arnaud_legoux_moving_average_test;
mod awesome_oscillator;
#[cfg(test)]
mod awesome_oscillator_test;
mod cumulative_count;
#[cfg(test)]
mod cumulative_count_test;
mod cumulative_product;
#[cfg(test)]
mod cumulative_product_test;
mod cumulative_sum;
#[cfg(test)]
mod cumulative_sum_test;
mod drawdown;
#[cfg(test)]
mod drawdown_test;
mod even_better_sinewave;
#[cfg(test)]
mod even_better_sinewave_test;
mod exponentially_weighted_correlation;
#[cfg(test)]
mod exponentially_weighted_correlation_test;
mod exponentially_weighted_covariance;
#[cfg(test)]
mod exponentially_weighted_covariance_test;
mod exponentially_weighted_sum;
#[cfg(test)]
mod exponentially_weighted_sum_test;
mod fisher_transform;
#[cfg(test)]
mod fisher_transform_test;
mod hull_moving_average;
#[cfg(test)]
mod hull_moving_average_test;
mod klinger_volume_oscillator;
#[cfg(test)]
mod klinger_volume_oscillator_test;
mod lag;
#[cfg(test)]
mod lag_test;
mod log_return;
#[cfg(test)]
mod log_return_test;
mod momentum;
#[cfg(test)]
mod momentum_test;
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
mod rate_of_change;
mod rate_of_change_percent;
#[cfg(test)]
mod rate_of_change_percent_test;
mod rate_of_change_ratio;
mod rate_of_change_ratio_percent;
#[cfg(test)]
mod rate_of_change_ratio_percent_test;
#[cfg(test)]
mod rate_of_change_ratio_test;
#[cfg(test)]
mod rate_of_change_test;
mod rolling_calmar;
#[cfg(test)]
mod rolling_calmar_test;
mod rolling_covariance;
#[cfg(test)]
mod rolling_covariance_test;
mod rolling_interquartile_range;
#[cfg(test)]
mod rolling_interquartile_range_test;
mod rolling_mode;
#[cfg(test)]
mod rolling_mode_test;
mod rolling_percentile;
#[cfg(test)]
mod rolling_percentile_test;
mod rolling_quantile;
#[cfg(test)]
mod rolling_quantile_test;
mod rolling_rank;
#[cfg(test)]
mod rolling_rank_test;
mod rolling_sharpe;
#[cfg(test)]
mod rolling_sharpe_test;
mod rolling_sortino;
#[cfg(test)]
mod rolling_sortino_test;
mod rolling_winsorize;
#[cfg(test)]
mod rolling_winsorize_test;
mod session_volume_levels;
#[cfg(test)]
mod session_volume_levels_test;
mod smoothed_trend_channel;
#[cfg(test)]
mod smoothed_trend_channel_test;
mod tom_de_mark_sequential;
#[cfg(test)]
mod tom_de_mark_sequential_test;
mod true_strength_index;
#[cfg(test)]
mod true_strength_index_test;
mod volume_weighted_moving_average;
#[cfg(test)]
mod volume_weighted_moving_average_test;
mod williams_percent_r;
#[cfg(test)]
mod williams_percent_r_test;
mod zero_lag_exponential_moving_average;
#[cfg(test)]
mod zero_lag_exponential_moving_average_test;

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

pub use arnaud_legoux_moving_average::ArnaudLegouxMovingAverage;
pub use awesome_oscillator::AwesomeOscillator;
pub use cumulative_count::CumulativeCount;
pub use cumulative_product::CumulativeProduct;
pub use cumulative_sum::CumulativeSum;
pub use drawdown::Drawdown;
pub use even_better_sinewave::EvenBetterSinewave;
pub use exponentially_weighted_correlation::ExponentiallyWeightedCorrelation;
pub use exponentially_weighted_covariance::ExponentiallyWeightedCovariance;
pub use exponentially_weighted_sum::ExponentiallyWeightedSum;
pub use fisher_transform::FisherTransform;
pub use hull_moving_average::HullMovingAverage;
pub use klinger_volume_oscillator::{KlingerVolumeOscillator, KlingerVolumeOscillatorValue};
pub use lag::Lag;
pub use log_return::LogReturn;
pub use momentum::Momentum;
pub use opening_range::{OpeningRange, OpeningRangeValue};
pub use parabolic_moving_average_stop::{
    ParabolicMovingAverageStop, ParabolicMovingAverageStopValue,
};
pub use pivot_points::{PivotPoints, PivotPointsValue};
pub use premium_discount::{PremiumDiscount, PremiumDiscountValue};
pub use rate_of_change::RateOfChange;
pub use rate_of_change_percent::RateOfChangePercent;
pub use rate_of_change_ratio::RateOfChangeRatio;
pub use rate_of_change_ratio_percent::RateOfChangeRatioPercent;
pub use rolling_calmar::RollingCalmar;
pub use rolling_covariance::RollingCovariance;
pub use rolling_interquartile_range::RollingInterquartileRange;
pub use rolling_mode::RollingMode;
pub use rolling_percentile::RollingPercentile;
pub use rolling_quantile::RollingQuantile;
pub use rolling_rank::RollingRank;
pub use rolling_sharpe::RollingSharpe;
pub use rolling_sortino::RollingSortino;
pub use rolling_winsorize::RollingWinsorize;
pub use session_volume_levels::{SessionVolumeLevels, SessionVolumeLevelsValue};
pub use smoothed_trend_channel::SmoothedTrendChannel;
pub use tom_de_mark_sequential::{TomDeMarkSequential, TomDeMarkSequentialValue};
pub use true_strength_index::TrueStrengthIndex;
pub use volume_weighted_moving_average::VolumeWeightedMovingAverage;
pub use williams_percent_r::WilliamsPercentR;
pub use zero_lag_exponential_moving_average::ZeroLagExponentialMovingAverage;
