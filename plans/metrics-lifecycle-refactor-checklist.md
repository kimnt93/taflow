# Metrics lifecycle refactor checklist

This checklist tracks the repository-wide migration from class-level input
calls to configuration-only construction followed by instance data
ingestion.

## Required contract

- Rust construction is `Metric::new(configuration...)`; Python construction is
  `Metric(configuration...)`.
- Constructors accept configuration only. They do not accept a series, an input
  domain, or P&L initial capital.
- `from_returns`, `from_log_returns`, `from_equity`, `from_pnl`,
  `from_trades`, `from_positions`, and weight-specific `from_*` methods are
  instance mutators and return the same mutable Rust/Python object.
- A `from_*` method accepts only its required series inputs. P&L-to-return
  conversion additionally accepts `initial_capital`.
- The first successful `from_*` call binds the append domain. Later `append`
  and `extend` calls use that domain. Calling a different `from_*` domain on a
  non-empty metric is rejected; `reset` clears observations while preserving
  the selected domain and metric configuration.
- Class-level calls such as `SharpeRatio.from_returns(returns, ...)` are
  removed rather than retained as compatibility wrappers.
- `MetricPipeline()` starts empty. `add(name, metric)` adds a configured metric
  instance. Pipeline `from_*`, `append`, `extend`, `compute`, `value`, `reset`,
  and `len` follow the same lifecycle, and results use the caller-provided
  names.
- Rust core, PyO3, Python adapters, tests, verification scripts, documentation,
  examples, and exports must agree before a row is complete.
- Benchmarks are not run during this migration unless explicitly authorized.

## Infrastructure and pipeline

- [x] Unbound/bind-on-`from_*` single-input state
- [x] Unbound/bind-on-`from_*` paired-input state
- [x] Rust `MetricPipeline` add/configure/ingest lifecycle
- [x] Python `MetricPipeline` add/configure/ingest lifecycle
- [x] Pipeline Rust and Python tests
- [x] Shared Python input helpers and interface verification

## Metrics

- [x] Alpha
- [x] AnnualizedReturn
- [x] AnnualizedVolatility
- [x] AverageDrawdown
- [x] AverageLoss
- [x] AverageWin
- [x] Beta
- [x] BreakevenRate
- [x] CalmarRatio
- [x] CaptureRatio
- [x] CoefficientOfDetermination
- [x] CommonSenseRatio
- [x] CompositeProfitabilityConsistencyIndex
- [x] ConditionalDrawdownAtRisk
- [x] DeflatedSharpeRatio
- [x] DownMarketCaptureRatio
- [x] DownsideDeviation
- [x] EffectiveNumberOfBets
- [x] EntropicValueAtRisk
- [x] Expectancy
- [x] Exposure
- [x] GainToPainRatio
- [x] GrossLoss
- [x] GrossProfit
- [x] HistoricalExpectedShortfall
- [x] HistoricalValueAtRisk
- [x] InformationRatio
- [x] KellyCriterion
- [x] LongestLosingStreak
- [x] LongestWinningStreak
- [x] MaximumDrawdown
- [x] MaximumDrawdownDuration
- [x] ModifiedSharpeRatio
- [x] NetProfit
- [x] OmegaRatio
- [x] PainIndex
- [x] PainRatio
- [x] ParametricExpectedShortfall
- [x] ParametricValueAtRisk
- [x] PayoffRatio
- [x] ProbabilisticSharpeRatio
- [x] ProfitFactor
- [x] RecoveryFactor
- [x] SharpeRatio
- [x] SortinoRatio
- [x] StabilityOfTimeSeries
- [x] SystemQualityNumber
- [x] TailRatio
- [x] TotalReturn
- [x] TrackingError
- [x] TreynorRatio
- [x] Turnover
- [x] UlcerIndex
- [x] UlcerPerformanceIndex
- [x] UpDownCaptureRatio
- [x] UpMarketCaptureRatio
- [x] WinRate

## Final gates

- [x] No class-level Python `from_*` calls remain
- [x] No metric constructor accepts an input series/domain
- [x] All Rust metric and pipeline tests pass
- [x] All Python metric and pipeline tests pass
- [x] Metrics correctness and interface verification pass
- [x] `cargo test --workspace`
- [x] `uv run pytest -q`
- [x] `make check`
- [x] `uv run python scripts/verification/interfaces.py`
- [x] `cargo fmt --all --check`
- [x] `git diff --check` and full diff audit
