"""Descriptive stateful interface for the Intraday Momentum Index."""

from taflow._native import StatefulImi


class IntradayMomentumIndex:
    """Incrementally compare rolling intraday candle gains and losses."""

    def __init__(self, period=14):
        self._state = StatefulImi(period)

    def append(self, open, close):
        return self._state.append(open, close)

    def extend(self, open, close):
        return self._state.extend(open, close)

    @property
    def value(self):
        return self._state.value

    def reset(self):
        self._state.reset()
