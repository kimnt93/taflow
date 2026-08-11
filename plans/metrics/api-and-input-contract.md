# Metrics API and input contract

## Public class lifecycle

Each scalar metric has the same lifecycle:

```python
metric = SharpeRatio.from_returns([], periods_per_year=252.0)
metric.append(0.01).extend([-0.02, 0.015])
latest: float | None = metric.value
result: float | None = metric.compute()
count: int = len(metric)
metric.reset()
```

- Factories, not a polymorphic positional constructor, select the input
  domain.
- `append`, `extend`, and `reset` mutate and return the concrete class.
- `value` and `compute()` return the same latest scalar. `compute()` must not
  replay prior input or mutate state. An expensive exact metric may lazily
  refresh a cached value when dirty.
- `None` means insufficient data or a mathematically undefined result. Valid
  finite results, signed zero, and valid infinity must not be conflated with
  warm-up.
- `__len__` is the number of usable observations in the metric domain. An
  equity series of length `n` produces `n - 1` returns, so its metric length is
  `n - 1`. The first streamed equity value establishes the baseline and does
  not increment the metric length.
- `reset()` preserves configuration, selected input mode, and the original
  `initial_equity` when applicable, but clears all processed observations and
  conversion state.

## Supported input factories

### `from_returns`

Input is a chronological one-dimensional series of decimal simple returns:
`0.01` means +1%. Values are non-cumulative. This is the canonical metric
domain and the primary external-oracle input.

### `from_log_returns`

Input is chronological log return `log(V_t / V_(t-1))`. Rust converts each
value with `expm1`. This factory is useful but should be delivered after
`from_returns`, `from_equity`, and `from_pnl` are stable.

### `from_equity`

Input is a chronological equity, NAV, account-value, or price-level series.
Rust derives:

```text
r_t = equity_t / equity_(t-1) - 1
```

All levels must be finite and strictly positive. The factory deliberately
does not infer or adjust dividends, splits, deposits, withdrawals, fees, or
currency conversion. The caller supplies a total-return-adjusted series when
those semantics are required.

`from_prices` may be provided as a documented semantic factory that enters
the exact same native level-input mode as `from_equity`; it must not implement
a second conversion path. It is lower priority than the three core factories.

### `from_pnl`

Input is non-cumulative P&L per observation period. For a return/path metric,
the factory requires a positive `initial_equity`; Rust derives returns and
advances equity in chronological order:

```text
equity_before_0 = initial_equity
r_t = pnl_t / equity_before_t
equity_after_t = equity_before_t + pnl_t
```

External deposits and withdrawals are unsupported in phase 1. P&L must already
exclude them. If equity reaches zero, `-100%` is a valid final return, but no
later P&L can be converted; reject such continuation. Equity below zero is an
error for this conversion contract.

For a P&L-native metric such as `GrossProfit`, `NetProfit`, or trade/period
`ProfitFactor`, `from_pnl` consumes the raw period P&L values and does not
accept `initial_equity`. Requiring and then ignoring a capital value would be
misleading. Each metric docstring must state whether its P&L factory converts
to returns or consumes raw P&L.

### `from_trades`

Only trade-compatible metrics expose this factory. Input is the realized P&L
of each closed trade, not bar P&L and not mark-to-market equity changes. It is
the correct input for `ProfitFactor`, `PayoffRatio`, `Expectancy`, streaks,
`KellyCriterion`, and `SystemQualityNumber`. Annualized return metrics must not
accept trade P&L because irregular trade arrival is not an annualization
frequency.

## Benchmark inputs

Benchmark-relative classes use paired, explicit factories:

```python
InformationRatio.from_returns(
    returns,
    benchmark_returns,
    periods_per_year=252.0,
)

Beta.from_equity(equity, benchmark_equity)
```

Both series are ordered primary first, benchmark second everywhere. Reject
length mismatch before mutating native state. Missing values are removed
pairwise; never drop a value from only one side. Do not align pandas indexes
implicitly in phase 1 because TAFlow's container adapter deliberately treats
all inputs as ordered series. Users align labels before calling TAFlow.

## Container conversion

Accepted Python inputs follow the existing `as_float64_series` behavior:
NumPy, Python lists/tuples, pandas Series, Polars Series, Arrow-compatible
series when supported, and a one-column dataframe or an explicit selected
column. The adapter creates one contiguous `float64` array and makes one native
call. Python must not loop, derive returns, repair NaNs, compound values, or
calculate a metric.

