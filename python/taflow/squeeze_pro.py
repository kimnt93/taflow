"""Persistent Squeeze PRO (pandas-ta classic alignment)."""

from typing import Any
import numpy as np
from ._native import SqueezeProOperator as _Native
from ._series import as_float64_series


class SqueezePro:
    """Stateful SqueezePro indicator.
    Parameters are documented by the constructor signature; scalar
    ``append`` returns the current value and ``compute`` returns
    the aligned history with NaN warm-up where applicable.
    """

    def __init__(
        self,
        high: Any | None = None,
        low: Any | None = None,
        close: Any | None = None,
        bb_length: int = 20,
        bb_std: float = 2.0,
        kc_length: int = 20,
        kc_scalar_wide: float = 2.0,
        kc_scalar_normal: float = 1.5,
        kc_scalar_narrow: float = 1.0,
        mom_length: int = 12,
        mom_smooth: int = 6,
    ):
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        high : object
            Input series, scalar parameter, or configuration value for this operation.
        low : object
            Input series, scalar parameter, or configuration value for this operation.
        close : object
            Input series, scalar parameter, or configuration value for this operation.
        bb_length : object
            Input series, scalar parameter, or configuration value for this operation.
        bb_std : object
            Input series, scalar parameter, or configuration value for this operation.
        kc_length : object
            Input series, scalar parameter, or configuration value for this operation.
        kc_scalar_wide : object
            Input series, scalar parameter, or configuration value for this operation.
        kc_scalar_normal : object
            Input series, scalar parameter, or configuration value for this operation.
        kc_scalar_narrow : object
            Input series, scalar parameter, or configuration value for this operation.
        mom_length : object
            Input series, scalar parameter, or configuration value for this operation.
        mom_smooth : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state = _Native(
            bb_length,
            bb_std,
            kc_length,
            kc_scalar_wide,
            kc_scalar_normal,
            kc_scalar_narrow,
            mom_length,
            mom_smooth,
        )
        (
            self.extend(high, low, close)
            if any(value is not None for value in (high, low, close))
            else None
        )

    def append(self, high: float, low: float, close: float):
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        high : object
            Input series, scalar parameter, or configuration value for this operation.
        low : object
            Input series, scalar parameter, or configuration value for this operation.
        close : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(high, low, close)
        return self

    def extend(self, high: Any, low: Any, close: Any):
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        high : object
            Input series, scalar parameter, or configuration value for this operation.
        low : object
            Input series, scalar parameter, or configuration value for this operation.
        close : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.extend(
            as_float64_series(high), as_float64_series(low), as_float64_series(close)
        )
        return self

    def compute(
        self,
    ) -> tuple[np.ndarray, np.ndarray, np.ndarray, np.ndarray, np.ndarray, np.ndarray]:
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
