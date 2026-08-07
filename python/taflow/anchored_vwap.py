"""Native anchored volume-weighted average price interface."""

from typing import Any

import numpy as np

from ._native import StatefulAnchoredVolumeWeightedAveragePrice


class AnchoredVolumeWeightedAveragePrice:
    """Compute anchored weighted price and running deviation bands.

    Parameters
    ----------
    high, low, close, volume : array-like, optional
        Initial aligned OHLCV history.
    anchor : array-like of bool, optional
        Session-boundary flags for the initial history.
    stdev : float, default 1
        Standard-deviation band multiplier.
    """

    def __init__(
        self,
        high: Any | None = None,
        low: Any | None = None,
        close: Any | None = None,
        volume: Any | None = None,
        anchor: Any | None = None,
        stdev: float = 1.0,
    ) -> None:
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        high : object
            High-price series or the current bar high.
        low : object
            Low-price series or the current bar low.
        close : object
            Close-price series or the current bar close.
        volume : object
            Volume series or the current bar volume.
        anchor : object
            Boolean series marking reset or anchor bars.
        stdev : object
            Values or parameters consumed by this operation.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = StatefulAnchoredVolumeWeightedAveragePrice(stdev)
        if close is not None:
            self.extend(high, low, close, volume, anchor)

    def append(
        self, high: float, low: float, close: float, volume: float, anchor: bool = False
    ) -> object:
        """Process one OHLCV bar and return mean, upper, and lower bands."""
        return self._state.append(
            float(high), float(low), float(close), float(volume), bool(anchor)
        )

    def extend(
        self, high: Any, low: Any, close: Any, volume: Any, anchor: Any | None = None
    ) -> object:
        """Process aligned OHLCV history and return this indicator."""
        close_array = np.asarray(close, dtype=np.float64)
        if anchor is None:
            anchor = np.zeros(close_array.shape, dtype=np.bool_)
        self._state.extend(
            np.asarray(high, dtype=np.float64),
            np.asarray(low, dtype=np.float64),
            close_array,
            np.asarray(volume, dtype=np.float64),
            np.asarray(anchor, dtype=np.bool_),
        )
        return self

    def compute(self) -> object:
        """Return mean, upper-band, and lower-band histories."""
        return self._state.compute()

    @property
    def value(self) -> object:
        """Return the latest weighted mean and bands."""
        return self._state.value

    def reset(self) -> object:
        """Clear weighted moments and output history."""
        self._state.reset()
        return self
