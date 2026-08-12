"""Non-zero observation exposure metric."""

from __future__ import annotations

from typing import Any

from .._native.metrics import Exposure as _Native
from ._input import as_metric_series


class Exposure:
    """Compute the share of usable periods with non-zero activity.

    ``from_positions`` is the preferred semantic factory: each observation is
    an explicit position, portfolio weight, or exposure-state scalar, exact
    zero means cash/flat, and any finite non-zero value means exposed.
    ``from_returns`` implements QuantStats' weaker proxy contract explicitly:
    a non-zero decimal simple period return is treated as exposed and an exact
    zero as unexposed. It does not claim to reconstruct actual positions; an
    invested asset may genuinely have a zero return.

    The independent oracle is QuantStats 0.0.81 ``exposure`` with return
    preparation disabled. TAFlow deliberately retains its ceiling quirk:
    ``ceil(raw_fraction * 100) / 100`` rounds any fractional result upward to
    the next percentage point. Before that formula, TAFlow applies its package
    missing-value contract, so omitted NaNs are absent from both numerator and
    denominator. During warm-up, an empty or all-omitted stream returns
    ``None`` instead of the oracle's division error. Exact negative zero is
    unexposed; infinities are rejected; return-proxy values below -100% are
    invalid, while explicit positions may be negative or leveraged.

    ``append``, ``extend``, and ``reset`` are fluent. Bulk work releases the
    GIL, while Rust owns the O(1)-memory counters. ``append`` and unchanged
    ``compute`` are O(1), and bulk ``extend`` is one native linear pass.
    """

    def __init__(self) -> None:
        """Reject ambiguous construction; use a semantic ``from_*`` factory."""
        raise TypeError("use Exposure.from_positions/from_returns")

    @classmethod
    def _create(
        cls,
        values: Any,
        input_mode: str,
        *,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "Exposure":
        state = cls.__new__(cls)
        state._state = _Native(input_mode, nan_policy=nan_policy)
        return state.extend(values, column=column)

    @classmethod
    def from_positions(
        cls,
        positions: Any,
        *,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "Exposure":
        """Construct from explicit chronological position or exposure states."""
        return cls._create(
            positions, "positions", nan_policy=nan_policy, column=column
        )

    @classmethod
    def from_returns(
        cls,
        returns: Any,
        *,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "Exposure":
        """Construct from the explicit non-zero-return activity proxy."""
        return cls._create(
            returns, "returns", nan_policy=nan_policy, column=column
        )

    def append(self, value: float) -> "Exposure":
        """Append one value in the factory-selected domain and return this metric."""
        self._state.append(float(value))
        return self

    def extend(
        self, values: Any, *, column: str | None = None
    ) -> "Exposure":
        """Append a chronological series in the selected domain and return this metric."""
        self._state.extend(as_metric_series(values, column=column))
        return self

    @property
    def value(self) -> float | None:
        """Return upward-rounded exposure, or ``None`` without usable periods."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the current O(1) scalar without replaying observations."""
        return self._state.compute()

    def reset(self) -> "Exposure":
        """Clear counters, preserve configuration, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of usable periods counted by Rust."""
        return len(self._state)


__all__ = ["Exposure"]
