"""Annualized-return-to-mean-drawdown Pain ratio metric."""

from __future__ import annotations

from typing import Any

from .._native.metrics import PainRatio as _Native
from ._input import as_metric_series


class PainRatio:
    """Compute geometric annualized excess return divided by Pain Index.

    TAFlow follows PerformanceAnalytics 2.1.0 ``PainRatio`` and ``PainIndex``:
    ``(geometric CAGR - annual_risk_free_rate) / mean(abs(drawdown))``. A
    phantom wealth level of one precedes the first normalized return, but only
    real usable observations enter the Pain Index divisor. The source contract
    is pinned to the CRAN 2.1.0 tarball with SHA-256
    ``fc801d39382818cd3a7052326b45d078302aef4d290c85dab83498ed4516d58d``.
    No R runtime is available in the test environment, so independent tests
    translate this pinned source rather than claiming an executable oracle
    match.

    ``periods_per_year`` defaults to 252 and is explicit rather than inferred
    from timestamps. ``annual_risk_free_rate`` is an annual effective rate and
    is subtracted after annualizing the portfolio return, exactly where the
    source uses ``Rf``. Warm-up ends with the first usable observation having
    positive drawdown pain. Empty and zero-pain paths return ``None``.

    Inputs may be decimal simple returns, log returns, positive equity levels,
    or non-cumulative period P&L with positive initial equity. Rust performs
    all conversions and O(1)-memory arithmetic. NaNs are omitted by default or
    rejected with ``nan_policy="raise"``; infinities and returns below -100%
    are rejected. Mutating lifecycle methods are fluent and bulk execution
    releases the GIL.
    """

    def __init__(self) -> None:
        """Reject ambiguous construction; use a semantic ``from_*`` factory."""
        raise TypeError(
            "use PainRatio.from_returns/from_equity/from_pnl/from_log_returns"
        )

    @classmethod
    def _create(
        cls,
        values: Any,
        input_mode: str,
        *,
        periods_per_year: float,
        annual_risk_free_rate: float,
        initial_equity: float | None,
        nan_policy: str,
        column: str | None,
    ) -> "PainRatio":
        state = cls.__new__(cls)
        state._state = _Native(
            input_mode,
            float(periods_per_year),
            float(annual_risk_free_rate),
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
        annual_risk_free_rate: float = 0.0,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "PainRatio":
        """Construct from chronological decimal simple returns."""
        return cls._create(
            returns,
            "returns",
            periods_per_year=periods_per_year,
            annual_risk_free_rate=annual_risk_free_rate,
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
        annual_risk_free_rate: float = 0.0,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "PainRatio":
        """Construct from chronological log returns converted by Rust."""
        return cls._create(
            log_returns,
            "log_returns",
            periods_per_year=periods_per_year,
            annual_risk_free_rate=annual_risk_free_rate,
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
        annual_risk_free_rate: float = 0.0,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "PainRatio":
        """Construct from positive chronological equity or adjusted-price levels."""
        return cls._create(
            equity,
            "equity",
            periods_per_year=periods_per_year,
            annual_risk_free_rate=annual_risk_free_rate,
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
        annual_risk_free_rate: float = 0.0,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "PainRatio":
        """Construct from non-cumulative period P&L and positive initial equity."""
        return cls._create(
            pnl,
            "pnl",
            periods_per_year=periods_per_year,
            annual_risk_free_rate=annual_risk_free_rate,
            initial_equity=float(initial_equity),
            nan_policy=nan_policy,
            column=column,
        )

    def append(self, value: float) -> "PainRatio":
        """Append one value in the factory-selected domain and return this metric."""
        self._state.append(float(value))
        return self

    def extend(
        self, values: Any, *, column: str | None = None
    ) -> "PainRatio":
        """Append a chronological series in the selected domain and return this metric."""
        self._state.extend(as_metric_series(values, column=column))
        return self

    @property
    def value(self) -> float | None:
        """Return the Pain ratio, or ``None`` until positive pain exists."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the current scalar without replaying processed input."""
        return self._state.compute()

    def reset(self) -> "PainRatio":
        """Clear observations, preserve configuration, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of usable normalized returns processed by Rust."""
        return len(self._state)


__all__ = ["PainRatio"]
