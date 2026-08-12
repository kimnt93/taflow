// Canonical public metric modules are declared and re-exported here.
mod alpha;
mod annualized_return;
mod annualized_volatility;
mod average_drawdown;
mod average_loss;
mod average_win;
mod beta;
mod breakeven_rate;
mod calmar_ratio;
mod capture_ratio;
mod coefficient_of_determination;
mod common_sense_ratio;
mod composite_profitability_consistency_index;
mod conditional_drawdown_at_risk;
mod deflated_sharpe_ratio;
mod down_market_capture_ratio;
mod downside_deviation;
mod effective_number_of_bets;
mod entropic_value_at_risk;
mod expectancy;
mod exposure;
mod gain_to_pain_ratio;
mod gross_loss;
mod gross_profit;
mod historical_expected_shortfall;
mod historical_value_at_risk;
mod information_ratio;
mod kelly_criterion;
mod longest_losing_streak;
mod longest_winning_streak;
mod maximum_drawdown;
mod maximum_drawdown_duration;
mod modified_sharpe_ratio;
mod net_profit;
mod omega_ratio;
mod pain_index;
mod pain_ratio;
mod parametric_expected_shortfall;
mod parametric_value_at_risk;
mod payoff_ratio;
mod probabilistic_sharpe_ratio;
mod profit_factor;
mod recovery_factor;
mod sharpe_ratio;
mod sortino_ratio;
mod stability_of_time_series;
mod system_quality_number;
mod tail_ratio;
mod total_return;
mod tracking_error;
mod treynor_ratio;
mod turnover;
mod ulcer_index;
mod ulcer_performance_index;
mod up_down_capture_ratio;
mod up_market_capture_ratio;
mod win_rate;
pub use alpha::Alpha;
pub use annualized_return::AnnualizedReturn;
pub use annualized_volatility::AnnualizedVolatility;
pub use average_drawdown::AverageDrawdown;
pub use average_loss::AverageLoss;
pub use average_win::AverageWin;
pub use beta::Beta;
pub use breakeven_rate::BreakevenRate;
pub use calmar_ratio::CalmarRatio;
pub use capture_ratio::CaptureRatio;
pub use coefficient_of_determination::CoefficientOfDetermination;
pub use common_sense_ratio::CommonSenseRatio;
pub use composite_profitability_consistency_index::CompositeProfitabilityConsistencyIndex;
pub use conditional_drawdown_at_risk::ConditionalDrawdownAtRisk;
pub use deflated_sharpe_ratio::DeflatedSharpeRatio;
pub use down_market_capture_ratio::DownMarketCaptureRatio;
pub use downside_deviation::DownsideDeviation;
pub use effective_number_of_bets::EffectiveNumberOfBets;
pub use entropic_value_at_risk::EntropicValueAtRisk;
pub use expectancy::Expectancy;
pub use exposure::{Exposure, ExposureInputKind};
pub use gain_to_pain_ratio::GainToPainRatio;
pub use gross_loss::GrossLoss;
pub use gross_profit::GrossProfit;
pub use historical_expected_shortfall::HistoricalExpectedShortfall;
pub use historical_value_at_risk::HistoricalValueAtRisk;
pub use information_ratio::InformationRatio;
pub use kelly_criterion::KellyCriterion;
pub use longest_losing_streak::LongestLosingStreak;
pub use longest_winning_streak::LongestWinningStreak;
pub use maximum_drawdown::MaximumDrawdown;
pub use maximum_drawdown_duration::MaximumDrawdownDuration;
pub use modified_sharpe_ratio::ModifiedSharpeRatio;
pub use net_profit::NetProfit;
pub use omega_ratio::OmegaRatio;
pub use pain_index::PainIndex;
pub use pain_ratio::PainRatio;
pub use parametric_expected_shortfall::ParametricExpectedShortfall;
pub use parametric_value_at_risk::ParametricValueAtRisk;
pub use payoff_ratio::PayoffRatio;
pub use probabilistic_sharpe_ratio::ProbabilisticSharpeRatio;
pub use profit_factor::ProfitFactor;
pub use recovery_factor::RecoveryFactor;
pub use sharpe_ratio::SharpeRatio;
pub use sortino_ratio::SortinoRatio;
pub use stability_of_time_series::StabilityOfTimeSeries;
pub use system_quality_number::SystemQualityNumber;
pub use tail_ratio::TailRatio;
pub use total_return::TotalReturn;
pub use tracking_error::TrackingError;
pub use treynor_ratio::TreynorRatio;
pub use turnover::Turnover;
pub use ulcer_index::UlcerIndex;
pub use ulcer_performance_index::UlcerPerformanceIndex;
pub use up_down_capture_ratio::UpDownCaptureRatio;
pub use up_market_capture_ratio::UpMarketCaptureRatio;
pub use win_rate::WinRate;

