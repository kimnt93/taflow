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
    or non-cumulative period P&L with positive initial equity. Rust performs
    conversion, missing-value handling, and O(1)-memory online moment updates.
    ``append`` is O(1), ``compute`` is O(1), and bulk ``extend`` executes in one
    native loop with the Python GIL released. ``nan_policy`` is ``"omit"`` or
    ``"raise"``; infinities are rejected. Mutating lifecycle methods are fluent.
    """

    def __init__(self) -> None:
        """Reject ambiguous construction; use a semantic ``from_*`` factory."""
        raise TypeError(
            "use ParametricValueAtRisk.from_returns/from_equity/from_pnl/"
            "from_log_returns"
        )

    @classmethod
    def _create(
        cls,
        values: Any,
        input_mode: str,
        *,
        cutoff: float = 0.05,
        initial_equity: float | None = None,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "ParametricValueAtRisk":
        state = cls.__new__(cls)
        state._state = _Native(input_mode, float(cutoff), initial_equity, nan_policy)
        return state.extend(values, column=column)

    @classmethod
    def from_returns(
        cls,
        returns: Any,
        *,
        cutoff: float = 0.05,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "ParametricValueAtRisk":
        """Construct from chronological decimal simple returns."""
        return cls._create(
            returns, "returns", cutoff=cutoff, nan_policy=nan_policy, column=column
        )

    @classmethod
    def from_log_returns(
        cls,
        log_returns: Any,
        *,
        cutoff: float = 0.05,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "ParametricValueAtRisk":
        """Construct from chronological log returns converted by Rust."""
        return cls._create(
            log_returns,
            "log_returns",
            cutoff=cutoff,
            nan_policy=nan_policy,
            column=column,
        )

    @classmethod
    def from_equity(
        cls,
        equity: Any,
        *,
        cutoff: float = 0.05,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "ParametricValueAtRisk":
        """Construct from positive equity or adjusted-price levels."""
        return cls._create(
            equity, "equity", cutoff=cutoff, nan_policy=nan_policy, column=column
        )

    @classmethod
    def from_pnl(
        cls,
        pnl: Any,
        *,
        initial_equity: float,
        cutoff: float = 0.05,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "ParametricValueAtRisk":
        """Construct from period P&L and required positive initial equity."""
        return cls._create(
            pnl,
            "pnl",
            cutoff=cutoff,
            initial_equity=float(initial_equity),
            nan_policy=nan_policy,
            column=column,
        )

    def append(self, value: float) -> "ParametricValueAtRisk":
        """Append one value in the factory-selected domain and return this metric."""
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
