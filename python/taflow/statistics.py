"""Canonical Python adapters for TA-Lib statistical series functions."""

from typing import Any

import numpy as np

from ._native import (
    StatefulAvgdev, StatefulBeta, StatefulCorrel, StatefulLinearreg,
    StatefulLinearregAngle, StatefulLinearregIntercept, StatefulLinearregSlope,
    StatefulMidpoint, StatefulMidprice, StatefulStddev, StatefulTsf,
    StatefulVar, StatefulMama,
    StatefulMinmax, StatefulMinmaxindex,
)
from ._series import as_float64_series
from ._unary_state import UnaryStateAdapter


class RollingAverageDeviation(UnaryStateAdapter): _native_cls = StatefulAvgdev
class RollingMidpoint(UnaryStateAdapter): _native_cls = StatefulMidpoint
class RollingStandardDeviation(UnaryStateAdapter): _native_cls = StatefulStddev
class RollingVariance(UnaryStateAdapter): _native_cls = StatefulVar
class RollingLinearRegression(UnaryStateAdapter): _native_cls = StatefulLinearreg
class RollingLinearRegressionAngle(UnaryStateAdapter): _native_cls = StatefulLinearregAngle
class RollingLinearRegressionIntercept(UnaryStateAdapter): _native_cls = StatefulLinearregIntercept
class RollingLinearRegressionSlope(UnaryStateAdapter): _native_cls = StatefulLinearregSlope
class RollingTimeSeriesForecast(UnaryStateAdapter): _native_cls = StatefulTsf


class _RollingPair(UnaryStateAdapter):
    """Native rolling pair with two aligned output series."""

    def __init__(self, _input: Any | None = None, timeperiod: int = 30) -> None:
        self._state = self._native_cls(timeperiod)
        if _input is not None:
            self.extend(_input)

    def append(self, _input: float):
        self._state.append(float(_input))
        return self

    def extend(self, _input: Any):
        self._state.extend(as_float64_series(_input))
        return self

    def compute(self):
        return self._state.compute()

    @property
    def value(self): return self._state.value

    def __len__(self) -> int:
        return len(self._state)

    def reset(self):
        self._state.reset(); return self


class RollingMinMax(_RollingPair): _native_cls = StatefulMinmax
class RollingMinMaxIndex(_RollingPair): _native_cls = StatefulMinmaxindex


class MesaAdaptiveMovingAverage:
    """Persistent MAMA/FAMA pair backed by the native Rust state."""

    def __init__(self, fastlimit: float = 0.5, slowlimit: float = 0.05,
                 _input: Any | None = None) -> None:
        self._state = StatefulMama(fastlimit, slowlimit)
        if _input is not None:
            self.extend(_input)

    def append(self, _input: float):
        self._state.append(float(_input))
        return self

    def extend(self, _input: Any):
        self._state.extend(as_float64_series(_input))
        return self

    def compute(self):
        return self._state.compute()

    @property
    def value(self): return self._state.value

    def __len__(self) -> int:
        return len(self._state)

    def reset(self):
        self._state.reset(); return self


class _Bivariate:
    _native_cls = None

    def __init__(self, _input0: Any | None = None, _input1: Any | None = None,
                 timeperiod: int = 5) -> None:
        self._state = self._native_cls(timeperiod)
        if _input0 is not None and _input1 is not None:
            self.extend(_input0, _input1)

    def append(self, input0: float, input1: float):
        self._state.append(float(input0), float(input1))
        return self

    def extend(self, input0: Any, input1: Any):
        a = as_float64_series(input0)
        b = as_float64_series(input1)
        if len(a) != len(b):
            raise ValueError("inputs must have equal lengths")
        self._state.extend(a, b)
        return self

    def compute(self) -> np.ndarray:
        return self._state.compute()

    @property
    def value(self): return self._state.value

    def __len__(self) -> int:
        return len(self._state)

    def reset(self):
        self._state.reset(); return self


class RollingBeta(_Bivariate): _native_cls = StatefulBeta
class RollingCorrelation(_Bivariate): _native_cls = StatefulCorrel
class RollingMidprice(_Bivariate): _native_cls = StatefulMidprice


__all__ = [name for name in globals() if name.startswith(("Rolling", "Mesa"))]
