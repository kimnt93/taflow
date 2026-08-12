"""Gaussian parametric value-at-risk metric."""

from __future__ import annotations

from typing import Any

from .._native.metrics import ParametricValueAtRisk as _Native
from ._input import as_metric_series


class ParametricValueAtRisk:
    """Estimate the signed lower-tail quantile of Gaussian simple returns.

    The definition is ``sample_mean + normal_ppf(cutoff) * sample_std``, with
    one degree of freedom and default lower-tail probability ``cutoff=0.05``.
    This freezes the Gaussian distribution and signed-return convention: a
    loss-side result is negative, not converted to a positive loss magnitude.
    The independent executable oracle is SciPy ``stats.norm.ppf`` with NumPy
    sample moments, corresponding to the Gaussian PerformanceAnalytics and
    Riskfolio-Lib convention. Warm-up requires two usable returns; a constant
    sample returns its mean, while empty and one-return states yield ``None``.

    Inputs may be decimal simple returns, log returns, positive equity levels,
    or non-cumulative period P&L with positive initial capital. Rust performs
    conversion, missing-value handling, and O(1)-memory online moment updates.
    ``append`` is O(1), ``compute`` is O(1), and bulk ``extend`` executes in one
    native loop with the Python GIL released. ``nan_policy`` is ``"omit"`` or
    ``"raise"``; infinities are rejected. Mutating lifecycle methods are fluent.
    """

    def __init__(self, cutoff: float = 0.05, nan_policy: str = "omit") -> None:
        """Initialize an empty configured metric."""
        self._state = _Native(float(cutoff), nan_policy)

    def from_returns(
        self, returns: Any, *, column: str | None = None
    ) -> "ParametricValueAtRisk":
        """Append chronological decimal simple returns and return this metric."""
        self._state.from_returns(as_metric_series(returns, column=column))
        return self

    def from_log_returns(
        self, log_returns: Any, *, column: str | None = None
    ) -> "ParametricValueAtRisk":
        """Append chronological log returns and return this metric."""
        self._state.from_log_returns(as_metric_series(log_returns, column=column))
        return self

    def from_equity(
        self, equity: Any, *, column: str | None = None
    ) -> "ParametricValueAtRisk":
        """Append chronological positive equity levels and return this metric."""
        self._state.from_equity(as_metric_series(equity, column=column))
        return self

    def from_pnl(
        self,
        pnl: Any,
        initial_capital: float,
        *,
        column: str | None = None,
    ) -> "ParametricValueAtRisk":
        """Append period P&L using required positive initial capital."""
        self._state.from_pnl(
            as_metric_series(pnl, column=column), float(initial_capital)
        )
        return self

    def append(self, value: float) -> "ParametricValueAtRisk":
        """Append one value in the selected domain and return this metric."""
        self._state.append(float(value))
        return self

    def extend(
        self, values: Any, *, column: str | None = None
    ) -> "ParametricValueAtRisk":
        """Append a chronological series and return this metric."""
        self._state.extend(as_metric_series(values, column=column))
        return self

    @property
    def value(self) -> float | None:
        """Return the signed Gaussian quantile, or ``None`` during warm-up."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the current scalar in O(1) without replaying input."""
        return self._state.compute()

    def reset(self) -> "ParametricValueAtRisk":
        """Clear observations, preserve configuration, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the usable normalized-return count delegated to Rust."""
        return len(self._state)


__all__ = ["ParametricValueAtRisk"]
