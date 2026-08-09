"""Descriptive stateful interface for extended Parabolic SAR."""

from taflow._native import StatefulSarext
from typing import Any

import numpy as np


class ParabolicSarExtended:
    """Incrementally compute signed SAREXT with independent trend settings

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    ParabolicSarExtended
        A persistent native-backed indicator adapter.
    """

    def __init__(
        self,
        high: Any,
        low: Any,
        start_value: object = 0.0,
        offset_on_reverse: object = 0.0,
        acceleration_init_long: object = 0.02,
        acceleration_long: object = 0.02,
        acceleration_max_long: object = 0.2,
        acceleration_init_short: object = 0.02,
        acceleration_short: object = 0.02,
        acceleration_max_short: object = 0.2,
    ) -> None:
        """Initialize this adapter and process the supplied input series.

        Parameters
        ----------
        start_value : object
            Input parameter or configuration value for this operation.
        offset_on_reverse : object
            Input parameter or configuration value for this operation.
        acceleration_init_long : object
            Input parameter or configuration value for this operation.
        acceleration_long : object
            Input parameter or configuration value for this operation.
        acceleration_max_long : object
            Input parameter or configuration value for this operation.
        acceleration_init_short : object
            Input parameter or configuration value for this operation.
        acceleration_short : object
            Input parameter or configuration value for this operation.
        acceleration_max_short : object
            Input parameter or configuration value for this operation.
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
        self._state.append(float(high), float(low))
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
        self._state.extend(high, low)
        return self

    def compute(self) -> np.ndarray:
        """Return the complete aligned history produced by Rust.

        Returns
        -------
        numpy.ndarray or tuple of numpy.ndarray
            One output per processed bar, including NaN warm-up positions."""
        return self._state.compute()

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
        return self

    def __len__(self) -> int:
        return len(self._state)
