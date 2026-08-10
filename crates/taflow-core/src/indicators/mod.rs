//! Canonical persistent technical indicators.

pub use crate::stream::StreamingIndicator;

mod exponentially_weighted_standard_deviation;
#[cfg(test)]
mod exponentially_weighted_standard_deviation_test;
mod exponentially_weighted_variance;
#[cfg(test)]
mod exponentially_weighted_variance_test;
mod fast_stochastic_oscillator;
#[cfg(test)]
mod fast_stochastic_oscillator_test;
mod fibonacci_retracement;
#[cfg(test)]
mod fibonacci_retracement_test;
mod frac_diff;
#[cfg(test)]
mod frac_diff_test;
mod minus_directional_indicator;
#[cfg(test)]
mod minus_directional_indicator_test;
mod minus_directional_movement;
#[cfg(test)]
mod minus_directional_movement_test;
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

pub use exponentially_weighted_standard_deviation::ExponentiallyWeightedStandardDeviation;
pub use exponentially_weighted_variance::ExponentiallyWeightedVariance;
pub use fast_stochastic_oscillator::{FastStochasticOscillator, FastStochasticOscillatorValue};
pub use fibonacci_retracement::{FibonacciRetracement, FibonacciRetracementValue};
pub use frac_diff::FracDiff;
pub use minus_directional_indicator::MinusDirectionalIndicator;
pub use minus_directional_movement::MinusDirectionalMovement;
pub use moving_average_convergence_divergence::{
    MovingAverageConvergenceDivergence, MovingAverageConvergenceDivergenceValue,
};
pub use moving_average_convergence_divergence_extended::MovingAverageConvergenceDivergenceExtended;
pub use moving_average_convergence_divergence_fixed::MovingAverageConvergenceDivergenceFixed;

mod decay_linear;
#[cfg(test)]
mod decay_linear_test;
mod double_exponential_moving_average;
#[cfg(test)]
mod double_exponential_moving_average_test;
mod exponential_moving_average;
#[cfg(test)]
mod exponential_moving_average_test;
mod hilbert_transform_trendline;
#[cfg(test)]
mod hilbert_transform_trendline_test;
mod ichimoku;
#[cfg(test)]
mod ichimoku_test;
mod money_flow_index;
#[cfg(test)]
mod money_flow_index_test;
mod moving_average;
#[cfg(test)]
mod moving_average_test;
mod parkinson;
#[cfg(test)]
mod parkinson_test;
mod percentage_price_oscillator;
#[cfg(test)]
mod percentage_price_oscillator_test;
mod relative_momentum_index;
#[cfg(test)]
mod relative_momentum_index_test;
mod relative_strength_index;
#[cfg(test)]
mod relative_strength_index_test;

pub use decay_linear::DecayLinear;
pub use double_exponential_moving_average::DoubleExponentialMovingAverage;
pub use exponential_moving_average::ExponentialMovingAverage;
pub use hilbert_transform_trendline::HilbertTransformTrendline;
pub use ichimoku::{Ichimoku, IchimokuValue};
pub use money_flow_index::MoneyFlowIndex;
pub use moving_average::MovingAverage;
pub use parkinson::Parkinson;
pub use percentage_price_oscillator::PercentagePriceOscillator;
pub use relative_momentum_index::RelativeMomentumIndex;
pub use relative_strength_index::RelativeStrengthIndex;

mod bars_since;
#[cfg(test)]
mod bars_since_test;
mod entry_exit;
#[cfg(test)]
mod entry_exit_test;
mod force_index;
#[cfg(test)]
mod force_index_test;
mod highest_since;
#[cfg(test)]
mod highest_since_test;
mod lowest_since;
#[cfg(test)]
mod lowest_since_test;
mod position_hold;
#[cfg(test)]
mod position_hold_test;
mod previous_high_low;
#[cfg(test)]
mod previous_high_low_test;
mod retracements;
#[cfg(test)]
mod retracements_test;
mod rogers_satchell;
#[cfg(test)]
mod rogers_satchell_test;

pub use bars_since::BarsSince;
pub use entry_exit::EntryExit;
pub use force_index::ForceIndex;
pub use highest_since::HighestSince;
pub use lowest_since::LowestSince;
pub use position_hold::PositionHold;
pub use previous_high_low::{PreviousHighLow, PreviousHighLowValue};
pub use retracements::{Retracements, RetracementsValue};
pub use rogers_satchell::RogersSatchell;

mod ease_of_movement;
#[cfg(test)]
mod ease_of_movement_test;
mod ornstein_uhlenbeck_half_life;
#[cfg(test)]
mod ornstein_uhlenbeck_half_life_test;
mod plus_directional_indicator;
#[cfg(test)]
mod plus_directional_indicator_test;
mod plus_directional_movement;
#[cfg(test)]
mod plus_directional_movement_test;
mod roll_spread;
#[cfg(test)]
mod roll_spread_test;
mod rolling_autocorr;
#[cfg(test)]
mod rolling_autocorr_test;
mod rolling_volume_weighted_average_price;
#[cfg(test)]
mod rolling_volume_weighted_average_price_test;
mod rolling_z_score;
#[cfg(test)]
mod rolling_z_score_test;
mod schaff_trend_cycle;
#[cfg(test)]
mod schaff_trend_cycle_test;
mod session_extrema;
#[cfg(test)]
mod session_extrema_test;

pub use ease_of_movement::EaseOfMovement;
pub use ornstein_uhlenbeck_half_life::OrnsteinUhlenbeckHalfLife;
pub use plus_directional_indicator::PlusDirectionalIndicator;
pub use plus_directional_movement::PlusDirectionalMovement;
pub use roll_spread::RollSpread;
pub use rolling_autocorr::RollingAutocorr;
pub use rolling_volume_weighted_average_price::RollingVolumeWeightedAveragePrice;
pub use rolling_z_score::RollingZScore;
pub use schaff_trend_cycle::{SchaffTrendCycle, SchaffTrendCycleValue};
pub use session_extrema::{SessionExtrema, SessionExtremaValue};

mod sessions;
#[cfg(test)]
mod sessions_test;
mod signal_delay;
#[cfg(test)]
mod signal_delay_test;
mod signed_power;
#[cfg(test)]
mod signed_power_test;
mod simple_moving_average;
#[cfg(test)]
mod simple_moving_average_test;
mod squeeze;
mod squeeze_pro;
#[cfg(test)]
mod squeeze_pro_test;
#[cfg(test)]
mod squeeze_test;
mod stochastic_oscillator;
#[cfg(test)]
mod stochastic_oscillator_test;
mod stochastic_relative_strength_index;
#[cfg(test)]
mod stochastic_relative_strength_index_test;
mod supertrend;
#[cfg(test)]
mod supertrend_test;
mod triangular_moving_average;
#[cfg(test)]
mod triangular_moving_average_test;

