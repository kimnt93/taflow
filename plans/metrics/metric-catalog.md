# Metric catalog and implementation priority

Checkboxes mean implementation, public-class oracle parity, lifecycle tests,
and interface gates are complete. Do not check an item after merely creating a
file or matching a local formula.

## P0: foundation

- [x] Add the `taflow-metrics` workspace crate and one-way dependency graph.
- [x] Add return, log-return, equity-level, and period-P&L native input modes.
- [x] Add paired benchmark input and pairwise missing-value handling.
- [x] Add online moments, paired moments, compounded growth, drawdown,
      downside-moment, gain/loss, and exact-order-statistic primitives.
- [x] Add the native `taflow._native.metrics` submodule.
- [x] Add `taflow.metrics` and the class-only adapter template.
- [x] Add metrics-specific registry, interface audit, correctness report, and
      benchmark harness. The benchmark executable remains disabled until
      explicitly authorized.
- [x] Freeze the edge-result matrix and oracle normalization before P1.

## P1: essential return and risk metrics

These are the first public release. They cover the metrics users most often
expect beside a strategy return stream.

| Done | Canonical class | Minimum input | Primary oracle | Exact contract note |
|:---:|---|---:|---|---|
| [x] | `TotalReturn` | 1 return | Empyrical `cum_returns_final` | Compounded simple return, not arithmetic sum. |
| [x] | `AnnualizedReturn` | 1 | Empyrical `annual_return` | Geometric CAGR using explicit `periods_per_year`. |
| [x] | `AnnualizedVolatility` | 2 | Empyrical `annual_volatility` | Sample standard deviation, Levy alpha fixed at 2. |
| [x] | `MaximumDrawdown` | 1 | Empyrical `max_drawdown` | Non-positive fraction from a phantom starting wealth of 1. |
| [x] | `DownsideDeviation` | 1 | Empyrical `downside_risk` | Lower partial moment over all observations, annualized. |
| [x] | `SharpeRatio` | 2 | Empyrical `sharpe_ratio` | Sample deviation; annual risk-free rate converted by TAFlow. |
| [x] | `SortinoRatio` | 2 | Empyrical `sortino_ratio` | Explicit annual minimum acceptable return. |
| [x] | `CalmarRatio` | 1 | Empyrical `calmar_ratio` | CAGR divided by absolute maximum drawdown. |
| [x] | `OmegaRatio` | 2 | Empyrical `omega_ratio` | Sum above threshold divided by absolute sum below threshold. |
| [x] | `HistoricalValueAtRisk` | 1 | Empyrical `value_at_risk` | Signed lower-tail linear quantile; default cutoff 0.05. |
| [x] | `HistoricalExpectedShortfall` | 1 | Empyrical `conditional_value_at_risk` | Signed mean of Empyrical's selected lowest order statistics. |
| [x] | `TailRatio` | 1 | Empyrical `tail_ratio` | Absolute 95th percentile divided by absolute 5th percentile. |

P1 intentionally omits rolling output. Existing TAFlow indicators own causal
aligned rolling series. These classes summarize all observations processed by
their own persistent metric object.

## P2: benchmark-relative metrics

All inputs are aligned pairs. Annualized and unannualized variants must be
parameters of one class, not separate classes.

| Done | Canonical class | Primary oracle | Exact contract note |
|:---:|---|---|---|
| [x] | `TrackingError` | NumPy/pandas plus QuantStats IR denominator | Sample standard deviation of active returns, annualized by default. |
| [x] | `InformationRatio` | Empyrical `excess_sharpe` and QuantStats `information_ratio` | Mean active return / tracking error; TAFlow annualizes by default. |
| [x] | `Beta` | Empyrical `beta_aligned` | Sample covariance / benchmark sample variance. |
| [x] | `Alpha` | Empyrical `alpha_aligned` | Intercept annualized with Empyrical's compounding convention. |
| [x] | `CoefficientOfDetermination` | QuantStats `r_squared` | Squared Pearson correlation for a single-factor fit. |
| [x] | `CaptureRatio` | Empyrical `capture` | Portfolio CAGR divided by benchmark CAGR. |
| [x] | `UpMarketCaptureRatio` | Empyrical `up_capture` | Filter periods where benchmark return is positive. |
| [x] | `DownMarketCaptureRatio` | Empyrical `down_capture` | Filter periods where benchmark return is negative. |
| [x] | `UpDownCaptureRatio` | Empyrical `up_down_capture` | Up capture divided by down capture. |
| [x] | `TreynorRatio` | PerformanceAnalytics `TreynorRatio` cross-check | Annual excess return divided by beta; defer if the exact return convention remains disputed. |

