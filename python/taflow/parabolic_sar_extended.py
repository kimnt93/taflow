"""Descriptive stateful interface for extended Parabolic SAR."""

from taflow._native import StatefulSarext
from typing import Any

import numpy as np


class ParabolicSarExtended:
    """Incrementally compute signed SAREXT with independent trend settings."""

    def __init__(
        self,
        start_value: object = 0.0,
        offset_on_reverse: object = 0.0,
        acceleration_init_long: object = 0.02,
        acceleration_long: object = 0.02,
        acceleration_max_long: object = 0.2,
        acceleration_init_short: object = 0.02,
        acceleration_short: object = 0.02,
        acceleration_max_short: object = 0.2,
        high: Any | None = None,
        low: Any | None = None,
    ) -> None:
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        start_value : object
            Values or parameters consumed by this operation.
        offset_on_reverse : object
            Values or parameters consumed by this operation.
        acceleration_init_long : object
            Values or parameters consumed by this operation.
        acceleration_long : object
            Values or parameters consumed by this operation.
        acceleration_max_long : object
            Values or parameters consumed by this operation.
        acceleration_init_short : object
            Values or parameters consumed by this operation.
        acceleration_short : object
            Values or parameters consumed by this operation.
        acceleration_max_short : object
            Values or parameters consumed by this operation.
        high : object
            High-price series or the current bar high.
        low : object
            Low-price series or the current bar low.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = StatefulSarext(
            start_value,
            offset_on_reverse,
            acceleration_init_long,
            acceleration_long,
            acceleration_max_long,
            acceleration_init_short,
            acceleration_short,
            acceleration_max_short,
        )
        self._values: list[float] = []
        if high is not None or low is not None:
            self.extend(high, low)

    def append(self, high: float, low: float) -> "ParabolicSarExtended":
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        high : object
            High-price series or the current bar high.
        low : object
            Low-price series or the current bar low.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        result = self._state.append(float(high), float(low))
        self._values.append(np.nan if result is None else float(result))
        return self

    def extend(self, high: Any, low: Any) -> "ParabolicSarExtended":
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        high : object
            High-price series or the current bar high.
        low : object
            Low-price series or the current bar low.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        result = self._state.extend(high, low)
        self._values.extend(np.asarray(result, dtype=np.float64).tolist())
        return self

    def compute(self) -> np.ndarray:
        """Return aligned extended Parabolic SAR values."""
        return np.asarray(self._values, dtype=np.float64)

    @property
    def value(self) -> object:
        """Return the latest computed value, or None during warm-up.

        Returns
        -------
        float, tuple, or None
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.value

    def reset(self) -> "ParabolicSarExtended":
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        self._values.clear()
        return self