pub use sessions::{Sessions, SessionsValue};
pub use signal_delay::SignalDelay;
pub use signed_power::SignedPower;
pub use simple_moving_average::SimpleMovingAverage;
pub use squeeze::{Squeeze, SqueezeValue};
pub use squeeze_pro::{SqueezePro, SqueezeProValue};
pub use stochastic_oscillator::{StochasticOscillator, StochasticOscillatorValue};
pub use stochastic_relative_strength_index::{
    StochasticRelativeStrengthIndex, StochasticRelativeStrengthIndexValue,
};
pub use supertrend::{Supertrend, SupertrendValue};
pub use triangular_moving_average::TriangularMovingAverage;

mod acceleration_bands;
#[cfg(test)]
mod acceleration_bands_test;
mod accumulation_distribution;
mod accumulation_distribution_oscillator;
#[cfg(test)]
mod accumulation_distribution_oscillator_test;
#[cfg(test)]
mod accumulation_distribution_test;
mod arnaud_legoux_moving_average;
#[cfg(test)]
mod arnaud_legoux_moving_average_test;
mod aroon;
mod aroon_oscillator;
#[cfg(test)]
mod aroon_oscillator_test;
#[cfg(test)]
mod aroon_test;
mod average_directional_index;
mod average_directional_index_rating;
#[cfg(test)]
mod average_directional_index_rating_test;
#[cfg(test)]
mod average_directional_index_test;
mod average_price;
#[cfg(test)]
mod average_price_test;
mod average_true_range;
#[cfg(test)]
mod average_true_range_test;
mod awesome_oscillator;
#[cfg(test)]
mod awesome_oscillator_test;
mod balance_of_power;
#[cfg(test)]
mod balance_of_power_test;
mod cumulative_count;
#[cfg(test)]
mod cumulative_count_test;
mod cumulative_product;
#[cfg(test)]
mod cumulative_product_test;
mod cumulative_sum;
#[cfg(test)]
mod cumulative_sum_test;
mod demand_index;
#[cfg(test)]
mod demand_index_test;
mod directional_movement_index;
#[cfg(test)]
mod directional_movement_index_test;
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
mod math_abs;
#[cfg(test)]
mod math_abs_test;
mod math_acos;
#[cfg(test)]
mod math_acos_test;
mod math_acosh;
#[cfg(test)]
mod math_acosh_test;
mod math_add;
#[cfg(test)]
mod math_add_test;
mod math_asin;
#[cfg(test)]
mod math_asin_test;
mod math_asinh;
#[cfg(test)]
mod math_asinh_test;
mod math_atan;
#[cfg(test)]
mod math_atan_test;
mod math_atanh;
#[cfg(test)]
mod math_atanh_test;
mod math_cbrt;
#[cfg(test)]
mod math_cbrt_test;
mod math_ceil;
#[cfg(test)]
mod math_ceil_test;
mod math_cos;
#[cfg(test)]
mod math_cos_test;
mod math_cosh;
#[cfg(test)]
mod math_cosh_test;
mod math_cot;
#[cfg(test)]
mod math_cot_test;
mod math_degrees;
#[cfg(test)]
mod math_degrees_test;
mod math_exp;
#[cfg(test)]
mod math_exp_test;
mod math_floor;
#[cfg(test)]
mod math_floor_test;
mod math_ln;
#[cfg(test)]
mod math_ln_test;
mod math_log10;
#[cfg(test)]
mod math_log10_test;
mod math_log1p;
#[cfg(test)]
mod math_log1p_test;
mod math_radians;
#[cfg(test)]
mod math_radians_test;
mod math_sin;
#[cfg(test)]
mod math_sin_test;
mod math_sinh;
#[cfg(test)]
mod math_sinh_test;
mod math_sqrt;
#[cfg(test)]
mod math_sqrt_test;
mod math_subtract;
#[cfg(test)]
mod math_subtract_test;
mod math_tan;
#[cfg(test)]
mod math_tan_test;
mod math_tanh;
#[cfg(test)]
mod math_tanh_test;
mod median_price;
#[cfg(test)]
mod median_price_test;
mod mesa_adaptive_moving_average;
#[cfg(test)]
mod mesa_adaptive_moving_average_test;
mod momentum;
#[cfg(test)]
mod momentum_test;
mod normalized_average_true_range;
#[cfg(test)]
mod normalized_average_true_range_test;
mod on_balance_volume;
#[cfg(test)]
mod on_balance_volume_test;
mod opening_range;
#[cfg(test)]
mod opening_range_test;
mod parabolic_moving_average_stop;
#[cfg(test)]
mod parabolic_moving_average_stop_test;
mod parabolic_sar;
mod parabolic_sar_extended;
#[cfg(test)]
mod parabolic_sar_extended_test;
#[cfg(test)]
mod parabolic_sar_test;
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
mod rolling_average_deviation;
#[cfg(test)]
mod rolling_average_deviation_test;
mod rolling_beta;
#[cfg(test)]
mod rolling_beta_test;
mod rolling_calmar;
#[cfg(test)]
mod rolling_calmar_test;
mod rolling_conditional_value_at_risk;
#[cfg(test)]
mod rolling_conditional_value_at_risk_test;
mod rolling_correlation;
#[cfg(test)]
mod rolling_correlation_test;
mod rolling_covariance;
#[cfg(test)]
mod rolling_covariance_test;
mod rolling_entropy;
#[cfg(test)]
mod rolling_entropy_test;
mod rolling_interquartile_range;
#[cfg(test)]
mod rolling_interquartile_range_test;
mod rolling_kelly_criterion;
#[cfg(test)]
mod rolling_kelly_criterion_test;
mod rolling_kurtosis;
#[cfg(test)]
mod rolling_kurtosis_test;
mod rolling_linear_regression;
mod rolling_linear_regression_angle;
#[cfg(test)]
mod rolling_linear_regression_angle_test;
mod rolling_linear_regression_intercept;
#[cfg(test)]
mod rolling_linear_regression_intercept_test;
mod rolling_linear_regression_slope;
#[cfg(test)]
mod rolling_linear_regression_slope_test;
#[cfg(test)]
mod rolling_linear_regression_test;
mod rolling_maximum;
mod rolling_maximum_drawdown;
#[cfg(test)]
mod rolling_maximum_drawdown_test;
mod rolling_maximum_index;
#[cfg(test)]
mod rolling_maximum_index_test;
#[cfg(test)]
mod rolling_maximum_test;
mod rolling_median;
#[cfg(test)]
mod rolling_median_test;
mod rolling_midpoint;
#[cfg(test)]
mod rolling_midpoint_test;
mod rolling_midprice;
#[cfg(test)]
mod rolling_midprice_test;
mod rolling_min_max;
mod rolling_min_max_index;
#[cfg(test)]
mod rolling_min_max_index_test;
#[cfg(test)]
mod rolling_min_max_test;
mod rolling_minimum;
mod rolling_minimum_index;
#[cfg(test)]
mod rolling_minimum_index_test;
#[cfg(test)]
mod rolling_minimum_test;
mod rolling_mode;
#[cfg(test)]
mod rolling_mode_test;
mod rolling_omega_ratio;
#[cfg(test)]
mod rolling_omega_ratio_test;
mod rolling_percentile;
#[cfg(test)]
mod rolling_percentile_test;
mod rolling_profit_factor;
#[cfg(test)]
mod rolling_profit_factor_test;
mod rolling_quantile;
#[cfg(test)]
mod rolling_quantile_test;
mod rolling_rank;
#[cfg(test)]
mod rolling_rank_test;
mod rolling_recovery_factor;
#[cfg(test)]
mod rolling_recovery_factor_test;
mod rolling_sharpe;
#[cfg(test)]
mod rolling_sharpe_test;
mod rolling_skew;
#[cfg(test)]
mod rolling_skew_test;
mod rolling_sortino;
#[cfg(test)]
mod rolling_sortino_test;
mod rolling_standard_deviation;
#[cfg(test)]
mod rolling_standard_deviation_test;
mod rolling_statistic_helpers;
mod rolling_sum;
#[cfg(test)]
mod rolling_sum_test;
mod rolling_time_series_forecast;
#[cfg(test)]
mod rolling_time_series_forecast_test;
mod rolling_treynor_ratio;
#[cfg(test)]
mod rolling_treynor_ratio_test;
mod rolling_value_at_risk;
#[cfg(test)]
mod rolling_value_at_risk_test;
mod rolling_variance;
#[cfg(test)]
mod rolling_variance_test;
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
mod true_range;
#[cfg(test)]
mod true_range_test;
mod true_strength_index;
#[cfg(test)]
mod true_strength_index_test;
mod typical_price;
#[cfg(test)]
mod typical_price_test;
mod variable_period_moving_average;
#[cfg(test)]
mod variable_period_moving_average_test;
mod volume_oscillator;
#[cfg(test)]
mod volume_oscillator_test;
mod volume_weighted_moving_average;
#[cfg(test)]
mod volume_weighted_moving_average_test;
mod volume_zone_oscillator;
#[cfg(test)]
mod volume_zone_oscillator_test;
mod weighted_close;
#[cfg(test)]
mod weighted_close_test;
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

