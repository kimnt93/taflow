"""Average one-way portfolio-weight turnover metric."""

from __future__ import annotations

from typing import Any

from .._native.metrics import Turnover as _Native
from ._input import as_metric_series


class Turnover:
    """Compute mean one-way turnover from chronological risky-asset weights.

    For a single risky asset with implicit cash, each transition is
    ``abs(weight[t] - weight[t - 1])`` and the result is the arithmetic mean
    across valid transitions. This explicit weight contract avoids inferring
    trades or rebalance timestamps from returns. Levered and short weights are
    accepted; all values must be finite. NaNs are omitted by default, which
    bridges the surrounding valid weights, or rejected with
    ``nan_policy="raise"``. Warm-up requires two valid weights.

    Rust owns allocation-free O(1) append and compute state; native bulk work
    releases the GIL. The independent executable oracle is NumPy ``diff`` and
    ``mean(abs(...))``. Multi-asset turnover remains a separate future matrix
    contract rather than being ambiguously flattened here.
    """

    def __init__(self) -> None:
        """Reject ambiguous construction; use :meth:`from_weights`."""
        raise TypeError("use Turnover.from_weights")

    @classmethod
    def from_weights(
        cls,
        weights: Any,
        *,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "Turnover":
        """Construct from chronological risky-asset portfolio weights."""
        state = cls.__new__(cls)
        state._state = _Native(nan_policy)
        return state.extend(weights, column=column)

    def append(self, weight: float) -> "Turnover":
        """Append one portfolio weight and return this metric."""
        self._state.append(float(weight))
        return self

    def extend(
        self, weights: Any, *, column: str | None = None
    ) -> "Turnover":
        """Append chronological portfolio weights and return this metric."""
        self._state.extend(as_metric_series(weights, column=column))
        return self

    @property
    def value(self) -> float | None:
        """Return mean turnover, or ``None`` during warm-up."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the current O(1) scalar without replaying weights."""
        return self._state.compute()

    def reset(self) -> "Turnover":
        """Clear weights, preserve missing-value policy, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the valid-weight count delegated to Rust."""
        return len(self._state)


__all__ = ["Turnover"]
