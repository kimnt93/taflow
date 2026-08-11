"""Geometric annual-return to maximum-drawdown ratio metric."""

from __future__ import annotations

from typing import Any

from .._native.metrics import CalmarRatio as _Native
from ._input import as_metric_series


class CalmarRatio:
    """Compute geometric annualized return divided by absolute drawdown.

    Rust compounds normalized simple returns and computes
    ``annualized_return / abs(maximum_drawdown)`` in one chronological pass
    with O(1) memory. ``periods_per_year`` defaults to 252 and is explicit; it
    is never inferred from an index. The warm-up requires one usable return and a
    strictly negative maximum drawdown. ``value`` and ``compute`` return
    ``None`` while empty, when the path has no drawdown, or when the ratio is
    non-finite. The external oracle/name mapping is Empyrical Reloaded 0.5.12
    ``calmar_ratio`` with its ``annualization`` argument.

    Select input meaning with ``from_returns``, ``from_log_returns``,
    ``from_equity``, or ``from_pnl``. Returns are decimal simple returns. Log
    returns are converted with ``expm1``. Positive equity levels establish a
    causal return stream, with the first level serving only as its baseline.
    P&L is non-cumulative period P&L and requires positive initial equity.
    NaNs are omitted by default or rejected with ``nan_policy="raise"``;
    infinities and simple returns below -1 are always rejected. ``append``,
    ``extend``, and ``reset`` mutate and fluently return this instance.
    """

    def __init__(self) -> None:
        """Reject ambiguous construction; use a semantic ``from_*`` factory."""
        raise TypeError(
            "use CalmarRatio.from_returns/from_equity/from_pnl/from_log_returns"
        )

    @classmethod
    def _create(
        cls,
        values: Any,
        input_mode: str,
        *,
        periods_per_year: float,
        initial_equity: float | None = None,
        nan_policy: str,
        column: str | None,
    ) -> "CalmarRatio":
        state = cls.__new__(cls)
        state._state = _Native(
            input_mode,
            float(periods_per_year),
            initial_equity=initial_equity,
            nan_policy=nan_policy,
        )
        return state.extend(values, column=column)

    @classmethod
    def from_returns(
        cls,
        returns: Any,
        *,
        periods_per_year: float = 252.0,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "CalmarRatio":
        """Construct from chronological decimal simple returns."""
        return cls._create(
            returns,
            "returns",
            periods_per_year=periods_per_year,
            nan_policy=nan_policy,
            column=column,
        )

    @classmethod
    def from_log_returns(
        cls,
        log_returns: Any,
        *,
        periods_per_year: float = 252.0,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "CalmarRatio":
        """Construct from chronological log returns converted by Rust."""
        return cls._create(
            log_returns,
            "log_returns",
            periods_per_year=periods_per_year,
            nan_policy=nan_policy,
            column=column,
        )

    @classmethod
    def from_equity(
        cls,
        equity: Any,
        *,
        periods_per_year: float = 252.0,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "CalmarRatio":
        """Construct from positive chronological equity or adjusted-price levels."""
        return cls._create(
            equity,
            "equity",
            periods_per_year=periods_per_year,
            nan_policy=nan_policy,
            column=column,
        )

    @classmethod
    def from_pnl(
        cls,
        pnl: Any,
        *,
        initial_equity: float,
        periods_per_year: float = 252.0,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "CalmarRatio":
        """Construct from non-cumulative period P&L and positive initial equity."""
        return cls._create(
            pnl,
            "pnl",
            periods_per_year=periods_per_year,
            initial_equity=float(initial_equity),
            nan_policy=nan_policy,
            column=column,
        )

    def append(self, value: float) -> "CalmarRatio":
        """Append one value in the factory-selected domain and return this metric."""
        self._state.append(float(value))
        return self

    def extend(
        self, values: Any, *, column: str | None = None
    ) -> "CalmarRatio":
        """Append a chronological series in the selected domain and return this metric."""
        self._state.extend(as_metric_series(values, column=column))
        return self

    @property
    def value(self) -> float | None:
        """Return the current ratio, or ``None`` until it is defined."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the current scalar without replaying processed input."""
        return self._state.compute()

    def reset(self) -> "CalmarRatio":
        """Clear observations, preserve configuration, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of usable normalized returns processed by Rust."""
        return len(self._state)


__all__ = ["CalmarRatio"]
