"""Descriptive stateful interface for extended Parabolic SAR."""

from typing import Any

import numpy as np

from .._native import ParabolicSarExtended as _NativeParabolicSarExtended
from .._series import as_float64_series


class ParabolicSarExtended:
    """Incrementally compute signed SAREXT with independent trend settings

    Parameters
    ----------
    Construct with configuration values only; supply input series through ``extend``.

    Returns
    -------
    ParabolicSarExtended
        A persistent native-backed indicator adapter.
    """

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
    ) -> None:
        """Initialize an empty configured native state.

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

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = _NativeParabolicSarExtended(
            start_value,
            offset_on_reverse,
            acceleration_init_long,
            acceleration_long,
            acceleration_max_long,
            acceleration_init_short,
            acceleration_short,
            acceleration_max_short,
        )

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
        high_array = as_float64_series(high)
        low_array = as_float64_series(low)
        if len(high_array) != len(low_array):
            raise ValueError("high and low must have equal lengths")
        self._state.extend(high_array, low_array)
        return self

    def compute(self) -> np.ndarray:
        """Return the complete aligned history produced by Rust.

        Returns
        -------
        numpy.ndarray or tuple of numpy.ndarray
            One output per processed bar, including NaN warm-up positions."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
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
