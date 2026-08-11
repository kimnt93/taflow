"""Ornstein-Uhlenbeck mean-reversion half-life of a price series."""

from typing import Any
import numpy as np
from .._adapter_protocol import adapter_length
from .._native import OrnsteinUhlenbeckHalfLifeOperator as _Native
from .._series import as_float64_series


class OrnsteinUhlenbeckHalfLife:
    """Ornstein-Uhlenbeck mean-reversion half-life of a price series.

    This public class owns a persistent native Rust state; Python performs container conversion only. `append`, `extend`, and `reset` are fluent, `value` exposes the latest result, and `compute` returns aligned history. Required input histories: `price`. Warm-up positions are represented by `NaN` in history."""

    def __init__(
        self,
        price: Any,
        timeperiod: int = 20,
    ) -> None:
        """Initialize this adapter and process the supplied input series.

        Parameters
        ----------
        price : object
            Price series or the current price observation.
        timeperiod : object
            Trailing window length in bars.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = _Native(timeperiod)
        self.extend(price)

    def append(self, price: float) -> "OrnsteinUhlenbeckHalfLife":
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        price : object
            Price series or the current price observation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(float(price))
        return self

    def extend(self, price: Any) -> "OrnsteinUhlenbeckHalfLife":
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        price : object
            Price series or the current price observation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        price_array = as_float64_series(price)
        self._state.extend(price_array)
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

    def reset(self) -> "OrnsteinUhlenbeckHalfLife":
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed prices."""
        return adapter_length(self)
