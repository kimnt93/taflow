"""Persistent Know Sure Thing (bukosabino `ta` alignment)."""

from typing import Any
import numpy as np
from ._native import KnowSureThingOperator as _Native
from ._series import as_float64_series


class KnowSureThing:
    """Persistent Know Sure Thing (bukosabino `ta` alignment).

    This public class owns a persistent native Rust state; Python performs container conversion only. `append`, `extend`, and `reset` are fluent, `value` exposes the latest result, and `compute` returns aligned history. Required input histories: `close`. Warm-up positions are represented by `NaN` in history."""

    def __init__(
        self,
        close: Any,
        roc1: int = 10,
        roc2: int = 15,
        roc3: int = 20,
        roc4: int = 30,
        sma1: int = 10,
        sma2: int = 10,
        sma3: int = 10,
        sma4: int = 15,
        signal: int = 9,
    ) -> None:
        """Initialize this adapter and process the supplied input series.

        Parameters
        ----------
        close : object
            Close-price series or the current bar close.
        roc1 : object
            First rate-of-change period.
        roc2 : object
            Second rate-of-change period.
        roc3 : object
            Third rate-of-change period.
        roc4 : object
            Fourth rate-of-change period.
        sma1 : object
            First SMA smoothing period.
        sma2 : object
            Second SMA smoothing period.
        sma3 : object
            Third SMA smoothing period.
        sma4 : object
            Fourth SMA smoothing period.
        signal : object
            Signal smoothing period in bars.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = _Native(roc1, roc2, roc3, roc4, sma1, sma2, sma3, sma4, signal)
        self.extend(close) if close is not None else None

    def append(self, close: float) -> "KnowSureThing":
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        close : object
            Close-price series or the current bar close.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(close)
        return self

    def extend(self, close: Any) -> "KnowSureThing":
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        close : object
            Close-price series or the current bar close.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.extend(as_float64_series(close))
        return self

    def compute(self) -> tuple[np.ndarray, np.ndarray]:
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

    def reset(self) -> "KnowSureThing":
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        return self