pub use acceleration_bands::{AccelerationBands, AccelerationBandsValue};
pub use accumulation_distribution::AccumulationDistribution;
pub use accumulation_distribution_oscillator::AccumulationDistributionOscillator;
pub use arnaud_legoux_moving_average::ArnaudLegouxMovingAverage;
pub use aroon::{Aroon, AroonValue};
pub use aroon_oscillator::AroonOscillator;
pub use average_directional_index::AverageDirectionalIndex;
pub use average_directional_index_rating::AverageDirectionalIndexRating;
pub use average_price::AveragePrice;
pub use average_true_range::AverageTrueRange;
pub use awesome_oscillator::AwesomeOscillator;
pub use balance_of_power::BalanceOfPower;
pub use cumulative_count::CumulativeCount;
pub use cumulative_product::CumulativeProduct;
pub use cumulative_sum::CumulativeSum;
pub use demand_index::DemandIndex;
pub use directional_movement_index::DirectionalMovementIndex;
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
pub use math_abs::MathAbs;
pub use math_acos::MathAcos;
pub use math_acosh::MathAcosh;
pub use math_add::MathAdd;
pub use math_asin::MathAsin;
pub use math_asinh::MathAsinh;
pub use math_atan::MathAtan;
pub use math_atanh::MathAtanh;
pub use math_cbrt::MathCbrt;
pub use math_ceil::MathCeil;
pub use math_cos::MathCos;
pub use math_cosh::MathCosh;
pub use math_cot::MathCot;
pub use math_degrees::MathDegrees;
pub use math_exp::MathExp;
pub use math_floor::MathFloor;
pub use math_ln::MathLn;
pub use math_log10::MathLog10;
pub use math_log1p::MathLog1p;
pub use math_radians::MathRadians;
pub use math_sin::MathSin;
pub use math_sinh::MathSinh;
pub use math_sqrt::MathSqrt;
pub use math_subtract::MathSubtract;
pub use math_tan::MathTan;
pub use math_tanh::MathTanh;
pub use median_price::MedianPrice;
pub use mesa_adaptive_moving_average::{MesaAdaptiveMovingAverage, MesaAdaptiveMovingAverageValue};
pub use momentum::Momentum;
pub use normalized_average_true_range::NormalizedAverageTrueRange;
pub use on_balance_volume::OnBalanceVolume;
pub use opening_range::{OpeningRange, OpeningRangeValue};
pub use parabolic_moving_average_stop::{
    ParabolicMovingAverageStop, ParabolicMovingAverageStopValue,
};
pub use parabolic_sar::ParabolicSar;
pub use parabolic_sar_extended::ParabolicSarExtended;
pub use pivot_points::{PivotPoints, PivotPointsValue};
pub use premium_discount::{PremiumDiscount, PremiumDiscountValue};
pub use rate_of_change::RateOfChange;
pub use rate_of_change_percent::RateOfChangePercent;
pub use rate_of_change_ratio::RateOfChangeRatio;
pub use rate_of_change_ratio_percent::RateOfChangeRatioPercent;
pub use rolling_average_deviation::RollingAverageDeviation;
pub use rolling_beta::RollingBeta;
pub use rolling_calmar::RollingCalmar;
pub use rolling_conditional_value_at_risk::RollingConditionalValueAtRisk;
pub use rolling_correlation::RollingCorrelation;
pub use rolling_covariance::RollingCovariance;
pub use rolling_entropy::RollingEntropy;
pub use rolling_interquartile_range::RollingInterquartileRange;
pub use rolling_kelly_criterion::RollingKellyCriterion;
pub use rolling_kurtosis::RollingKurtosis;
pub use rolling_linear_regression::RollingLinearRegression;
pub use rolling_linear_regression_angle::RollingLinearRegressionAngle;
pub use rolling_linear_regression_intercept::RollingLinearRegressionIntercept;
pub use rolling_linear_regression_slope::RollingLinearRegressionSlope;
pub use rolling_maximum::RollingMaximum;
pub use rolling_maximum_drawdown::RollingMaximumDrawdown;
pub use rolling_maximum_index::RollingMaximumIndex;
pub use rolling_median::RollingMedian;
pub use rolling_midpoint::RollingMidpoint;
pub use rolling_midprice::RollingMidprice;
pub use rolling_min_max::{RollingMinMax, RollingMinMaxValue};
pub use rolling_min_max_index::{RollingMinMaxIndex, RollingMinMaxIndexValue};
pub use rolling_minimum::RollingMinimum;
pub use rolling_minimum_index::RollingMinimumIndex;
pub use rolling_mode::RollingMode;
pub use rolling_omega_ratio::RollingOmegaRatio;
pub use rolling_percentile::RollingPercentile;
pub use rolling_profit_factor::RollingProfitFactor;
pub use rolling_quantile::RollingQuantile;
pub use rolling_rank::RollingRank;
pub use rolling_recovery_factor::RollingRecoveryFactor;
pub use rolling_sharpe::RollingSharpe;
pub use rolling_skew::RollingSkew;
pub use rolling_sortino::RollingSortino;
pub use rolling_standard_deviation::RollingStandardDeviation;
pub use rolling_sum::RollingSum;
pub use rolling_time_series_forecast::RollingTimeSeriesForecast;
pub use rolling_treynor_ratio::RollingTreynorRatio;
pub use rolling_value_at_risk::RollingValueAtRisk;
pub use rolling_variance::RollingVariance;
pub use rolling_winsorize::RollingWinsorize;
pub use session_volume_levels::{SessionVolumeLevels, SessionVolumeLevelsValue};
pub use smoothed_trend_channel::SmoothedTrendChannel;
pub use tom_de_mark_sequential::{TomDeMarkSequential, TomDeMarkSequentialValue};
pub use true_range::TrueRange;
pub use true_strength_index::TrueStrengthIndex;
pub use typical_price::TypicalPrice;
pub use variable_period_moving_average::VariablePeriodMovingAverage;
pub use volume_oscillator::VolumeOscillator;
pub use volume_weighted_moving_average::VolumeWeightedMovingAverage;
pub use volume_zone_oscillator::VolumeZoneOscillator;
pub use weighted_close::WeightedClose;
pub use williams_percent_r::WilliamsPercentR;
pub use zero_lag_exponential_moving_average::ZeroLagExponentialMovingAverage;

