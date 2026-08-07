"""Bounded active-zone storage used by causal zone indicators."""

from ._native import ActiveZoneListOperator as _Native


class ActiveZoneList:
    """Stateful ActiveZoneList indicator.
    Parameters are documented by the constructor signature; scalar
    ``append`` returns the current value and ``compute`` returns
    the aligned history with NaN warm-up where applicable.
    """

    def __init__(self, capacity: int = 64) -> None:
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        capacity : object
            Maximum number of stored zones.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = _Native(capacity)

    def add(self, top: float, bottom: float, flags: int = 0) -> object:
        """Execute the add operation through the native Rust implementation.

        Parameters
        ----------
        top : object
            Values or parameters consumed by this operation.
        bottom : object
            Values or parameters consumed by this operation.
        flags : object
            Values or parameters consumed by this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.add(top, bottom, flags)

    def advance(self, price: float, max_age: int | None = None) -> object:
        """Execute the advance operation through the native Rust implementation.

        Parameters
        ----------
        price : object
            Price series or the current price observation.
        max_age : object
            Values or parameters consumed by this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.advance(price, max_age)

    @property
    def size(self) -> object:
        """Execute the size operation through the native Rust implementation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.size

    def reset(self) -> object:
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        return self
