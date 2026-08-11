"""Annualized Sortino-ratio metric."""

from __future__ import annotations

from typing import Any

from .._native.metrics import SortinoRatio as _Native
from ._input import as_metric_series


class SortinoRatio:
    """Compute annualized excess return per unit of downside deviation.

    Rust converts the annual effective ``annual_required_return`` to a
    per-period minimum acceptable return and computes Empyrical Reloaded 0.5.12
    ``sortino_ratio``: annualized arithmetic mean excess return divided by the
    annualized square root of the lower partial second moment over all usable
    observations. This two-observation warm-up and a zero downside denominator
    both make ``value`` and ``compute`` return ``None``. The independent oracle
    and name mapping is Empyrical Reloaded 0.5.12 ``sortino_ratio``.

    Select input meaning with ``from_returns``, ``from_log_returns``,
    ``from_equity``, or ``from_pnl``. The P&L factory accepts non-cumulative
    period P&L and positive initial equity for causal return conversion. The
    first equity level only establishes the baseline. ``periods_per_year``
    defaults to 252 and is never inferred. NaNs are omitted by default;
    ``nan_policy="raise"`` rejects them, and infinities are always rejected.
    Lifecycle mutations are fluent and state uses O(1) memory.
    """

    def __init__(self) -> None:
        """Reject ambiguous construction; use a semantic ``from_*`` factory."""
        raise TypeError(
            "use SortinoRatio.from_returns/from_equity/from_pnl/from_log_returns"
        )

    @classmethod
    def _create(
        cls,
        values: Any,
        input_mode: str,
        *,
        periods_per_year: float,
        annual_required_return: float,
        initial_equity: float | None,
        nan_policy: str,
        column: str | None,
    ) -> "SortinoRatio":
        state = cls.__new__(cls)
        state._state = _Native(
            input_mode,
            float(periods_per_year),
            float(annual_required_return),
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
        annual_required_return: float = 0.0,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "SortinoRatio":
        """Build from chronological decimal simple returns."""
        return cls._create(
            returns,
            "returns",
            periods_per_year=periods_per_year,
            annual_required_return=annual_required_return,
            initial_equity=None,
            nan_policy=nan_policy,
            column=column,
        )

    @classmethod
    def from_log_returns(
        cls,
        log_returns: Any,
        *,
        periods_per_year: float = 252.0,
        annual_required_return: float = 0.0,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "SortinoRatio":
        """Build from chronological log returns converted by Rust."""
        return cls._create(
            log_returns,
            "log_returns",
            periods_per_year=periods_per_year,
            annual_required_return=annual_required_return,
            initial_equity=None,
            nan_policy=nan_policy,
            column=column,
        )

    @classmethod
    def from_equity(
        cls,
        equity: Any,
        *,
        periods_per_year: float = 252.0,
        annual_required_return: float = 0.0,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "SortinoRatio":
        """Build from positive chronological equity or adjusted-price levels."""
        return cls._create(
            equity,
            "equity",
            periods_per_year=periods_per_year,
            annual_required_return=annual_required_return,
            initial_equity=None,
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
        annual_required_return: float = 0.0,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "SortinoRatio":
        """Build from non-cumulative period P&L and positive initial equity."""
        return cls._create(
            pnl,
            "pnl",
            periods_per_year=periods_per_year,
            annual_required_return=annual_required_return,
            initial_equity=float(initial_equity),
            nan_policy=nan_policy,
            column=column,
        )

    def append(self, value: float) -> "SortinoRatio":
        """Append one factory-domain observation and return this metric."""
        self._state.append(float(value))
        return self

    def extend(
        self, values: Any, *, column: str | None = None
    ) -> "SortinoRatio":
        """Append a chronological series in the selected domain and return self."""
        self._state.extend(as_metric_series(values, column=column))
        return self

    @property
    def value(self) -> float | None:
        """Return the current ratio, or ``None`` when it is undefined."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the current scalar without replaying or mutating native state."""
        return self._state.compute()

    def reset(self) -> "SortinoRatio":
        """Clear observations, preserve configuration, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of usable normalized returns processed by Rust."""
        return len(self._state)


__all__ = ["SortinoRatio"]
