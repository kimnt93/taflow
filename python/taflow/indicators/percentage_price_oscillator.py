"""Native-backed Percentage Price Oscillator adapter."""

from typing import Any

import numpy as np

from .._native import PercentagePriceOscillator as _Native
from .._series import as_float64_series


class PercentagePriceOscillator:
    """Compute the percentage difference between fast and slow averages.

    ``_input`` is the required chronological price history and may be empty
    for a fresh stream. ``fastperiod`` defaults to 12, ``slowperiod`` to 26,
    and ``moving_average_type`` to 0 (SMA), matching TA-Lib ``PPO``. Rust owns
    moving-average arithmetic and NaN warm-up; ``compute`` returns one aligned
    float array and ``value`` exposes the latest scalar. Lifecycle mutators
    return ``self``.
    """

    def __init__(
        self,
        fastperiod: int = 12,
        slowperiod: int = 26,
        moving_average_type: int = 0,
    ) -> None:
        """Initialize an empty configured native state.

        Parameters
        ----------
        fastperiod, slowperiod : int, default 12 and 26
            Positive fast and slow average periods; slow must exceed fast.
        moving_average_type : int, default 0
            TA-Lib moving-average selector.
        """
        self._state = _Native(
            int(fastperiod), int(slowperiod), int(moving_average_type)
        )

    def append(self, _input: float) -> "PercentagePriceOscillator":
        """Append one price and return this adapter."""
        self._state.append(float(_input))
        return self

    def extend(self, _input: Any) -> "PercentagePriceOscillator":
        """Append a chronological price history and return this adapter."""
        self._state.extend(as_float64_series(_input))
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned PPO history with NaN warm-up."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest PPO value, or ``None`` during warm-up."""
        return self._state.value

    def reset(self) -> "PercentagePriceOscillator":
        """Restore fresh native state and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of prices processed by Rust."""
        return len(self._state)


__all__ = ["PercentagePriceOscillator"]
