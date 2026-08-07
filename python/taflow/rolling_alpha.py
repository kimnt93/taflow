"""Rolling regression alpha and information ratio features."""

from typing import Any
import numpy as np
from ._native import RollingAlphaOperator, RollingInformationRatioOperator
from ._series import as_float64_series


class RollingAlpha:
    """Stateful RollingAlpha indicator.
    Parameters are documented by the constructor signature; scalar
    ``append`` returns the current value and ``compute`` returns
    the aligned history with NaN warm-up where applicable.
    """

    def __init__(
        self,
        _input: Any | None = None,
        benchmark: Any | None = None,
        timeperiod: int = 20,
    ):
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        _input : object
            Input series, scalar parameter, or configuration value for this operation.
        benchmark : object
            Input series, scalar parameter, or configuration value for this operation.
        timeperiod : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state = RollingAlphaOperator(timeperiod)
        (
            self.extend(_input, benchmark)
            if _input is not None or benchmark is not None
            else None
        )

    def append(self, _input: float, benchmark: float):
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        _input : object
            Input series, scalar parameter, or configuration value for this operation.
        benchmark : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(_input, benchmark)
        return self

    def extend(self, _input: Any, benchmark: Any):
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        _input : object
            Input series, scalar parameter, or configuration value for this operation.
        benchmark : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.extend(as_float64_series(_input), as_float64_series(benchmark))
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned output history as a NumPy array.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.compute()

    @property
    def value(self):
        """Return the latest computed value, or None during warm-up.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.value

    def reset(self):
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        return self


class RollingInformationRatio:
    """Stateful RollingInformationRatio indicator.
    Parameters are documented by the constructor signature; scalar
    ``append`` returns the current value and ``compute`` returns
    the aligned history with NaN warm-up where applicable.
    """

    def __init__(
        self,
        _input: Any | None = None,
        benchmark: Any | None = None,
        timeperiod: int = 20,
    ):
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        _input : object
            Input series, scalar parameter, or configuration value for this operation.
        benchmark : object
            Input series, scalar parameter, or configuration value for this operation.
        timeperiod : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state = RollingInformationRatioOperator(timeperiod)
        (
            self.extend(_input, benchmark)
            if _input is not None or benchmark is not None
            else None
        )

    def append(self, _input: float, benchmark: float):
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        _input : object
            Input series, scalar parameter, or configuration value for this operation.
        benchmark : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(_input, benchmark)
        return self

    def extend(self, _input: Any, benchmark: Any):
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        _input : object
            Input series, scalar parameter, or configuration value for this operation.
        benchmark : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.extend(as_float64_series(_input), as_float64_series(benchmark))
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned output history as a NumPy array.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.compute()

    @property
    def value(self):
        """Return the latest computed value, or None during warm-up.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.value

    def reset(self):
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        return self
