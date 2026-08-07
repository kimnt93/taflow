from typing import Any
import numpy as np
from ._native import (
    BarsSinceOperator,
    ValueWhenOperator,
    HighestSinceOperator,
    LowestSinceOperator,
    SignalDelayOperator,
    PositionHoldOperator,
    EntryExitOperator,
)
from ._series import as_float64_series


class BarsSince:
    """Track the number of bars since a boolean condition was true.

    Parameters
    ----------
    condition : array-like, optional
        Initial boolean condition history.
    """

    def __init__(self, condition: Any | None = None) -> None:
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        condition : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state = BarsSinceOperator()
        self.extend(condition) if condition is not None else None

    def append(self, condition: bool) -> object:
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        condition : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(condition)
        return self

    def extend(self, condition: Any) -> object:
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        condition : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.extend(np.asarray(condition, dtype=bool))
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
    def value(self) -> object:
        """Return the latest computed value, or None during warm-up.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.value

    def reset(self) -> object:
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        return self


def _make(native: object, name: object) -> object:
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

    class State:
        """Adapt a native two-input state helper to the Python lifecycle API."""

        def __init__(
            self, condition: Any | None = None, _input: Any | None = None
        ) -> None:
            """Initialize this adapter and optionally process the supplied input series.

            Parameters
            ----------
            condition : object
                Input series, scalar parameter, or configuration value for this operation.
            _input : object
                Input series, scalar parameter, or configuration value for this operation.

            Returns
            -------
            object
                The updated adapter, native value, aligned output array, or execution node.
            """
            self._state = native()
            (
                self.extend(condition, _input)
                if condition is not None or _input is not None
                else None
            )

        def append(self, condition: bool, _input: float) -> object:
            """Append one observation or aligned bar to the native Rust state.

            Parameters
            ----------
            condition : object
                Input series, scalar parameter, or configuration value for this operation.
            _input : object
                Input series, scalar parameter, or configuration value for this operation.

            Returns
            -------
            object
                The updated adapter, native value, aligned output array, or execution node.
            """
            self._state.append(condition, _input)
            return self

        def extend(self, condition: Any, _input: Any) -> object:
            """Append aligned input series to the native Rust state.

            Parameters
            ----------
            condition : object
                Input series, scalar parameter, or configuration value for this operation.
            _input : object
                Input series, scalar parameter, or configuration value for this operation.

            Returns
            -------
            object
                The updated adapter, native value, aligned output array, or execution node.
            """
            self._state.extend(
                np.asarray(condition, dtype=bool), as_float64_series(_input)
            )
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
        def value(self) -> object:
            """Return the latest computed value, or None during warm-up.

            Returns
            -------
            object
                The updated adapter, native value, aligned output array, or execution node.
            """
            return self._state.value

        def reset(self) -> object:
            """Execute the reset operation through the native Rust implementation.

            Returns
            -------
            object
                The updated adapter, native value, aligned output array, or execution node.
            """
            self._state.reset()
            return self

    State.__name__ = name
    return State


ValueWhen = _make(ValueWhenOperator, "ValueWhen")
HighestSince = _make(HighestSinceOperator, "HighestSince")
LowestSince = _make(LowestSinceOperator, "LowestSince")


class SignalDelay:
    """Delay a scalar series by a fixed causal number of bars.

    Parameters
    ----------
    timeperiod : int
        Number of bars to delay.
    _input : array-like, optional
        Initial input history.
    """

    def __init__(self, timeperiod: int, _input: Any | None = None) -> None:
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        timeperiod : object
            Input series, scalar parameter, or configuration value for this operation.
        _input : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state = SignalDelayOperator(timeperiod)
        self.extend(_input) if _input is not None else None

    def append(self, _input: float) -> object:
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        _input : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(_input)
        return self

    def extend(self, _input: Any) -> object:
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        _input : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.extend(as_float64_series(_input))
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
    def value(self) -> object:
        """Return the latest computed value, or None during warm-up.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.value

    def reset(self) -> object:
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        return self


class PositionHold:
    """Hold the most recently supplied position value.

    Parameters
    ----------
    _input : array-like, optional
        Initial position history.
    """

    def __init__(self, _input: Any | None = None) -> None:
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        _input : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state = PositionHoldOperator()
        self.extend(_input) if _input is not None else None

    def append(self, _input: float) -> object:
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        _input : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(_input)
        return self

    def extend(self, _input: Any) -> object:
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        _input : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.extend(as_float64_series(_input))
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
    def value(self) -> object:
        """Return the latest computed value, or None during warm-up.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.value

    def reset(self) -> object:
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        return self


class EntryExit:
    """Track entry and exit events as a stateful position signal.

    Parameters
    ----------
    entry : array-like, optional
        Initial entry-event history.
    exit : array-like, optional
        Initial exit-event history.
    """

    def __init__(self, entry: Any | None = None, exit: Any | None = None) -> None:
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        entry : object
            Input series, scalar parameter, or configuration value for this operation.
        exit : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state = EntryExitOperator()
        self.extend(entry, exit) if entry is not None or exit is not None else None

    def append(self, entry: bool, exit: bool) -> object:
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        entry : object
            Input series, scalar parameter, or configuration value for this operation.
        exit : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(entry, exit)
        return self

    def extend(self, entry: Any, exit: Any) -> object:
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        entry : object
            Input series, scalar parameter, or configuration value for this operation.
        exit : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.extend(np.asarray(entry, dtype=bool), np.asarray(exit, dtype=bool))
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
    def value(self) -> object:
        """Return the latest computed value, or None during warm-up.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.value

    def reset(self) -> object:
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        return self
