"""Native SSL Channel interface."""

from typing import Any

import numpy as np

from ._native import StatefulSmoothedTrendChannel


class SmoothedTrendChannel:
    """Compute rolling high/low averages with causal trend-side ordering.

    Parameters
    ----------
    high, low, close : array-like, optional
        Initial aligned OHLC history.
    length : int, default 10
        Rolling average period.
    """

    def __init__(
        self,
        high: Any | None = None,
        low: Any | None = None,
        close: Any | None = None,
        length: int = 10,
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
        length : object
            Values or parameters consumed by this operation.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = StatefulSmoothedTrendChannel(length)
        if high is not None or low is not None or close is not None:
            self.extend(high, low, close)

    def append(self, high: float, low: float, close: float) -> object:
        """Process one OHLC bar and return lower/upper channel values."""
        return self._state.append(float(high), float(low), float(close))

    def extend(self, high: Any, low: Any, close: Any) -> object:
        """Process aligned OHLC history and return this indicator."""
        self._state.extend(
            np.asarray(high, dtype=np.float64),
            np.asarray(low, dtype=np.float64),
            np.asarray(close, dtype=np.float64),
        )
        return self

    def compute(self) -> object:
        """Return lower and upper channel histories."""
        return self._state.compute()

    @property
    def value(self) -> object:
        """Return the latest channel pair, or ``None`` if not warm."""
        return self._state.value

    def reset(self) -> object:
        """Clear state and accumulated channel history."""
        self._state.reset()
        return self
