"""Compatibility re-exports for statistical indicator classes."""

from .indicators.mesa_adaptive_moving_average import MesaAdaptiveMovingAverage
from .rolling_average_deviation import RollingAverageDeviation
from .rolling_beta import RollingBeta
from .rolling_correlation import RollingCorrelation
from .rolling_linear_regression import RollingLinearRegression
from .rolling_linear_regression_angle import RollingLinearRegressionAngle
from .rolling_linear_regression_intercept import RollingLinearRegressionIntercept
from .rolling_linear_regression_slope import RollingLinearRegressionSlope
from .rolling_midpoint import RollingMidpoint
from .rolling_midprice import RollingMidprice
from .indicators.rolling_min_max import RollingMinMax
from .indicators.rolling_min_max_index import RollingMinMaxIndex
from .rolling_standard_deviation import RollingStandardDeviation
from .rolling_time_series_forecast import RollingTimeSeriesForecast
from .rolling_variance import RollingVariance

__all__ = [name for name in globals() if name.startswith(("Rolling", "Mesa"))]
