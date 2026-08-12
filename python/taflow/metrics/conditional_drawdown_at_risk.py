"""Exact discrete-episode conditional drawdown-at-risk metric."""

from __future__ import annotations

from typing import Any

from .._native.metrics import ConditionalDrawdownAtRisk as _Native
from ._input import as_metric_series


class ConditionalDrawdownAtRisk:
    """Compute expected shortfall across discrete drawdown-episode troughs.

    The independent oracle and contract is PerformanceAnalytics 2.1.0 ``CDD`` with
    ``method="discrete"``, geometric compounding, and positive/inverted output.
    Wealth starts at a phantom one. The exact distribution contains the signed
    trough of every contiguous negative or recovered/non-negative drawdown
    segment produced by ``findDrawdowns``. The current unrecovered segment is
    included exactly once. At ``1 - confidence``, the estimator uses R's
    type-7 interpolated quantile and averages all signed troughs at or below the
    boundary, including ties, before returning the positive magnitude.
    ``confidence`` defaults to ``0.95`` and must be strictly between zero and
    one. During warm-up, empty and all-omitted states return ``None``; a
    non-empty path without loss returns ``0.0``.

    This discrete episode contract intentionally differs from Riskfolio-Lib
    7.3.0 ``CDaR_Rel``, which uses the continuous per-observation drawdown path
    and fractional tail weighting. The PerformanceAnalytics source is pinned
    to the CRAN 2.1.0 tarball SHA-256
    ``fc801d39382818cd3a7052326b45d078302aef4d290c85dab83498ed4516d58d``.

    Inputs may be decimal simple returns, log returns, positive equity levels,
    or non-cumulative period P&L. The P&L factory requires positive initial
    equity and Rust performs causal capital conversion. The first equity level
    establishes a baseline and does not increment metric length.
    ``nan_policy`` is ``"omit"`` or ``"raise"``; infinities and simple returns
    below -100% are rejected. Mutating methods are fluent, bulk conversion and
    recurrence execute natively with the GIL released, and repeated unchanged
    ``compute`` calls are O(1). Exact episode order statistics retain O(e)
    memory and a dirty computation is O(e log e), where e is the segment count.
    """

    def __init__(self) -> None:
        """Reject ambiguous construction; use a semantic ``from_*`` factory."""
        raise TypeError(
            "use ConditionalDrawdownAtRisk.from_returns/from_equity/from_pnl/"
            "from_log_returns"
        )

    @classmethod
    def _create(
        cls,
        values: Any,
        input_mode: str,
        *,
        confidence: float = 0.95,
        initial_equity: float | None = None,
        nan_policy: str = "omit",
    ) -> "ConditionalDrawdownAtRisk":
        state = cls.__new__(cls)
        state._state = _Native(
            input_mode, float(confidence), initial_equity, nan_policy
        )
        return state.extend(values)

    @classmethod
    def from_returns(
        cls,
        returns: Any,
        *,
        confidence: float = 0.95,
        nan_policy: str = "omit",
    ) -> "ConditionalDrawdownAtRisk":
        """Construct from chronological decimal simple returns."""
        return cls._create(
            returns, "returns", confidence=confidence, nan_policy=nan_policy
        )

    @classmethod
    def from_log_returns(
        cls,
        log_returns: Any,
        *,
        confidence: float = 0.95,
        nan_policy: str = "omit",
    ) -> "ConditionalDrawdownAtRisk":
        """Construct from chronological log returns converted by Rust."""
        return cls._create(
            log_returns,
            "log_returns",
            confidence=confidence,
            nan_policy=nan_policy,
        )

    @classmethod
    def from_equity(
        cls,
        equity: Any,
        *,
        confidence: float = 0.95,
        nan_policy: str = "omit",
    ) -> "ConditionalDrawdownAtRisk":
        """Construct from positive chronological equity or adjusted-price levels."""
        return cls._create(
            equity, "equity", confidence=confidence, nan_policy=nan_policy
        )

    @classmethod
    def from_pnl(
        cls,
        pnl: Any,
        *,
        initial_equity: float,
        confidence: float = 0.95,
        nan_policy: str = "omit",
    ) -> "ConditionalDrawdownAtRisk":
        """Construct from non-cumulative period P&L and positive initial equity."""
        return cls._create(
            pnl,
            "pnl",
            confidence=confidence,
            initial_equity=float(initial_equity),
            nan_policy=nan_policy,
        )

    def append(self, value: float) -> "ConditionalDrawdownAtRisk":
        """Append one value in the factory-selected domain and return this metric."""
        self._state.append(float(value))
        return self

    def extend(self, values: Any) -> "ConditionalDrawdownAtRisk":
        """Append a chronological series in the selected domain and return this metric."""
        self._state.extend(as_metric_series(values))
        return self

    @property
    def value(self) -> float | None:
        """Return positive discrete-episode CDaR, or ``None`` while empty."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the cached exact scalar without replaying processed input."""
        return self._state.compute()

    def reset(self) -> "ConditionalDrawdownAtRisk":
        """Clear episodes, preserve configuration/capacity, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of usable normalized returns processed by Rust."""
        return len(self._state)


__all__ = ["ConditionalDrawdownAtRisk"]
