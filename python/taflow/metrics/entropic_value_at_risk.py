"""Empirical entropic value-at-risk metric."""

from __future__ import annotations

from typing import Any

from .._native.metrics import EntropicValueAtRisk as _Native
from ._input import as_metric_series


class EntropicValueAtRisk:
    """Compute empirical positive-loss Entropic Value at Risk.

    For loss observations ``L=-return``, TAFlow minimizes
    ``z * (log(mean(exp(L/z))) + log(1/cutoff))`` over ``z > 0``. The default
    lower-tail probability is ``cutoff=0.05``. This is Riskfolio-Lib 7.3's
    empirical ``EVaR_Hist`` convention: output is a positive loss magnitude
    when the selected tail represents losses, not a signed return quantile.
    The executable oracle independently minimizes that pinned Riskfolio-Lib
    objective with SciPy rather than relying on TAFlow's optimizer.

    TAFlow uses shifted log-sum-exp and deterministic bracketing/bisection of
    the monotone first-order condition. The relative interval tolerance is
    ``1e-12`` with at most 256 iterations. If the infimum occurs as ``z`` tends
    to zero, the exact worst loss is returned. Empty input is ``None``; one
    observation completes warm-up and returns its negated return.

    Exact empirical EVaR cannot be updated from fixed sufficient statistics:
    Rust retains O(n) normalized returns. ``append`` is amortized O(1), bulk
    ``extend`` is one native GIL-free loop, and the first ``compute`` after new
    input is O(n * iterations). Repeated unchanged ``compute`` calls are O(1)
    through a lazy cache. Semantic input methods accept simple returns, log
    returns, equity levels, and period P&L with positive initial capital.
    ``nan_policy`` is ``"omit"`` or ``"raise"``; infinities are rejected.
    """

    def __init__(self, cutoff: float = 0.05, nan_policy: str = "omit") -> None:
        """Initialize an empty configured metric."""
        self._state = _Native(float(cutoff), nan_policy)

    def from_returns(
        self, returns: Any, *, column: str | None = None
    ) -> "EntropicValueAtRisk":
        """Append chronological decimal simple returns and return this metric."""
        self._state.from_returns(as_metric_series(returns, column=column))
        return self

    def from_log_returns(
        self, log_returns: Any, *, column: str | None = None
    ) -> "EntropicValueAtRisk":
        """Append chronological log returns and return this metric."""
        self._state.from_log_returns(as_metric_series(log_returns, column=column))
        return self

    def from_equity(
        self, equity: Any, *, column: str | None = None
    ) -> "EntropicValueAtRisk":
        """Append chronological positive equity levels and return this metric."""
        self._state.from_equity(as_metric_series(equity, column=column))
        return self

    def from_pnl(
        self,
        pnl: Any,
        initial_capital: float,
        *,
        column: str | None = None,
    ) -> "EntropicValueAtRisk":
        """Append period P&L using required positive initial capital."""
        self._state.from_pnl(
            as_metric_series(pnl, column=column), float(initial_capital)
        )
        return self

    def append(self, value: float) -> "EntropicValueAtRisk":
        """Append one selected-domain observation and return this metric."""
        self._state.append(float(value))
        return self

    def extend(
        self, values: Any, *, column: str | None = None
    ) -> "EntropicValueAtRisk":
        """Append observations through one native loop and return this metric."""
        self._state.extend(as_metric_series(values, column=column))
        return self

    @property
    def value(self) -> float | None:
        """Return current positive-loss EVaR, or ``None`` when empty."""
        return self._state.value

    def compute(self) -> float | None:
        """Optimize after changed input, otherwise return the O(1) cached scalar."""
        return self._state.compute()

    def reset(self) -> "EntropicValueAtRisk":
        """Clear observations, retain capacity and settings, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the usable normalized-return count delegated to Rust."""
        return len(self._state)


__all__ = ["EntropicValueAtRisk"]