Indexes and timestamps are not used to infer frequency. Require explicit
`periods_per_year`; the default is `252.0`. Later calendar-aware metrics should
be a separate API with explicit timestamps and day-count convention.

## Annualization and target rates

Use descriptive parameters:

- `periods_per_year: float = 252.0`
- `annual_risk_free_rate: float = 0.0`
- `annual_required_return: float = 0.0`
- `degrees_of_freedom: int = 1` where dispersion is configurable

Rates are decimal annual effective rates. Convert an annual rate to a
per-period rate in Rust:

```text
period_rate = expm1(log1p(annual_rate) / periods_per_year)
```

Require `periods_per_year > 0` and annual rates greater than `-1`. Oracle
adapters must convert these values to the oracle's convention rather than
quietly changing TAFlow's public contract. For example, Empyrical Sharpe takes
a per-period `risk_free`, while QuantStats documents its `rf` as annual.

Do not infer 252 versus 365 from the values or index. Crypto, hourly, monthly,
and irregular data must supply the factor they intend.

## Missing and invalid data

Default `nan_policy="omit"` so users can pass an aligned indicator output whose
first observations are warm-up NaNs. Supported policies are:

- `"omit"`: ignore NaNs; for paired inputs, omit the whole pair.
- `"raise"`: reject the first NaN before mutation.
- `"propagate"`: accept NaN and make the current result undefined until reset;
  defer this mode unless a clear use case appears.

Always reject positive/negative infinity. Record the count of valid metric
observations, not the raw container length. Oracle tests must apply the same
policy explicitly.

For compounding metrics, a simple return below `-1` is invalid. A return equal
to `-1` represents total loss; annualized growth thereafter is defined only if
there is no continuation requiring a positive capital base.

## Sign and estimator conventions

- `MaximumDrawdown` returns a non-positive fraction, matching Empyrical and the
  existing TAFlow drawdown series. Calmar and recovery metrics divide by its
  absolute magnitude.
- `HistoricalValueAtRisk` returns the signed lower-tail return quantile. At a
  5% cutoff it is normally negative. Do not silently flip it to a positive
  loss amount.
- `HistoricalExpectedShortfall` returns the signed mean of the selected lower
  tail and is normally negative.
- Sample standard deviation uses `degrees_of_freedom=1` by default. Downside
  deviation follows the lower-partial-moment definition: square all shortfalls
  after clipping positive differences to zero, average over all valid
  observations, then take the square root.
- Historical VaR uses NumPy/Empyrical's linear percentile convention.
  Historical expected shortfall follows Empyrical 0.5.12 exactly: select the
  lowest `floor((n - 1) * cutoff) + 1` observations and average them.
- Annualized return is geometric CAGR:
  `(product(1 + r)) ** (periods_per_year / n) - 1`.
- Annualized volatility is sample standard deviation times
  `sqrt(periods_per_year)` in the first release. Do not expose Empyrical's
  Levy-alpha generalization until it is a separately specified metric.

## Undefined and edge results

Use `None`, not an arbitrary zero, for an empty sample, insufficient degrees
of freedom, zero benchmark variance for beta, zero drawdown for Calmar, or a
zero denominator where the ratio has no unique value.

The exception is an economically meaningful unbounded ratio:

- `ProfitFactor` is `+inf` when gross profit is positive and gross loss is
  zero; it is `None` when both are zero.
- Other zero-risk positive-return ratios remain `None` in phase 1. This avoids
  conflicting `inf`, zero, and NaN behavior across external libraries. The
  verifier normalizes each oracle's sentinel to this contract.

Every metric file must document its minimum observations and edge matrix.

## Metric-specific factory eligibility

| Metric family | returns | equity/price | period P&L | closed trades |
|---|:---:|:---:|:---:|:---:|
| Return/risk/benchmark ratios | yes | yes | yes | no |
| Drawdown/path metrics | yes | yes | yes | no |
| Tail return distribution | yes | yes | yes | no |
| Period hit-rate/quality | yes | yes | yes (raw P&L) | optional, semantics named |
| Trade quality and streaks | no by default | no | no | yes |
| Absolute gross/net P&L | no | no | yes (raw P&L) | yes |

Do not expose a factory just because a numeric conversion is possible. The
result must have a defensible financial meaning.
