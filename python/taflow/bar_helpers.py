from typing import Any
import numpy as np
from ._series import as_float64_series


def _make(native, name):
    """Execute the _make operation through the native Rust implementation.

    Parameters
    ----------
    native : object
        Input series, scalar parameter, or configuration value for this operation.
    name : object
        Input series, scalar parameter, or configuration value for this operation.

    Returns
    -------
    object
        The updated adapter, native value, aligned output array, or execution node.
    """

    def init(self, high: Any | None = None, low: Any | None = None):
        """Execute the init operation through the native Rust implementation.

        Parameters
        ----------
        high : object
            Input series, scalar parameter, or configuration value for this operation.
        low : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state = native()
        self.extend(high, low) if high is not None or low is not None else None

    def append(self, high: float, low: float):
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        high : object
            Input series, scalar parameter, or configuration value for this operation.
        low : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(high, low)
        return self

    def extend(self, high: Any, low: Any):
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        high : object
            Input series, scalar parameter, or configuration value for this operation.
        low : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.extend(as_float64_series(high), as_float64_series(low))
        return self

    return type(
        name,
        (),
        {
            "__init__": init,
            "append": append,
            "extend": extend,
            "compute": lambda self: self._state.compute(),
            "value": property(lambda self: self._state.value),
            "reset": lambda self: (self._state.reset() or self),
        },
    )


from ._native import (
    HigherHighOperator,
    LowerLowOperator,
    InsideBarOperator,
    OutsideBarOperator,
    GapUpOperator,
    GapDownOperator,
)

HigherHigh = _make(HigherHighOperator, "HigherHigh")
LowerLow = _make(LowerLowOperator, "LowerLow")
InsideBar = _make(InsideBarOperator, "InsideBar")
OutsideBar = _make(OutsideBarOperator, "OutsideBar")
GapUp = _make(GapUpOperator, "GapUp")
GapDown = _make(GapDownOperator, "GapDown")