mod math_divide;
#[cfg(test)]
mod math_divide_test;
mod math_multiply;
#[cfg(test)]
mod math_multiply_test;

pub use math_divide::MathDivide;
pub use math_multiply::MathMultiply;

mod intraday_momentum_index;
#[cfg(test)]
mod intraday_momentum_index_test;
mod jurik_moving_average;
#[cfg(test)]
mod jurik_moving_average_test;
mod kalman_hedge_ratio;
#[cfg(test)]
mod kalman_hedge_ratio_test;
mod kaufman_adaptive_moving_average;
#[cfg(test)]
mod kaufman_adaptive_moving_average_test;
mod know_sure_thing;
#[cfg(test)]
mod know_sure_thing_test;
mod laguerre_relative_strength_index;
#[cfg(test)]
mod laguerre_relative_strength_index_test;
mod liquidity;
#[cfg(test)]
mod liquidity_test;
mod mass_index;
#[cfg(test)]
mod mass_index_test;
mod mc_ginley_dynamic;
#[cfg(test)]
mod mc_ginley_dynamic_test;

pub use intraday_momentum_index::IntradayMomentumIndex;
pub use jurik_moving_average::JurikMovingAverage;
pub use kalman_hedge_ratio::KalmanHedgeRatio;
pub use kaufman_adaptive_moving_average::KaufmanAdaptiveMovingAverage;
pub use know_sure_thing::KnowSureThing;
pub use laguerre_relative_strength_index::LaguerreRelativeStrengthIndex;
pub use liquidity::Liquidity;
pub use mass_index::MassIndex;
pub use mc_ginley_dynamic::McGinleyDynamic;

mod garman_klass;
#[cfg(test)]
mod garman_klass_test;
mod garman_klass_yang_zhang;
#[cfg(test)]
mod garman_klass_yang_zhang_test;
mod hedge_ratio;
#[cfg(test)]
mod hedge_ratio_test;
mod heikin_ashi;
#[cfg(test)]
mod heikin_ashi_test;
mod hilbert_transform_dominant_cycle_period;
#[cfg(test)]
mod hilbert_transform_dominant_cycle_period_test;
mod hilbert_transform_dominant_cycle_phase;
#[cfg(test)]
mod hilbert_transform_dominant_cycle_phase_test;
mod hilbert_transform_phasor;
#[cfg(test)]
mod hilbert_transform_phasor_test;
mod hilbert_transform_sine_wave;
#[cfg(test)]
mod hilbert_transform_sine_wave_test;
mod hilbert_transform_trend_mode;
#[cfg(test)]
mod hilbert_transform_trend_mode_test;
mod hurst;
#[cfg(test)]
mod hurst_test;

pub use garman_klass::GarmanKlass;
pub use garman_klass_yang_zhang::GarmanKlassYangZhang;
pub use hedge_ratio::HedgeRatio;
pub use heikin_ashi::HeikinAshi;
pub use hilbert_transform_dominant_cycle_period::HilbertTransformDominantCyclePeriod;
pub use hilbert_transform_dominant_cycle_phase::HilbertTransformDominantCyclePhase;
pub use hilbert_transform_phasor::{HilbertTransformPhasor, HilbertTransformPhasorValue};
pub use hilbert_transform_sine_wave::{HilbertTransformSineWave, HilbertTransformSineWaveValue};
pub use hilbert_transform_trend_mode::HilbertTransformTrendMode;
pub use hurst::Hurst;

mod chaikin_money_flow;
#[cfg(test)]
mod chaikin_money_flow_test;
mod chaikin_volatility;
#[cfg(test)]
mod chaikin_volatility_test;
mod chande_momentum_oscillator;
#[cfg(test)]
mod chande_momentum_oscillator_test;
mod close_to_close_sigma;
#[cfg(test)]
mod close_to_close_sigma_test;
mod commodity_channel_index;
#[cfg(test)]
mod commodity_channel_index_test;
mod cross;
#[cfg(test)]
mod cross_test;
mod crossover;
#[cfg(test)]
mod crossover_test;
mod crossunder;
#[cfg(test)]
mod crossunder_test;
mod cumulative_maximum;
#[cfg(test)]
mod cumulative_maximum_test;
mod cumulative_minimum;
#[cfg(test)]
mod cumulative_minimum_test;
mod cumulative_sum_control_chart;
#[cfg(test)]
mod cumulative_sum_control_chart_test;
mod detrended_price_oscillator;
#[cfg(test)]
mod detrended_price_oscillator_test;
mod donchian;
#[cfg(test)]
mod donchian_test;
mod equal_highs_lows;
#[cfg(test)]
mod equal_highs_lows_test;
mod fair_value_gap;
#[cfg(test)]
mod fair_value_gap_test;
mod falling;
#[cfg(test)]
mod falling_test;
mod fractal_dimension;
#[cfg(test)]
mod fractal_dimension_test;
mod gap_down;
#[cfg(test)]
mod gap_down_test;
mod gap_up;
#[cfg(test)]
mod gap_up_test;
mod higher_high;
#[cfg(test)]
mod higher_high_test;
mod inside_bar;
#[cfg(test)]
mod inside_bar_test;
mod keltner_channels;
#[cfg(test)]
mod keltner_channels_test;
mod lower_low;
#[cfg(test)]
mod lower_low_test;
mod negative_volume_index;
#[cfg(test)]
mod negative_volume_index_test;
mod order_block;
#[cfg(test)]
mod order_block_test;
mod outside_bar;
#[cfg(test)]
mod outside_bar_test;
mod positive_volume_index;
#[cfg(test)]
mod positive_volume_index_test;
mod rising;
#[cfg(test)]
mod rising_test;
mod rolling_alpha;
#[cfg(test)]
mod rolling_alpha_test;
mod rolling_information_ratio;
#[cfg(test)]
mod rolling_information_ratio_test;
mod spread_z_score;
#[cfg(test)]
mod spread_z_score_test;
mod swing_high_low;
#[cfg(test)]
mod swing_high_low_test;
mod ulcer_index;
#[cfg(test)]
mod ulcer_index_test;
mod volume_price_trend;
#[cfg(test)]
mod volume_price_trend_test;

