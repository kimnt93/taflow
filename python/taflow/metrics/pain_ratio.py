"""Annualized-return-to-mean-drawdown Pain ratio metric."""

from __future__ import annotations

from typing import Any

from .._native.metrics import PainRatio as _Native
from ._input import as_metric_series


class PainRatio:
    """Compute geometric annualized excess return divided by Pain Index.

    TAFlow follows PerformanceAnalytics 2.1.0 ``PainRatio`` and ``PainIndex``:
    ``(geometric CAGR - annual_risk_free_rate) / mean(abs(drawdown))``. A
    phantom wealth level of one precedes the first normalized return, but only
    real usable observations enter the Pain Index divisor. The source contract
    is pinned to the CRAN 2.1.0 tarball with SHA-256
    ``fc801d39382818cd3a7052326b45d078302aef4d290c85dab83498ed4516d58d``.
    No R runtime is available in the test environment, so independent tests
    translate this pinned source rather than claiming an executable oracle
    match.

    ``periods_per_year`` defaults to 252 and is explicit rather than inferred
    from timestamps. ``annual_risk_free_rate`` is an annual effective rate and
    is subtracted after annualizing the portfolio return, exactly where the
    source uses ``Rf``. Warm-up ends with the first usable observation having
    positive drawdown pain. Empty and zero-pain paths return ``None``.

    Inputs may be decimal simple returns, log returns, positive equity levels,
    or non-cumulative period P&L with positive initial capital. Rust performs
    all conversions and O(1)-memory arithmetic. NaNs are omitted by default or
    rejected with ``nan_policy="raise"``; infinities and returns below -100%
    are rejected. Mutating lifecycle methods are fluent and bulk execution
    releases the GIL.
    """

    def __init__(self, periods_per_year: float = 252.0, annual_risk_free_rate: float = 0.0, nan_policy: str = "omit") -> None:
        """Initialize an empty configured metric."""
        self._state = _Native(float(periods_per_year), float(annual_risk_free_rate), nan_policy)

    def from_returns(self, returns: Any, *, column: str | None = None) -> "PainRatio":
        """Append chronological returns and return this metric."""
        self._state.from_returns(as_metric_series(returns, column=column))
        return self

    def from_log_returns(self, log_returns: Any, *, column: str | None = None) -> "PainRatio":
        """Append chronological log returns and return this metric."""
        self._state.from_log_returns(as_metric_series(log_returns, column=column))
        return self

    def from_equity(self, equity: Any, *, column: str | None = None) -> "PainRatio":
        """Append chronological equity and return this metric."""
        self._state.from_equity(as_metric_series(equity, column=column))
        return self

    def from_pnl(self, pnl: Any, initial_capital: float, *, column: str | None = None) -> "PainRatio":
        """Append period P&L using required positive initial capital."""
        self._state.from_pnl(as_metric_series(pnl, column=column), float(initial_capital))
        return self

    def append(self, value: float) -> "PainRatio":
        """Append one value in the input method-selected domain and return this metric."""
        self._state.append(float(value))
        return self

    def extend(
        self, values: Any, *, column: str | None = None
    ) -> "PainRatio":
        """Append a chronological series in the selected domain and return this metric."""
        self._state.extend(as_metric_series(values, column=column))
        return self

    @property
    def value(self) -> float | None:
        """Return the Pain ratio, or ``None`` until positive pain exists."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the current scalar without replaying processed input."""
        return self._state.compute()

    def reset(self) -> "PainRatio":
        """Clear observations, preserve configuration, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of usable normalized returns processed by Rust."""
        return len(self._state)


__all__ = ["PainRatio"]
