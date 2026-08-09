"""Persistent anchored volume-weighted average price and deviation bands."""

from typing import Any

import numpy as np

from ._native import (
    AnchoredVolumeWeightedAveragePrice as _NativeAnchoredVolumeWeightedAveragePrice,
)
from ._series import as_bool_series, as_float64_series


class AnchoredVolumeWeightedAveragePrice:
    """Compute anchored volume-weighted typical price and deviation bands.

    Parameters
    ----------
    high : array-like
        Initial chronological high prices. Pass an empty aligned series for a
        fresh streaming state.
    low : array-like
        Initial chronological low prices aligned with ``high``.
    close : array-like
        Initial chronological close prices aligned with ``high``.
    volume : array-like
        Initial chronological volumes aligned with ``high``.
    anchor : array-like of bool
        Reset flags aligned with ``high``. A true flag starts a new weighted
        accumulation at the current bar.
    standard_deviation_multiplier : float, default 1.0
        Non-negative finite multiplier for the upper and lower bands.

    Notes
    -----
    Rust computes typical price as ``(high + low + close) / 3`` and maintains
    cumulative volume-weighted first and second moments from the latest anchor.
    There is no warm-up: every input bar yields ``(average, upper, lower)``;
    zero cumulative volume yields NaN for all three outputs. ``compute`` returns
    three arrays in that order. The independent oracle is pandas grouped
    cumulative weighted moments. ``append``, ``extend``, and ``reset`` mutate
    and return this adapter. All five series are required; use five empty
    aligned series to create an unseeded streaming state.
    """

    def __init__(
        self,
        high: Any,
        low: Any,
        close: Any,
        volume: Any,
        anchor: Any,
        standard_deviation_multiplier: float = 1.0,
    ) -> None:
        self._state = _NativeAnchoredVolumeWeightedAveragePrice(
            float(standard_deviation_multiplier)
        )
        self.extend(high, low, close, volume, anchor)

    def append(
        self, high: float, low: float, close: float, volume: float, anchor: bool
    ) -> "AnchoredVolumeWeightedAveragePrice":
        """Append one chronological high/low/close/volume/anchor bar.

        Parameters
        ----------
        high, low, close, volume : float
            The next price and volume inputs in that order.
        anchor : bool
            Whether the current bar resets the weighted accumulation.

        Returns
        -------
        AnchoredVolumeWeightedAveragePrice
            This updated adapter; read ``value`` for the latest three outputs.
        """
        self._state.append(
            float(high), float(low), float(close), float(volume), bool(anchor)
        )
        return self

    def extend(
        self, high: Any, low: Any, close: Any, volume: Any, anchor: Any
    ) -> "AnchoredVolumeWeightedAveragePrice":
        """Append aligned chronological high/low/close/volume/anchor histories.

        Parameters
        ----------
        high, low, close, volume : array-like
            Numeric one-dimensional series in that order.
        anchor : array-like of bool
            One-dimensional reset flags. Unequal lengths are rejected by Rust
            before state or output histories are mutated.

        Returns
        -------
        AnchoredVolumeWeightedAveragePrice
            This updated adapter.
        """
        self._state.extend(
            as_float64_series(high),
            as_float64_series(low),
            as_float64_series(close),
            as_float64_series(volume),
            as_bool_series(anchor),
        )
        return self

    def compute(self) -> tuple[np.ndarray, np.ndarray, np.ndarray]:
        """Return complete average, upper-band, and lower-band histories.

        Returns
        -------
        tuple of numpy.ndarray
            Three aligned arrays in average, upper, lower order, including NaN
            values produced by bars with zero cumulative volume.
        """
        return self._state.compute()

    @property
    def value(self) -> tuple[float, float, float] | None:
        """Return the latest average/upper/lower tuple, or ``None`` when empty."""
        return self._state.value

    def reset(self) -> "AnchoredVolumeWeightedAveragePrice":
        """Restore fresh native state and clear all three output histories.

        Returns
        -------
        AnchoredVolumeWeightedAveragePrice
            This reset adapter.
        """
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed bars."""
        return len(self._state)


__all__ = ["AnchoredVolumeWeightedAveragePrice"]