pub use chaikin_money_flow::ChaikinMoneyFlow;
pub use chaikin_volatility::ChaikinVolatility;
pub use chande_momentum_oscillator::ChandeMomentumOscillator;
pub use close_to_close_sigma::CloseToCloseSigma;
pub use commodity_channel_index::CommodityChannelIndex;
pub use cross::Cross;
pub use crossover::Crossover;
pub use crossunder::Crossunder;
pub use cumulative_maximum::CumulativeMaximum;
pub use cumulative_minimum::CumulativeMinimum;
pub use cumulative_sum_control_chart::CumulativeSumControlChart;
pub use detrended_price_oscillator::DetrendedPriceOscillator;
pub use donchian::{Donchian, DonchianValue};
pub use equal_highs_lows::EqualHighsLows;
pub use fair_value_gap::{FairValueGap, FairValueGapValue};
pub use falling::Falling;
pub use fractal_dimension::FractalDimension;
pub use gap_down::GapDown;
pub use gap_up::GapUp;
pub use higher_high::HigherHigh;
pub use inside_bar::InsideBar;
pub use keltner_channels::{KeltnerChannels, KeltnerValue};
pub use lower_low::LowerLow;
pub use negative_volume_index::NegativeVolumeIndex;
pub use order_block::{OrderBlock, OrderBlockValue};
pub use outside_bar::OutsideBar;
pub use positive_volume_index::PositiveVolumeIndex;
pub use rising::Rising;
pub use rolling_alpha::RollingAlpha;
pub use rolling_information_ratio::RollingInformationRatio;
pub use spread_z_score::SpreadZScore;
pub use swing_high_low::{SwingHighLow, SwingValue};
pub use ulcer_index::UlcerIndex;
pub use volume_price_trend::VolumePriceTrend;

mod candle_three_stars_in_south;
#[cfg(test)]
mod candle_three_stars_in_south_test;
mod candle_three_white_soldiers;
#[cfg(test)]
mod candle_three_white_soldiers_test;
mod candle_thrusting;
#[cfg(test)]
mod candle_thrusting_test;
mod candle_tri_star;
#[cfg(test)]
mod candle_tri_star_test;
mod candle_two_crows;
#[cfg(test)]
mod candle_two_crows_test;
mod candle_unique_three_river;
#[cfg(test)]
mod candle_unique_three_river_test;
mod candle_up_down_side_gap_three_methods;
#[cfg(test)]
mod candle_up_down_side_gap_three_methods_test;
mod candle_upside_gap_two_crows;
#[cfg(test)]
mod candle_upside_gap_two_crows_test;

pub use candle_three_stars_in_south::CandleThreeStarsInSouth;
pub use candle_three_white_soldiers::CandleThreeWhiteSoldiers;
pub use candle_thrusting::CandleThrusting;
pub use candle_tri_star::CandleTriStar;
pub use candle_two_crows::CandleTwoCrows;
pub use candle_unique_three_river::CandleUniqueThreeRiver;
pub use candle_up_down_side_gap_three_methods::CandleUpDownSideGapThreeMethods;
pub use candle_upside_gap_two_crows::CandleUpsideGapTwoCrows;

mod candle_mat_hold;
#[cfg(test)]
mod candle_mat_hold_test;
mod candle_matching_low;
#[cfg(test)]
mod candle_matching_low_test;
mod candle_rise_fall_three_methods;
#[cfg(test)]
mod candle_rise_fall_three_methods_test;
mod candle_separating_lines;
#[cfg(test)]
mod candle_separating_lines_test;
mod candle_takuri;
#[cfg(test)]
mod candle_takuri_test;
mod candle_tasuki_gap;
#[cfg(test)]
mod candle_tasuki_gap_test;
mod candle_three_black_crows;
#[cfg(test)]
mod candle_three_black_crows_test;
mod candle_three_inside;
#[cfg(test)]
mod candle_three_inside_test;
mod candle_three_line_strike;
#[cfg(test)]
mod candle_three_line_strike_test;
mod candle_three_outside;
#[cfg(test)]
mod candle_three_outside_test;

pub use candle_mat_hold::CandleMatHold;
pub use candle_matching_low::CandleMatchingLow;
pub use candle_rise_fall_three_methods::CandleRiseFallThreeMethods;
pub use candle_separating_lines::CandleSeparatingLines;
pub use candle_takuri::CandleTakuri;
pub use candle_tasuki_gap::CandleTasukiGap;
pub use candle_three_black_crows::CandleThreeBlackCrows;
pub use candle_three_inside::CandleThreeInside;
pub use candle_three_line_strike::CandleThreeLineStrike;
pub use candle_three_outside::CandleThreeOutside;

mod candle_morning_doji_star;
#[cfg(test)]
mod candle_morning_doji_star_test;
mod candle_morning_star;
#[cfg(test)]
mod candle_morning_star_test;
mod candle_on_neck;
#[cfg(test)]
mod candle_on_neck_test;
mod candle_piercing;
#[cfg(test)]
mod candle_piercing_test;
mod candle_rickshawman;
#[cfg(test)]
mod candle_rickshawman_test;
mod candle_shooting_star;
#[cfg(test)]
mod candle_shooting_star_test;
mod candle_short_line;
#[cfg(test)]
mod candle_short_line_test;
mod candle_spinning_top;
#[cfg(test)]
mod candle_spinning_top_test;
mod candle_stalled_pattern;
#[cfg(test)]
mod candle_stalled_pattern_test;
mod candle_stick_sandwich;
#[cfg(test)]
mod candle_stick_sandwich_test;

pub use candle_morning_doji_star::CandleMorningDojiStar;
pub use candle_morning_star::CandleMorningStar;
pub use candle_on_neck::CandleOnNeck;
pub use candle_piercing::CandlePiercing;
pub use candle_rickshawman::CandleRickshawman;
pub use candle_shooting_star::CandleShootingStar;
pub use candle_short_line::CandleShortLine;
pub use candle_spinning_top::CandleSpinningTop;
pub use candle_stalled_pattern::CandleStalledPattern;
pub use candle_stick_sandwich::CandleStickSandwich;