#[cfg(test)]
mod alpha_test;
#[cfg(test)]
mod annualized_return_test;
#[cfg(test)]
mod annualized_volatility_test;
#[cfg(test)]
mod average_drawdown_test;
#[cfg(test)]
mod average_loss_test;
#[cfg(test)]
mod average_win_test;
#[cfg(test)]
mod beta_test;
#[cfg(test)]
mod breakeven_rate_test;
#[cfg(test)]
mod calmar_ratio_test;
#[cfg(test)]
mod capture_ratio_test;
#[cfg(test)]
mod coefficient_of_determination_test;
#[cfg(test)]
mod common_sense_ratio_test;
#[cfg(test)]
mod composite_profitability_consistency_index_test;
#[cfg(test)]
mod conditional_drawdown_at_risk_test;
#[cfg(test)]
mod deflated_sharpe_ratio_test;
#[cfg(test)]
mod down_market_capture_ratio_test;
#[cfg(test)]
mod downside_deviation_test;
#[cfg(test)]
mod effective_number_of_bets_test;
#[cfg(test)]
mod entropic_value_at_risk_test;
#[cfg(test)]
mod expectancy_test;
#[cfg(test)]
mod exposure_test;
#[cfg(test)]
mod gain_to_pain_ratio_test;
#[cfg(test)]
mod gross_loss_test;
#[cfg(test)]
mod gross_profit_test;
#[cfg(test)]
mod historical_expected_shortfall_test;
#[cfg(test)]
mod historical_value_at_risk_test;
#[cfg(test)]
mod information_ratio_test;
#[cfg(test)]
mod kelly_criterion_test;
#[cfg(test)]
mod longest_losing_streak_test;
#[cfg(test)]
mod longest_winning_streak_test;
#[cfg(test)]
mod maximum_drawdown_duration_test;
#[cfg(test)]
mod maximum_drawdown_test;
#[cfg(test)]
mod modified_sharpe_ratio_test;
#[cfg(test)]
mod net_profit_test;
#[cfg(test)]
mod omega_ratio_test;
#[cfg(test)]
mod pain_index_test;
#[cfg(test)]
mod pain_ratio_test;
#[cfg(test)]
mod parametric_expected_shortfall_test;
#[cfg(test)]
mod parametric_value_at_risk_test;
#[cfg(test)]
mod payoff_ratio_test;
#[cfg(test)]
mod probabilistic_sharpe_ratio_test;
#[cfg(test)]
mod profit_factor_test;
#[cfg(test)]
mod recovery_factor_test;
#[cfg(test)]
mod sharpe_ratio_test;
#[cfg(test)]
mod sortino_ratio_test;
#[cfg(test)]
mod stability_of_time_series_test;
#[cfg(test)]
mod system_quality_number_test;
#[cfg(test)]
mod tail_ratio_test;
#[cfg(test)]
mod total_return_test;
#[cfg(test)]
mod tracking_error_test;
#[cfg(test)]
mod treynor_ratio_test;
#[cfg(test)]
mod turnover_test;
#[cfg(test)]
mod ulcer_index_test;
#[cfg(test)]
mod ulcer_performance_index_test;
#[cfg(test)]
mod up_down_capture_ratio_test;
#[cfg(test)]
mod up_market_capture_ratio_test;
#[cfg(test)]
mod win_rate_test;
