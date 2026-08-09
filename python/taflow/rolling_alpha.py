"""Rolling regression alpha and information ratio features."""

from typing import Any
import numpy as np
from ._native import RollingAlphaOperator
from ._series import as_float64_series


class RollingAlpha:
    """Rolling regression alpha and information ratio features.

    This public class owns a persistent native Rust state; Python performs container conversion only. `append`, `extend`, and `reset` are fluent, `value` exposes the latest result, and `compute` returns aligned history. Required input histories: `_input`, `benchmark`. Warm-up positions are represented by `NaN` in history."""

    def __init__(
        self,
        _input: Any,
        benchmark: Any,
        timeperiod: int = 20,
    ) -> None:
        """Initialize this adapter and process the supplied input series.

        Parameters
        ----------
        _input : object
            Input series or the current scalar observation.
        benchmark : object
            Benchmark series aligned with the input observations.
        timeperiod : object
            Trailing window length in bars.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = RollingAlphaOperator(timeperiod)
        (
            self.extend(_input, benchmark)
            if _input is not None or benchmark is not None
            else None
        )

    def append(self, _input: float, benchmark: float) -> "RollingAlpha":
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        _input : object
            Input series or the current scalar observation.
        benchmark : object
            Benchmark series aligned with the input observations.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(_input, benchmark)
        return self

    def extend(self, _input: Any, benchmark: Any) -> "RollingAlpha":
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        _input : object
            Input series or the current scalar observation.
        benchmark : object
            Benchmark series aligned with the input observations.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.extend(as_float64_series(_input), as_float64_series(benchmark))
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned output history as a NumPy array.

        Returns
        -------
        numpy.ndarray or tuple of numpy.ndarray
            The updated adapter, native value, aligned output array, or execution node.
        """
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

    def reset(self) -> "RollingAlpha":
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        return self