mod candle_homing_pigeon;
#[cfg(test)]
mod candle_homing_pigeon_test;
mod candle_identical_three_crows;
#[cfg(test)]
mod candle_identical_three_crows_test;
mod candle_in_neck;
#[cfg(test)]
mod candle_in_neck_test;
mod candle_inverted_hammer;
#[cfg(test)]
mod candle_inverted_hammer_test;
mod candle_kicking;
mod candle_kicking_by_length;
#[cfg(test)]
mod candle_kicking_by_length_test;
#[cfg(test)]
mod candle_kicking_test;
mod candle_ladder_bottom;
#[cfg(test)]
mod candle_ladder_bottom_test;
mod candle_long_legged_doji;
#[cfg(test)]
mod candle_long_legged_doji_test;
mod candle_long_line;
#[cfg(test)]
mod candle_long_line_test;
mod candle_marubozu;
#[cfg(test)]
mod candle_marubozu_test;

pub use candle_homing_pigeon::CandleHomingPigeon;
pub use candle_identical_three_crows::CandleIdenticalThreeCrows;
pub use candle_in_neck::CandleInNeck;
pub use candle_inverted_hammer::CandleInvertedHammer;
pub use candle_kicking::CandleKicking;
pub use candle_kicking_by_length::CandleKickingByLength;
pub use candle_ladder_bottom::CandleLadderBottom;
pub use candle_long_legged_doji::CandleLongLeggedDoji;
pub use candle_long_line::CandleLongLine;
pub use candle_marubozu::CandleMarubozu;

mod candle_evening_star;
#[cfg(test)]
mod candle_evening_star_test;
mod candle_gap_side_side_white;
#[cfg(test)]
mod candle_gap_side_side_white_test;
mod candle_gravestone_doji;
#[cfg(test)]
mod candle_gravestone_doji_test;
mod candle_hammer;
#[cfg(test)]
mod candle_hammer_test;
mod candle_hanging_man;
#[cfg(test)]
mod candle_hanging_man_test;
mod candle_harami;
mod candle_harami_cross;
#[cfg(test)]
mod candle_harami_cross_test;
#[cfg(test)]
mod candle_harami_test;
mod candle_high_wave;
#[cfg(test)]
mod candle_high_wave_test;
mod candle_hikkake;
mod candle_hikkake_modified;
#[cfg(test)]
mod candle_hikkake_modified_test;
#[cfg(test)]
mod candle_hikkake_test;

pub use candle_evening_star::CandleEveningStar;
pub use candle_gap_side_side_white::CandleGapSideSideWhite;
pub use candle_gravestone_doji::CandleGravestoneDoji;
pub use candle_hammer::CandleHammer;
pub use candle_hanging_man::CandleHangingMan;
pub use candle_harami::CandleHarami;
pub use candle_harami_cross::CandleHaramiCross;
pub use candle_high_wave::CandleHighWave;
pub use candle_hikkake::CandleHikkake;
pub use candle_hikkake_modified::CandleHikkakeModified;

mod candle_breakaway;
#[cfg(test)]
mod candle_breakaway_test;
mod candle_closing_marubozu;
#[cfg(test)]
mod candle_closing_marubozu_test;
mod candle_conceal_baby_swall;
#[cfg(test)]
mod candle_conceal_baby_swall_test;
mod candle_counter_attack;
#[cfg(test)]
mod candle_counter_attack_test;
mod candle_dark_cloud_cover;
#[cfg(test)]
mod candle_dark_cloud_cover_test;
mod candle_doji;
mod candle_doji_star;
#[cfg(test)]
mod candle_doji_star_test;
#[cfg(test)]
mod candle_doji_test;
mod candle_dragonfly_doji;
#[cfg(test)]
mod candle_dragonfly_doji_test;
mod candle_engulfing;
#[cfg(test)]
mod candle_engulfing_test;
mod candle_evening_doji_star;
#[cfg(test)]
mod candle_evening_doji_star_test;

pub use candle_breakaway::CandleBreakaway;
pub use candle_closing_marubozu::CandleClosingMarubozu;
pub use candle_conceal_baby_swall::CandleConcealBabySwall;
pub use candle_counter_attack::CandleCounterAttack;
pub use candle_dark_cloud_cover::CandleDarkCloudCover;
pub use candle_doji::CandleDoji;
pub use candle_doji_star::CandleDojiStar;
pub use candle_dragonfly_doji::CandleDragonflyDoji;
pub use candle_engulfing::CandleEngulfing;
pub use candle_evening_doji_star::CandleEveningDojiStar;

mod absolute_price_oscillator;
#[cfg(test)]
mod absolute_price_oscillator_test;
mod active_zone_list;
#[cfg(test)]
mod active_zone_list_test;
mod amihud;
#[cfg(test)]
mod amihud_test;
mod anchored_volume_weighted_average_price;
#[cfg(test)]
mod anchored_volume_weighted_average_price_test;
mod average_daily_dollar_value;
#[cfg(test)]
mod average_daily_dollar_value_test;
mod bollinger_bands;
#[cfg(test)]
mod bollinger_bands_test;
mod break_of_structure_change_of_character;
#[cfg(test)]
mod break_of_structure_change_of_character_test;
mod candle_abandoned_baby;
#[cfg(test)]
mod candle_abandoned_baby_test;
mod candle_advance_block;
#[cfg(test)]
mod candle_advance_block_test;
mod candle_belt_hold;
#[cfg(test)]
mod candle_belt_hold_test;

pub use absolute_price_oscillator::AbsolutePriceOscillator;
pub use active_zone_list::ActiveZoneList;
pub use amihud::Amihud;
pub use anchored_volume_weighted_average_price::{
    AnchoredVolumeWeightedAveragePrice, AnchoredVolumeWeightedAveragePriceValue,
};
pub use average_daily_dollar_value::AverageDailyDollarValue;
pub use bollinger_bands::{BollingerBands, BollingerBandsValue};
pub use break_of_structure_change_of_character::{
    BreakOfStructureChangeOfCharacter, BreakOfStructureChangeOfCharacterValue,
};
pub use candle_abandoned_baby::CandleAbandonedBaby;
pub use candle_advance_block::CandleAdvanceBlock;
pub use candle_belt_hold::CandleBeltHold;

mod time_series_rank;
#[cfg(test)]
mod time_series_rank_test;
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
mod value_when;
#[cfg(test)]
mod value_when_test;
mod variable_index_dynamic_average;
#[cfg(test)]
mod variable_index_dynamic_average_test;
mod vortex;
#[cfg(test)]
mod vortex_test;
mod weighted_moving_average;
#[cfg(test)]
mod weighted_moving_average_test;
mod yang_zhang;
#[cfg(test)]
mod yang_zhang_test;

