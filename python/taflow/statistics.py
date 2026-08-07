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
        self._left: list[float] = []
        self._right: list[float] = []
        if _input is not None:
            self.extend(_input)

    def append(self, _input: float):
        value = self._state.append(float(_input))
        if value is None:
            left = right = np.nan
        else:
            left, right = value
        self._left.append(float(left)); self._right.append(float(right))
        return self

    def extend(self, _input: Any):
        for value in as_float64_series(_input):
            self.append(value)
        return self

    def compute(self):
        return (np.asarray(self._left), np.asarray(self._right))

    @property
    def value(self): return self._state.value

    def reset(self):
        self._state.reset(); self._left.clear(); self._right.clear(); return self


class RollingMinMax(_RollingPair): _native_cls = StatefulMinmax
class RollingMinMaxIndex(_RollingPair): _native_cls = StatefulMinmaxindex


class MesaAdaptiveMovingAverage:
    """Persistent MAMA/FAMA pair backed by the native Rust state."""

    def __init__(self, fastlimit: float = 0.5, slowlimit: float = 0.05,
                 _input: Any | None = None) -> None:
        self._state = StatefulMama(fastlimit, slowlimit)
        self._mama: list[float] = []
        self._fama: list[float] = []
        if _input is not None:
            self.extend(_input)

    def append(self, _input: float):
        value = self._state.append(float(_input))
        if value is None:
            self._mama.append(np.nan); self._fama.append(np.nan)
        else:
            self._mama.append(float(value[0])); self._fama.append(float(value[1]))
        return self

    def extend(self, _input: Any):
        for value in as_float64_series(_input):
            self.append(value)
        return self

    def compute(self):
        return (np.asarray(self._mama, dtype=np.float64),
                np.asarray(self._fama, dtype=np.float64))

    @property
    def value(self): return self._state.value

    def reset(self):
        self._state.reset(); self._mama.clear(); self._fama.clear(); return self


class _Bivariate:
    _native_cls = None

    def __init__(self, _input0: Any | None = None, _input1: Any | None = None,
                 timeperiod: int = 5) -> None:
        self._state = self._native_cls(timeperiod)
        self._values: list[float] = []
        if _input0 is not None and _input1 is not None:
            self.extend(_input0, _input1)

    def append(self, input0: float, input1: float):
        value = self._state.append(float(input0), float(input1))
        self._values.append(np.nan if value is None else float(value))
        return self

    def extend(self, input0: Any, input1: Any):
        a = as_float64_series(input0)
        b = as_float64_series(input1)
        if len(a) != len(b):
            raise ValueError("inputs must have equal lengths")
        for x, y in zip(a, b):
            self.append(x, y)
        return self

    def compute(self) -> np.ndarray:
        return np.asarray(self._values, dtype=np.float64)

    @property
    def value(self): return self._state.value

    def reset(self):
        self._state.reset(); self._values.clear(); return self


class RollingBeta(_Bivariate): _native_cls = StatefulBeta
class RollingCorrelation(_Bivariate): _native_cls = StatefulCorrel
class RollingMidprice(_Bivariate): _native_cls = StatefulMidprice


__all__ = [name for name in globals() if name.startswith(("Rolling", "Mesa"))]
