"""Bounded active-zone storage used by causal zone indicators."""

from ._native import ActiveZoneListOperator as _Native


class ActiveZoneList:
    """Stateful ActiveZoneList indicator.
    Parameters are documented by the constructor signature; scalar
    ``append`` returns the current value and ``compute`` returns
    the aligned history with NaN warm-up where applicable.
    """

    def __init__(self, capacity: int = 64):
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        capacity : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state = _Native(capacity)

    def add(self, top: float, bottom: float, flags: int = 0):
        """Execute the add operation through the native Rust implementation.

        Parameters
        ----------
        top : object
            Input series, scalar parameter, or configuration value for this operation.
        bottom : object
            Input series, scalar parameter, or configuration value for this operation.
        flags : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.add(top, bottom, flags)

    def advance(self, price: float, max_age: int | None = None):
        """Execute the advance operation through the native Rust implementation.

        Parameters
        ----------
        price : object
            Input series, scalar parameter, or configuration value for this operation.
        max_age : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.advance(price, max_age)

    @property
    def size(self):
        """Execute the size operation through the native Rust implementation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.size

    def reset(self):
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        return self