pub use time_series_rank::TimeSeriesRank;
pub use triple_exponential_average::TripleExponentialAverage;
pub use triple_exponential_moving_average::TripleExponentialMovingAverage;
pub use triple_exponential_rate_of_change::TripleExponentialRateOfChange;
pub use ultimate_oscillator::UltimateOscillator;
pub use value_when::ValueWhen;
pub use variable_index_dynamic_average::VariableIndexDynamicAverage;
pub use vortex::{Vortex, VortexValue};
pub use weighted_moving_average::WeightedMovingAverage;
pub use yang_zhang::YangZhang;
mod rolling_average_drawdown;
#[cfg(test)]
mod rolling_average_drawdown_test;
mod rolling_cointegration;
#[cfg(test)]
mod rolling_cointegration_test;
mod rolling_drawdown_duration;
#[cfg(test)]
mod rolling_drawdown_duration_test;
mod rolling_gain_loss_ratio;
#[cfg(test)]
mod rolling_gain_loss_ratio_test;
mod rolling_granger_causality;
#[cfg(test)]
mod rolling_granger_causality_test;
mod rolling_kendall_rank_correlation;
#[cfg(test)]
mod rolling_kendall_rank_correlation_test;
mod rolling_pain_index;
#[cfg(test)]
mod rolling_pain_index_test;
mod rolling_spearman_correlation;
#[cfg(test)]
mod rolling_spearman_correlation_test;
mod rolling_variance_ratio;
#[cfg(test)]
mod rolling_variance_ratio_test;
mod volume_relative_strength_index;
#[cfg(test)]
mod volume_relative_strength_index_test;
pub use rolling_average_drawdown::RollingAverageDrawdown;
pub use rolling_cointegration::RollingCointegration;
pub use rolling_drawdown_duration::RollingDrawdownDuration;
pub use rolling_gain_loss_ratio::RollingGainLossRatio;
pub use rolling_granger_causality::RollingGrangerCausality;
mod better_volume;
#[cfg(test)]
mod better_volume_test;
mod intraday_intensity;
#[cfg(test)]
mod intraday_intensity_test;
mod rolling_beta_neutral_spread;
#[cfg(test)]
mod rolling_beta_neutral_spread_test;
mod rolling_coefficient_of_determination;
#[cfg(test)]
mod rolling_coefficient_of_determination_test;
mod rolling_lead_lag_cross_correlation;
#[cfg(test)]
mod rolling_lead_lag_cross_correlation_test;
mod rolling_median_absolute_deviation;
#[cfg(test)]
mod rolling_median_absolute_deviation_test;
mod rolling_pairwise_beta;
#[cfg(test)]
mod rolling_pairwise_beta_test;
mod rolling_standard_error;
#[cfg(test)]
mod rolling_standard_error_test;
mod trade_volume_index;
#[cfg(test)]
mod trade_volume_index_test;
mod volume_weighted_moving_average_convergence_divergence;
#[cfg(test)]
mod volume_weighted_moving_average_convergence_divergence_test;
pub use better_volume::BetterVolume;
pub use intraday_intensity::IntradayIntensity;
pub use rolling_beta_neutral_spread::RollingBetaNeutralSpread;
pub use rolling_coefficient_of_determination::RollingCoefficientOfDetermination;
pub use rolling_kendall_rank_correlation::RollingKendallRankCorrelation;
pub use rolling_lead_lag_cross_correlation::RollingLeadLagCrossCorrelation;
pub use rolling_median_absolute_deviation::RollingMedianAbsoluteDeviation;
pub use rolling_pain_index::RollingPainIndex;
pub use rolling_pairwise_beta::RollingPairwiseBeta;
pub use rolling_spearman_correlation::RollingSpearmanCorrelation;
pub use rolling_standard_error::RollingStandardError;
pub use rolling_variance_ratio::RollingVarianceRatio;
pub use trade_volume_index::TradeVolumeIndex;
mod average_true_range_bands;
#[cfg(test)]
mod average_true_range_bands_test;
mod double_bollinger_bands;
#[cfg(test)]
mod double_bollinger_bands_test;
mod linear_regression_channel;
#[cfg(test)]
mod linear_regression_channel_test;
mod market_facilitation_index;
#[cfg(test)]
mod market_facilitation_index_test;
mod moving_average_envelope;
#[cfg(test)]
mod moving_average_envelope_test;
mod projection_bands;
#[cfg(test)]
mod projection_bands_test;
mod standard_error_bands;
#[cfg(test)]
mod standard_error_bands_test;
mod time_segmented_volume;
#[cfg(test)]
mod time_segmented_volume_test;
mod twiggs_money_flow;
#[cfg(test)]
mod twiggs_money_flow_test;
mod williams_accumulation_distribution;
#[cfg(test)]
mod williams_accumulation_distribution_test;
pub use average_true_range_bands::AverageTrueRangeBands;
pub use double_bollinger_bands::DoubleBollingerBands;
pub use linear_regression_channel::LinearRegressionChannel;
pub use market_facilitation_index::MarketFacilitationIndex;
pub use moving_average_envelope::MovingAverageEnvelope;
pub use projection_bands::ProjectionBands;
mod adaptive_cycle;
#[cfg(test)]
mod adaptive_cycle_test;
mod center_of_gravity;
#[cfg(test)]
mod center_of_gravity_test;
mod decycler;
mod decycler_oscillator;
#[cfg(test)]
mod decycler_oscillator_test;
#[cfg(test)]
mod decycler_test;
mod ehlers_stochastic;
#[cfg(test)]
mod ehlers_stochastic_test;
mod hurst_channel;
#[cfg(test)]
mod hurst_channel_test;
mod instantaneous_trendline;
#[cfg(test)]
mod instantaneous_trendline_test;
mod inverse_fisher_transform;
#[cfg(test)]
mod inverse_fisher_transform_test;
mod roofing_filter;
#[cfg(test)]
mod roofing_filter_test;
mod super_smoother;
#[cfg(test)]
mod super_smoother_test;
pub use adaptive_cycle::AdaptiveCycle;
pub use center_of_gravity::CenterOfGravity;
pub use decycler::Decycler;
pub use decycler_oscillator::DecyclerOscillator;
pub use ehlers_stochastic::EhlersStochastic;
pub use hurst_channel::HurstChannel;
pub use instantaneous_trendline::InstantaneousTrendline;
pub use inverse_fisher_transform::InverseFisherTransform;
pub use roofing_filter::RoofingFilter;
pub use standard_error_bands::StandardErrorBands;
pub use super_smoother::SuperSmoother;
pub use time_segmented_volume::TimeSegmentedVolume;
pub use twiggs_money_flow::TwiggsMoneyFlow;
pub use volume_relative_strength_index::VolumeRelativeStrengthIndex;
pub use volume_weighted_moving_average_convergence_divergence::VolumeWeightedMovingAverageConvergenceDivergence;
pub use williams_accumulation_distribution::WilliamsAccumulationDistribution;
mod average_daily_range;
#[cfg(test)]
mod average_daily_range_test;
mod day_of_week_return_profile;
#[cfg(test)]
mod day_of_week_return_profile_test;
mod empirical_mode_decomposition;
#[cfg(test)]
mod empirical_mode_decomposition_test;
mod hilbert_dominant_cycle;
#[cfg(test)]
mod hilbert_dominant_cycle_test;
mod overnight_gap;
#[cfg(test)]
mod overnight_gap_test;
mod overnight_intraday_return;
#[cfg(test)]
mod overnight_intraday_return_test;
mod session_range;
#[cfg(test)]
mod session_range_test;
mod session_volume_weighted_average_price;
#[cfg(test)]
mod session_volume_weighted_average_price_test;
mod time_of_day_return_profile;
#[cfg(test)]
mod time_of_day_return_profile_test;
mod zig_zag;
#[cfg(test)]
mod zig_zag_test;
pub use average_daily_range::AverageDailyRange;
pub use day_of_week_return_profile::DayOfWeekReturnProfile;
pub use empirical_mode_decomposition::EmpiricalModeDecomposition;
pub use hilbert_dominant_cycle::HilbertDominantCycle;
pub use overnight_gap::OvernightGap;
pub use overnight_intraday_return::{OvernightIntradayReturn, OvernightIntradayReturnValue};
pub use session_range::SessionRange;
pub use session_volume_weighted_average_price::SessionVolumeWeightedAveragePrice;
pub use time_of_day_return_profile::TimeOfDayReturnProfile;
pub use zig_zag::{ZigZag, ZigZagValue};
mod absolute_breadth_index;
#[cfg(test)]
mod absolute_breadth_index_test;
mod bullish_percent_index;
#[cfg(test)]
mod bullish_percent_index_test;
mod cumulative_volume_index;
#[cfg(test)]
mod cumulative_volume_index_test;
mod high_low_index;
#[cfg(test)]
mod high_low_index_test;
mod intraday_volatility_profile;
#[cfg(test)]
mod intraday_volatility_profile_test;
mod median_channel;
#[cfg(test)]
mod median_channel_test;
mod percent_above_moving_average;
#[cfg(test)]
mod percent_above_moving_average_test;
mod quartile_bands;
#[cfg(test)]
mod quartile_bands_test;
mod up_down_volume_ratio;
#[cfg(test)]
mod up_down_volume_ratio_test;
mod volume_by_time_profile;
#[cfg(test)]
mod volume_by_time_profile_test;
pub use absolute_breadth_index::AbsoluteBreadthIndex;
pub use bullish_percent_index::BullishPercentIndex;
pub use cumulative_volume_index::CumulativeVolumeIndex;
pub use high_low_index::HighLowIndex;
pub use intraday_volatility_profile::IntradayVolatilityProfile;
pub use median_channel::{MedianChannel, MedianChannelValue};
pub use percent_above_moving_average::PercentAboveMovingAverage;
pub use quartile_bands::{QuartileBands, QuartileBandsValue};
pub use up_down_volume_ratio::UpDownVolumeRatio;
pub use volume_by_time_profile::VolumeByTimeProfile;
mod arms_index;
#[cfg(test)]
mod arms_index_test;
mod breadth_thrust;
#[cfg(test)]
mod breadth_thrust_test;
mod cup_and_handle;
#[cfg(test)]
mod cup_and_handle_test;
mod flag_pennant;
#[cfg(test)]
mod flag_pennant_test;
mod mc_clellan_oscillator;
#[cfg(test)]
mod mc_clellan_oscillator_test;
mod mc_clellan_summation_index;
#[cfg(test)]
mod mc_clellan_summation_index_test;
mod new_highs_new_lows;
#[cfg(test)]
mod new_highs_new_lows_test;
mod rectangle_range;
#[cfg(test)]
mod rectangle_range_test;
mod triangle_pattern;
#[cfg(test)]
mod triangle_pattern_test;
mod wedge_pattern;
#[cfg(test)]
mod wedge_pattern_test;
pub use arms_index::ArmsIndex;
pub use breadth_thrust::BreadthThrust;
pub use cup_and_handle::CupAndHandle;
pub use flag_pennant::FlagPennant;
pub use mc_clellan_oscillator::McClellanOscillator;
pub use mc_clellan_summation_index::McClellanSummationIndex;
pub use new_highs_new_lows::NewHighsNewLows;
pub use rectangle_range::RectangleRange;
pub use triangle_pattern::TrianglePattern;
pub use wedge_pattern::WedgePattern;
mod bat_pattern;
#[cfg(test)]
mod bat_pattern_test;
mod butterfly_pattern;
#[cfg(test)]
mod butterfly_pattern_test;
mod crab_pattern;
#[cfg(test)]
mod crab_pattern_test;
mod cypher_pattern;
#[cfg(test)]
mod cypher_pattern_test;
mod four_point_harmonic_pattern;
#[cfg(test)]
mod four_point_harmonic_pattern_test;
mod gartley_pattern;
#[cfg(test)]
mod gartley_pattern_test;
mod head_and_shoulders;
#[cfg(test)]
mod head_and_shoulders_test;
mod shark_pattern;
#[cfg(test)]
mod shark_pattern_test;
mod three_drives;
#[cfg(test)]
mod three_drives_test;
mod triple_top_bottom;
#[cfg(test)]
mod triple_top_bottom_test;
pub use bat_pattern::BatPattern;
pub use butterfly_pattern::ButterflyPattern;
pub use crab_pattern::CrabPattern;
pub use cypher_pattern::CypherPattern;
pub use four_point_harmonic_pattern::FourPointHarmonicPattern;
pub use gartley_pattern::GartleyPattern;
pub use head_and_shoulders::HeadAndShoulders;
pub use shark_pattern::SharkPattern;
pub use three_drives::ThreeDrives;
pub use triple_top_bottom::TripleTopBottom;
mod automatic_fibonacci;
#[cfg(test)]
mod automatic_fibonacci_test;
mod fibonacci_arcs;
#[cfg(test)]
mod fibonacci_arcs_test;
mod fibonacci_channel;
#[cfg(test)]
mod fibonacci_channel_test;
mod fibonacci_confluence;
#[cfg(test)]
mod fibonacci_confluence_test;
mod fibonacci_extension;
#[cfg(test)]
mod fibonacci_extension_test;
mod fibonacci_fan;
#[cfg(test)]
mod fibonacci_fan_test;
mod fibonacci_projection;
#[cfg(test)]
mod fibonacci_projection_test;
mod fibonacci_time_zones;
#[cfg(test)]
mod fibonacci_time_zones_test;
mod golden_pocket;
#[cfg(test)]
mod golden_pocket_test;
pub use automatic_fibonacci::{AutomaticFibonacci, AutomaticFibonacciValue};
pub use fibonacci_arcs::{FibonacciArcs, FibonacciArcsValue};
pub use fibonacci_channel::{FibonacciChannel, FibonacciChannelValue};
pub use fibonacci_confluence::{FibonacciConfluence, FibonacciConfluenceValue};
pub use fibonacci_extension::{FibonacciExtension, FibonacciExtensionValue};
pub use fibonacci_fan::{FibonacciFan, FibonacciFanValue};
pub use fibonacci_projection::{FibonacciProjection, FibonacciProjectionValue};
pub use fibonacci_time_zones::{FibonacciTimeZones, FibonacciTimeZonesValue};
pub use golden_pocket::{GoldenPocket, GoldenPocketValue};