Do not infer date alignment from pandas indexes. The metric package accepts
already aligned ordered arrays and rejects mismatched lengths before mutation.

## P3: drawdown and path-quality metrics

| Done | Canonical class | Primary oracle | Exact contract note |
|:---:|---|---|---|
| [x] | `UlcerIndex` | QuantStats `ulcer_index`; PerformanceAnalytics cross-check | RMS percentage drawdown, positive magnitude. Separate namespace avoids collision with indicator `UlcerIndex`. |
| [x] | `UlcerPerformanceIndex` | QuantStats `ulcer_performance_index` | Excess return divided by Ulcer Index; freeze arithmetic-vs-compounded numerator first. |
| [x] | `RecoveryFactor` | QuantStats `recovery_factor` | Freeze compounded total-return versus arithmetic-sum variant before implementation. |
| [x] | `GainToPainRatio` | QuantStats `gain_to_pain_ratio` | Sum gains / absolute sum losses at a declared aggregation resolution. Phase 1 supports input resolution only. |
| [x] | `PainIndex` | PerformanceAnalytics `PainIndex` | Mean absolute percentage drawdown. |
| [x] | `PainRatio` | PerformanceAnalytics `PainRatio` | Annualized excess return / Pain Index. |
| [x] | `AverageDrawdown` | PerformanceAnalytics drawdown functions | Define episode-based average, not average per-bar drawdown. |
| [x] | `MaximumDrawdownDuration` | PerformanceAnalytics drawdown table | Count observations, not calendar days, in phase 1. |
| [x] | `StabilityOfTimeSeries` | Empyrical `stability_of_timeseries` | R-squared of cumulative log returns against observation index. |

Rows with a “freeze” note require a short definition decision record in the
metric test file or verification registry before coding. QuantStats contains
some heuristic return/price preparation; call its lower-level function with
preparation disabled where possible and never copy that heuristic into TAFlow.

## P4: period and closed-trade quality metrics

These metrics must name whether an observation is a period return, period P&L,
or closed trade. Do not annualize closed-trade statistics.

