"""Online Kalman estimate of the hedge ratio in ``y = alpha + beta*x``."""

from typing import Any
import numpy as np
from ._native import KalmanHedgeRatioOperator as _Native
from ._series import as_float64_series


class KalmanHedgeRatio:
    """Stateful KalmanHedgeRatio indicator.
    Parameters are documented by the constructor signature; scalar
    ``append`` returns the current value and ``compute`` returns
    the aligned history with NaN warm-up where applicable.
    """

    def __init__(
        self,
        x: Any | None = None,
        y: Any | None = None,
        delta: float = 1e-4,
        observation_variance: float = 1e-3,
    ) -> None:
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        x : object
            First aligned input series or scalar observation.
        y : object
            Second aligned input series or scalar observation.
        delta : object
            Kalman process-noise parameter.
        observation_variance : object
            Kalman observation-noise parameter.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = _Native(delta, observation_variance)
        if x is not None or y is not None:
            self.extend(x, y)

    def append(self, x: float, y: float) -> object:
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        x : object
            First aligned input series or scalar observation.
        y : object
            Second aligned input series or scalar observation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(x, y)
        return self

    def extend(self, x: Any, y: Any) -> object:
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        x : object
            First aligned input series or scalar observation.
        y : object
            Second aligned input series or scalar observation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.extend(as_float64_series(x), as_float64_series(y))
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

    @property
    def alpha(self) -> object:
        """Execute the alpha operation through the native Rust implementation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.alpha

    @property
    def innovation(self) -> object:
        """Execute the innovation operation through the native Rust implementation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.innovation

    @property
    def std(self) -> object:
        """Execute the std operation through the native Rust implementation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.std

    def reset(self) -> object:
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        return self
