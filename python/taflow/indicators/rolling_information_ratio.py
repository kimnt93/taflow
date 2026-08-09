"""Rolling information-ratio feature."""

from typing import Any
import numpy as np
from .._native import RollingInformationRatioOperator as _Native
from .._series import as_float64_series


class RollingInformationRatio:
    """Rolling information-ratio feature.

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
        self._state = _Native(timeperiod)
        self._length = 0
        self.extend(_input, benchmark)

    def append(self, _input: float, benchmark: float) -> "RollingInformationRatio":
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
        self._state.append(float(_input), float(benchmark))
        self._length += 1
        return self

    def extend(self, _input: Any, benchmark: Any) -> "RollingInformationRatio":
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
        input_array = as_float64_series(_input)
        benchmark_array = as_float64_series(benchmark)
        if input_array.shape != benchmark_array.shape:
            raise ValueError("_input and benchmark must have equal lengths")
        self._state.extend(input_array, benchmark_array)
        self._length += len(input_array)
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
    def value(self) -> float | None:
        """Return the latest computed value, or None during warm-up.

        Returns
        -------
        float, tuple, or None
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.value

    def reset(self) -> "RollingInformationRatio":
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        self._length = 0
        return self

    def __len__(self) -> int:
        """Return the number of processed observations."""
        return self._length