| Done | Canonical class | Accepted domains | Primary oracle | Exact contract note |
|:---:|---|---|---|---|
| [x] | `WinRate` | returns, period P&L, trades | QuantStats `win_rate` | Wins are strictly greater than zero; zero is breakeven. |
| [x] | `BreakevenRate` | returns, period P&L, trades | NumPy count reference | Exact zero count / valid observations. |
| [x] | `AverageWin` | returns, period P&L, trades | QuantStats `avg_win` | Mean strictly positive observation. |
| [x] | `AverageLoss` | returns, period P&L, trades | QuantStats `avg_loss` | Mean strictly negative observation; result stays negative. |
| [x] | `PayoffRatio` | returns, period P&L, trades | QuantStats `payoff_ratio` | Average win / absolute average loss. |
| [x] | `ProfitFactor` | returns, period P&L, trades | QuantStats `profit_factor`; vectorbt Trades cross-check | Gross positive sum / absolute gross negative sum. |
| [x] | `Expectancy` | period P&L, trades | QuantStats components | `P(win)*avg_win + P(loss)*avg_loss`; breakeven contributes zero. |
| [x] | `KellyCriterion` | returns, trades | QuantStats `kelly_criterion` | Historical binary Kelly fraction from win probability and payoff ratio, not an order-sizing action. |
| [x] | `LongestWinningStreak` | returns, period P&L, trades | QuantStats `consecutive_wins` | Strictly positive observations only. |
| [x] | `LongestLosingStreak` | returns, period P&L, trades | QuantStats `consecutive_losses` | Strictly negative observations only. |
| [x] | `GrossProfit` | period P&L, trades | NumPy plus QuantStats profit-factor numerator | Sum strictly positive P&L. |
| [x] | `GrossLoss` | period P&L, trades | NumPy plus QuantStats profit-factor denominator | Signed sum of strictly negative P&L. |
| [x] | `NetProfit` | period P&L, trades | NumPy/QuantStats composition | Gross profit plus signed gross loss. |
| [x] | `SystemQualityNumber` | trades | vectorbt `Trades.sqn` | `sqrt(n) * mean(trade_pnl) / sample_std(trade_pnl)`. |
| [x] | `CommonSenseRatio` | returns | QuantStats `common_sense_ratio` | Profit factor times tail ratio; lower priority because it is composite. |
| [x] | `CompositeProfitabilityConsistencyIndex` | returns, trades | QuantStats `cpc_index` | Descriptive TAFlow nomenclature for profit factor × win rate × payoff ratio; it does not claim a historical expansion of the external alias. |

Do not implement a class named `CpcIndex` until the abbreviation is resolved
to a complete descriptive canonical name, per repository naming rules.

## P5: advanced estimators, opt-in after the core release

| Done | Proposed class | Candidate oracle/specification | Reason for deferral |
|:---:|---|---|---|
| [ ] | `ProbabilisticSharpeRatio` | vectorbt returns metrics; Bailey and López de Prado | Requires skew, kurtosis, benchmark Sharpe, and a frozen small-sample formula. |
| [ ] | `DeflatedSharpeRatio` | vectorbt `deflated_sharpe_ratio`; Bailey and López de Prado | Requires number of trials and variance across tested Sharpe ratios; not a single-stream default. |
| [ ] | `ModifiedSharpeRatio` | PerformanceAnalytics `SharpeRatio.modified` | Requires a declared Cornish-Fisher estimator. |
| [ ] | `ParametricValueAtRisk` | PerformanceAnalytics/Riskfolio-Lib | Must name Gaussian vs Student-t and positive-loss vs signed-return convention. |
| [ ] | `ParametricExpectedShortfall` | PerformanceAnalytics/Riskfolio-Lib | Same distribution and sign decisions as parametric VaR. |
| [ ] | `ConditionalDrawdownAtRisk` | PerformanceAnalytics/Riskfolio-Lib | Exact drawdown-episode estimator and confidence convention required. |
| [ ] | `EntropicValueAtRisk` | Riskfolio-Lib | Numerical optimization and convergence contract required. |
| [ ] | `EffectiveNumberOfBets` | Riskfolio-Lib or a pinned paper implementation | Requires portfolio weights/covariance, outside the first one-series API. |
| [ ] | `Turnover` | vectorbt portfolio records | Requires weights/positions and timestamp semantics. |
| [ ] | `Exposure` | QuantStats/vectorbt | Requires position state, not return inference. |

## Explicitly out of scope for this package phase

- Rolling aligned metrics already owned by `taflow.indicators`.
- Order simulation, trade matching, slippage, commissions, and backtesting.
- Portfolio optimization, allocation, and risk-budget solvers.
- HTML tear sheets, plotting, data download, and broker integration.
- Implicit resampling or frequency inference from pandas indexes.
- Cross-sectional portfolio metrics requiring an asset-by-time matrix until a
  separate matrix input contract is designed.
- Money-weighted return/IRR and time-weighted return with external cash flows
  until dated cash-flow types and day-count conventions are defined.

These are valuable future domains, but mixing them into scalar metrics would
make input meaning and oracle comparison unreliable.
