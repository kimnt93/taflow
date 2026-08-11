"""Whole-history arithmetic-return recovery factor metric."""

from __future__ import annotations

from typing import Any

from .._native.metrics import RecoveryFactor as _Native
from ._input import as_metric_series


class RecoveryFactor:
    """Compute absolute arithmetic return sum over absolute maximum drawdown.

    The frozen formula is ``abs(sum(simple_returns)) / abs(maximum_drawdown)``.
    It intentionally uses an arithmetic sum, not compounded total return, and
    freezes QuantStats 0.0.81 ``recovery_factor`` with ``rf=0.0`` and
    ``prepare_returns=False`` as the external oracle. QuantStats still obtains
    drawdown from the compounded wealth path; TAFlow applies that recurrence
    directly and never applies QuantStats' heuristic return preparation. A
    phantom wealth level of one precedes the first return. The result is
    ``None`` for empty input or a zero maximum-drawdown denominator, including
    an all-nonnegative path. Zero is a valid result when the signed arithmetic
    returns cancel after a non-zero drawdown. Warm-up ends after one usable
    return when it causes drawdown; otherwise the result remains undefined
    until a drawdown occurs.

    Select input meaning with ``from_returns``, ``from_log_returns``,
    ``from_equity``, or ``from_pnl``. Returns are decimal simple returns. Rust
    converts log returns with ``expm1`` and positive equity levels into causal
    returns. P&L is non-cumulative period P&L and requires positive initial
    equity. Raw P&L and closed trades are deliberately unsupported because this
    factor requires a wealth-return path. NaNs are omitted by default or
    rejected with ``nan_policy="raise"``; infinities and simple returns below
    -100% are always rejected. ``append``, ``extend``, and ``reset`` mutate and
    fluently return this instance; all arithmetic and state live in Rust.
    """

    def __init__(self) -> None:
        """Reject ambiguous construction; use a semantic ``from_*`` factory."""
        raise TypeError(
            "use RecoveryFactor.from_returns/from_equity/from_pnl/from_log_returns"
        )

    @classmethod
    def _create(
        cls,
        values: Any,
        input_mode: str,
        *,
        initial_equity: float | None = None,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "RecoveryFactor":
        state = cls.__new__(cls)
        state._state = _Native(
            input_mode, initial_equity=initial_equity, nan_policy=nan_policy
        )
        return state.extend(values, column=column)

    @classmethod
    def from_returns(
        cls,
        returns: Any,
        *,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "RecoveryFactor":
        """Construct from chronological decimal simple returns."""
        return cls._create(
            returns, "returns", nan_policy=nan_policy, column=column
        )

    @classmethod
    def from_log_returns(
        cls,
        log_returns: Any,
        *,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "RecoveryFactor":
        """Construct from chronological log returns converted by Rust."""
        return cls._create(
            log_returns, "log_returns", nan_policy=nan_policy, column=column
        )

    @classmethod
    def from_equity(
        cls,
        equity: Any,
        *,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "RecoveryFactor":
        """Construct from positive chronological equity or adjusted-price levels."""
        return cls._create(equity, "equity", nan_policy=nan_policy, column=column)

    @classmethod
    def from_pnl(
        cls,
        pnl: Any,
        *,
        initial_equity: float,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "RecoveryFactor":
        """Construct from non-cumulative period P&L and positive initial equity."""
        return cls._create(
            pnl,
            "pnl",
            initial_equity=float(initial_equity),
            nan_policy=nan_policy,
            column=column,
        )

    def append(self, value: float) -> "RecoveryFactor":
        """Append one value in the factory-selected domain and return this metric."""
        self._state.append(float(value))
        return self

    def extend(
        self, values: Any, *, column: str | None = None
    ) -> "RecoveryFactor":
        """Append a series in the selected domain and return this metric."""
        self._state.extend(as_metric_series(values, column=column))
        return self

    @property
    def value(self) -> float | None:
        """Return the recovery factor, or ``None`` until it is defined."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the current scalar without replaying processed input."""
        return self._state.compute()

    def reset(self) -> "RecoveryFactor":
        """Clear observations, preserve input configuration, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of usable normalized returns processed by Rust."""
        return len(self._state)


__all__ = ["RecoveryFactor"]
