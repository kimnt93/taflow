"""Compatibility re-exports for statistical indicator classes."""

from .indicators.mesa_adaptive_moving_average import MesaAdaptiveMovingAverage
from .indicators.rolling_average_deviation import RollingAverageDeviation
from .indicators.rolling_beta import RollingBeta
from .indicators.rolling_correlation import RollingCorrelation
from .indicators.rolling_linear_regression import RollingLinearRegression
from .indicators.rolling_linear_regression_angle import RollingLinearRegressionAngle
from .indicators.rolling_linear_regression_intercept import RollingLinearRegressionIntercept
from .indicators.rolling_linear_regression_slope import RollingLinearRegressionSlope
from .indicators.rolling_midpoint import RollingMidpoint
from .indicators.rolling_midprice import RollingMidprice
from .indicators.rolling_min_max import RollingMinMax
from .indicators.rolling_min_max_index import RollingMinMaxIndex
from .indicators.rolling_standard_deviation import RollingStandardDeviation
from .indicators.rolling_time_series_forecast import RollingTimeSeriesForecast
from .indicators.rolling_variance import RollingVariance

__all__ = [name for name in globals() if name.startswith(("Rolling", "Mesa"))]
