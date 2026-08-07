"""Descriptive stateful interface for the Intraday Momentum Index."""

from taflow._native import StatefulImi
from typing import Any


class IntradayMomentumIndex:
    """Incrementally compare rolling intraday candle gains and losses."""

    def __init__(self, period: int = 14, open: Any | None = None,
                 close: Any | None = None):
        """Create IMI with an optional aligned open/close history."""
        self._state = StatefulImi(period)
        if open is not None or close is not None:
            self.extend(open, close)

    def append(self, open, close):
        return self._state.append(open, close)

    def extend(self, open, close):
        return self._state.extend(open, close)

    @property
    def value(self):
        return self._state.value

    def reset(self):
        self._state.reset()
