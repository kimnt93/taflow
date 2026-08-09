"""Bounded active-zone storage used by causal zone indicators."""

from numpy.typing import NDArray

from ._native import ActiveZoneListOperator as _Native


class ActiveZoneList:
    """Bounded storage primitive shared by causal zone indicators.

    This is infrastructure rather than a technical indicator. Zones are
    retained up to ``capacity`` and can be invalidated by price or age.
    """

    def __init__(self, capacity: int = 64) -> None:
        """Initialize this adapter and process the supplied input series.

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

    def add(self, top: float, bottom: float, flags: int = 0) -> int:
        """Store a zone and return its internal index.

        Parameters
        ----------
        top : object
            Upper zone boundary.
        bottom : object
            Lower zone boundary.
        flags : object
            Integer zone flags.

        Returns
        -------
        int
            Index assigned to the stored zone.
        """
        return self._state.add(top, bottom, flags)

    def advance(self, price: float, max_age: int | None = None) -> NDArray:
        """Advance one bar and return flags for zones invalidated on this bar.

        Parameters
        ----------
        price : object
            Price series or the current price observation.
        max_age : object
            Maximum age retained for a zone.

        Returns
        -------
        numpy.ndarray
            Boolean invalidation flags aligned to the zones present before advancing.
        """
        return self._state.advance(price, max_age)

    @property
    def size(self) -> int:
        """Return the number of active zones.

        Returns
        -------
        int
            Current active-zone count.
        """
        return self._state.size

    def reset(self) -> "ActiveZoneList":
        """Clear all zones and return this container.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        return self
