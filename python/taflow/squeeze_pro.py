"""Persistent Squeeze PRO (pandas-ta classic alignment)."""

from typing import Any
import numpy as np
from ._native import SqueezeProOperator as _Native
from ._series import as_float64_series


class SqueezePro:
    """Persistent Squeeze PRO (pandas-ta classic alignment).

    This public class owns a persistent native Rust state; Python performs container conversion only. `append`, `extend`, and `reset` are fluent, `value` exposes the latest result, and `compute` returns aligned history. Required input histories: `high`, `low`, `close`. Warm-up positions are represented by `NaN` in history."""

    def __init__(
        self,
        high: Any,
        low: Any,
        close: Any,
        bb_length: int = 20,
        bb_std: float = 2.0,
        kc_length: int = 20,
        kc_scalar_wide: float = 2.0,
        kc_scalar_normal: float = 1.5,
        kc_scalar_narrow: float = 1.0,
        mom_length: int = 12,
        mom_smooth: int = 6,
    ) -> None:
        """Initialize this adapter and process the supplied input series.

        Parameters
        ----------
        high : object
            High-price series or the current bar high.
        low : object
            Low-price series or the current bar low.
        close : object
            Close-price series or the current bar close.
        bb_length : object
            Bollinger-band lookback in bars.
        bb_std : object
            Bollinger-band standard-deviation multiplier.
        kc_length : object
            Keltner-channel lookback in bars.
        kc_scalar_wide : object
            Wide Keltner multiplier.
        kc_scalar_normal : object
            Normal Keltner multiplier.
        kc_scalar_narrow : object
            Narrow Keltner multiplier.
        mom_length : object
            Momentum lookback in bars.
        mom_smooth : object
            Momentum smoothing length in bars.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
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

    def append(self, high: float, low: float, close: float) -> "SqueezePro":
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        high : object
            High-price series or the current bar high.
        low : object
            Low-price series or the current bar low.
        close : object
            Close-price series or the current bar close.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(high, low, close)
        return self

    def extend(self, high: Any, low: Any, close: Any) -> "SqueezePro":
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        high : object
            High-price series or the current bar high.
        low : object
            Low-price series or the current bar low.
        close : object
            Close-price series or the current bar close.

        Returns
        -------
        Self
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

    def reset(self) -> "SqueezePro":
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        return self
