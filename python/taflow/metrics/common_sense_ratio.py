"""Profit-factor and tail-ratio composite return metric."""

from __future__ import annotations

from typing import Any

from .._native.metrics import CommonSenseRatio as _Native
from ._input import as_metric_series


class CommonSenseRatio:
    """Compute profit factor multiplied by the 95th-to-5th percentile tail ratio.

    Profit factor is the sum of strictly positive returns divided by the
    absolute sum of strictly negative returns. Tail ratio is the absolute 95th
    linear percentile divided by the absolute 5th linear percentile. The
    independent oracle is QuantStats 0.0.81 ``common_sense_ratio``; correctness
    uses its ``profit_factor`` and ``tail_ratio`` components with preparation
    disabled to avoid QuantStats' price/return inference heuristic.

    Only decimal simple returns are accepted. Empty/all-missing warm-up, a zero
    gross-loss denominator, and a zero-magnitude lower percentile return
    ``None``. A loss-only sample validly returns ``0.0``. This deliberately
    applies TAFlow's general ratio rule to the composite: unlike standalone
    ``ProfitFactor``, a positive-only sample does not expose infinity. NaNs are
    omitted by default or rejected by ``nan_policy="raise"``; infinities and
    returns below -100% are rejected. ``append``, ``extend``, and ``reset`` are
    fluent. Rust retains O(n) observations for exact quantiles, lazily refreshes
    a sorted cache, and sorts once after each bulk extension. Native bulk work
    releases the GIL; Python performs no financial arithmetic.
    """

    def __init__(self, nan_policy: str = "omit") -> None:
        """Initialize an empty configured metric."""
        self._state = _Native(nan_policy)

    def from_returns(self, returns: Any, *, column: str | None = None) -> "CommonSenseRatio":
        """Append chronological returns observations and return this metric."""
        self._state.from_returns(as_metric_series(returns, column=column))
        return self

    def append(self, value: float) -> "CommonSenseRatio":
        """Append one simple return and return this metric."""
        self._state.append(float(value))
        return self

    def extend(
        self, returns: Any, *, column: str | None = None
    ) -> "CommonSenseRatio":
        """Append chronological simple returns and return this metric."""
        self._state.extend(as_metric_series(returns, column=column))
        return self

    @property
    def value(self) -> float | None:
        """Return the current exact composite ratio, or ``None`` if undefined."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the current exact scalar without replaying retained returns."""
        return self._state.compute()

    def reset(self) -> "CommonSenseRatio":
        """Clear retained returns, preserve missing policy, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of usable simple returns retained by Rust."""
        return len(self._state)


__all__ = ["CommonSenseRatio"]
