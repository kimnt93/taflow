"""Probability that a strategy Sharpe ratio exceeds a benchmark."""

from __future__ import annotations

from typing import Any

from .._native.metrics import ProbabilisticSharpeRatio as _Native
from ._input import as_metric_series


class ProbabilisticSharpeRatio:
    """Estimate the probability that Sharpe exceeds a benchmark Sharpe.

    TAFlow freezes the Bailey and Lopez de Prado probabilistic-Sharpe formula
    used by vectorbt 0.28.5's deflated-Sharpe kernel, with its estimated maximum
    Sharpe replaced by ``annual_benchmark_sharpe_ratio``. The pinned vectorbt
    source commit is ``993ceca7116fc8e55f4cd3a36fe43d83dab62b27`` and is
    the independent oracle.
    The estimated Sharpe uses sample standard deviation; skewness and Pearson
    kurtosis use SciPy-compatible bias-corrected sample estimators
    (``bias=False``). Both estimated and benchmark Sharpe are converted to
    per-period scale before evaluation. Warm-up requires four usable returns.

    ``annual_risk_free_rate`` is an annual effective rate converted by Rust to
    a per-period return using explicit ``periods_per_year``. Inputs may be
    decimal simple returns, log returns, positive equity levels, or
    non-cumulative period P&L with positive initial equity. NaNs are omitted by
    default or rejected with ``nan_policy="raise"``; infinities and simple
    returns below -100% are rejected. Rust owns conversion and O(1) online
    moments through fourth order. ``append`` is allocation-free, ``compute`` is
    O(1), mutating methods are fluent, and bulk work releases the GIL.
    """

    def __init__(self) -> None:
        """Reject ambiguous construction; use a semantic ``from_*`` factory."""
        raise TypeError(
            "use ProbabilisticSharpeRatio.from_returns/from_equity/"
            "from_pnl/from_log_returns"
        )

    @classmethod
    def _create(
        cls,
        values: Any,
        input_mode: str,
        *,
        periods_per_year: float,
        annual_risk_free_rate: float,
        annual_benchmark_sharpe_ratio: float,
        initial_equity: float | None,
        nan_policy: str,
        column: str | None,
    ) -> "ProbabilisticSharpeRatio":
        state = cls.__new__(cls)
        state._state = _Native(
            input_mode,
            float(periods_per_year),
            float(annual_risk_free_rate),
            float(annual_benchmark_sharpe_ratio),
            initial_equity,
            nan_policy,
        )
        return state.extend(values, column=column)

    @classmethod
    def from_returns(
        cls,
        returns: Any,
        *,
        periods_per_year: float = 252.0,
        annual_risk_free_rate: float = 0.0,
        annual_benchmark_sharpe_ratio: float = 0.0,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "ProbabilisticSharpeRatio":
        """Construct from chronological decimal simple returns."""
        return cls._create(
            returns, "returns", periods_per_year=periods_per_year,
            annual_risk_free_rate=annual_risk_free_rate,
            annual_benchmark_sharpe_ratio=annual_benchmark_sharpe_ratio,
            initial_equity=None, nan_policy=nan_policy, column=column,
        )

    @classmethod
    def from_log_returns(
        cls,
        log_returns: Any,
        *,
        periods_per_year: float = 252.0,
        annual_risk_free_rate: float = 0.0,
        annual_benchmark_sharpe_ratio: float = 0.0,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "ProbabilisticSharpeRatio":
        """Construct from chronological log returns converted by Rust."""
        return cls._create(
            log_returns, "log_returns", periods_per_year=periods_per_year,
            annual_risk_free_rate=annual_risk_free_rate,
            annual_benchmark_sharpe_ratio=annual_benchmark_sharpe_ratio,
            initial_equity=None, nan_policy=nan_policy, column=column,
        )

    @classmethod
    def from_equity(
        cls,
        equity: Any,
        *,
        periods_per_year: float = 252.0,
        annual_risk_free_rate: float = 0.0,
        annual_benchmark_sharpe_ratio: float = 0.0,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "ProbabilisticSharpeRatio":
        """Construct from positive chronological equity or adjusted-price levels."""
        return cls._create(
            equity, "equity", periods_per_year=periods_per_year,
            annual_risk_free_rate=annual_risk_free_rate,
            annual_benchmark_sharpe_ratio=annual_benchmark_sharpe_ratio,
            initial_equity=None, nan_policy=nan_policy, column=column,
        )

    @classmethod
    def from_pnl(
        cls,
        pnl: Any,
        *,
        initial_equity: float,
        periods_per_year: float = 252.0,
        annual_risk_free_rate: float = 0.0,
        annual_benchmark_sharpe_ratio: float = 0.0,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "ProbabilisticSharpeRatio":
        """Construct from non-cumulative period P&L and positive initial equity."""
        return cls._create(
            pnl, "pnl", periods_per_year=periods_per_year,
            annual_risk_free_rate=annual_risk_free_rate,
            annual_benchmark_sharpe_ratio=annual_benchmark_sharpe_ratio,
            initial_equity=float(initial_equity), nan_policy=nan_policy,
            column=column,
        )

    def append(self, value: float) -> "ProbabilisticSharpeRatio":
        """Append one value in the selected domain and return this metric."""
        self._state.append(float(value))
        return self

    def extend(
        self, values: Any, *, column: str | None = None
    ) -> "ProbabilisticSharpeRatio":
        """Append chronological values in the selected domain and return self."""
        self._state.extend(as_metric_series(values, column=column))
        return self

    @property
    def value(self) -> float | None:
        """Return the current exceedance probability, or ``None`` if undefined."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the cached O(1) result without replaying prior input."""
        return self._state.compute()

    def reset(self) -> "ProbabilisticSharpeRatio":
        """Clear observations, preserve configuration, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of usable normalized returns processed by Rust."""
        return len(self._state)


__all__ = ["ProbabilisticSharpeRatio"]
