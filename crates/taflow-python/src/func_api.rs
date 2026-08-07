use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow as core;

use crate::conversion::to_py_array;

// 辅助宏: 将 TaError 转为 PyValueError
macro_rules! ta_err {
    ($expr:expr) => {
        $expr.map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    };
}

// ============================================================
// Overlap Studies
// ============================================================

#[pyfunction]
#[pyo3(signature = (high, low, close, timeperiod=20))]
/// Computes or updates `ACCBANDS` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn ACCBANDS(
    py: Python<'_>,
    high: PyReadonlyArray1<f64>,
    low: PyReadonlyArray1<f64>,
    close: PyReadonlyArray1<f64>,
    timeperiod: usize,
) -> PyResult<(Py<PyArray1<f64>>, Py<PyArray1<f64>>, Py<PyArray1<f64>>)> {
    let (upper, middle, lower) = ta_err!(core::stream::acceleration_bands(
        high.as_slice()?,
        low.as_slice()?,
        close.as_slice()?,
        timeperiod,
    ))?;
    Ok((
        to_py_array(py, upper),
        to_py_array(py, middle),
        to_py_array(py, lower),
    ))
}

#[pyfunction]
#[pyo3(signature = (_input, timeperiod=30))]
/// Computes or updates `SMA` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn SMA(
    py: Python<'_>,
    _input: PyReadonlyArray1<f64>,
    timeperiod: usize,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::stream::simple_moving_average(_input.as_slice()?, timeperiod))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (_input, timeperiod=30))]
/// Computes or updates `EMA` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn EMA(
    py: Python<'_>,
    _input: PyReadonlyArray1<f64>,
    timeperiod: usize,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::stream::exponential_moving_average(_input.as_slice()?, timeperiod))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (_input, timeperiod=30))]
/// Computes or updates `WMA` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn WMA(
    py: Python<'_>,
    _input: PyReadonlyArray1<f64>,
    timeperiod: usize,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::stream::weighted_moving_average(_input.as_slice()?, timeperiod))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (_input, timeperiod=30))]
/// Computes or updates `DEMA` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn DEMA(
    py: Python<'_>,
    _input: PyReadonlyArray1<f64>,
    timeperiod: usize,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::stream::double_exponential_moving_average(_input.as_slice()?, timeperiod))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (_input, timeperiod=30))]
/// Computes or updates `TEMA` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn TEMA(
    py: Python<'_>,
    _input: PyReadonlyArray1<f64>,
    timeperiod: usize,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::stream::triple_exponential_moving_average(_input.as_slice()?, timeperiod))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (_input, timeperiod=30))]
/// Computes or updates `TRIMA` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn TRIMA(
    py: Python<'_>,
    _input: PyReadonlyArray1<f64>,
    timeperiod: usize,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::stream::triangular_moving_average(_input.as_slice()?, timeperiod))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (_input, timeperiod=30))]
/// Computes or updates `KAMA` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn KAMA(
    py: Python<'_>,
    _input: PyReadonlyArray1<f64>,
    timeperiod: usize,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::stream::kaufman_adaptive_moving_average(_input.as_slice()?, timeperiod))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (_input, timeperiod=5, vfactor=0.7))]
/// Computes or updates `TripleExponentialAverage` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn TripleExponentialAverage(
    py: Python<'_>,
    _input: PyReadonlyArray1<f64>,
    timeperiod: usize,
    vfactor: f64,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::stream::triple_exponential_average(_input.as_slice()?, timeperiod, vfactor))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (_input, fastlimit=0.5, slowlimit=0.05))]
/// Computes or updates `MAMA` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn MAMA(
    py: Python<'_>,
    _input: PyReadonlyArray1<f64>,
    fastlimit: f64,
    slowlimit: f64,
) -> PyResult<(Py<PyArray1<f64>>, Py<PyArray1<f64>>)> {
    let (mama, fama) = ta_err!(core::overlap::mesa_adaptive_moving_average(_input.as_slice()?, fastlimit, slowlimit))?;
    Ok((to_py_array(py, mama), to_py_array(py, fama)))
}

#[pyfunction]
#[pyo3(signature = (_input, timeperiod=5, nbdevup=2.0, nbdevdn=2.0, matype=0))]
/// Computes or updates `BBANDS` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn BBANDS(
    py: Python<'_>,
    _input: PyReadonlyArray1<f64>,
    timeperiod: usize,
    nbdevup: f64,
    nbdevdn: f64,
    matype: i32,
) -> PyResult<(Py<PyArray1<f64>>, Py<PyArray1<f64>>, Py<PyArray1<f64>>)> {
    let ma = core::MaType::try_from(matype)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    let (upper, middle, lower) = ta_err!(core::stream::bollinger_bands(
        _input.as_slice()?,
        timeperiod,
        nbdevup,
        nbdevdn,
        ma
    ))?;
    Ok((
        to_py_array(py, upper),
        to_py_array(py, middle),
        to_py_array(py, lower),
    ))
}

#[pyfunction]
#[pyo3(signature = (high, low, acceleration=0.02, maximum=0.2))]
/// Computes or updates `SAR` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn SAR(
    py: Python<'_>,
    high: PyReadonlyArray1<f64>,
    low: PyReadonlyArray1<f64>,
    acceleration: f64,
    maximum: f64,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::stream::parabolic_sar(
        high.as_slice()?,
        low.as_slice()?,
        acceleration,
        maximum
    ))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (high, low, startvalue=0.0, offsetonreverse=0.0, accelerationinitlong=0.02, accelerationlong=0.02, accelerationmaxlong=0.2, accelerationinitshort=0.02, accelerationshort=0.02, accelerationmaxshort=0.2))]
/// Computes or updates `SAREXT` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn SAREXT(
    py: Python<'_>,
    high: PyReadonlyArray1<f64>,
    low: PyReadonlyArray1<f64>,
    startvalue: f64,
    offsetonreverse: f64,
    accelerationinitlong: f64,
    accelerationlong: f64,
    accelerationmaxlong: f64,
    accelerationinitshort: f64,
    accelerationshort: f64,
    accelerationmaxshort: f64,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::stream::extended_parabolic_sar(
        high.as_slice()?,
        low.as_slice()?,
        startvalue,
        offsetonreverse,
        accelerationinitlong,
        accelerationlong,
        accelerationmaxlong,
        accelerationinitshort,
        accelerationshort,
        accelerationmaxshort
    ))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (_input, timeperiod=14))]
/// Computes or updates `MIDPOINT` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn MIDPOINT(
    py: Python<'_>,
    _input: PyReadonlyArray1<f64>,
    timeperiod: usize,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::overlap::midpoint(_input.as_slice()?, timeperiod))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (high, low, timeperiod=14))]
/// Computes or updates `MIDPRICE` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn MIDPRICE(
    py: Python<'_>,
    high: PyReadonlyArray1<f64>,
    low: PyReadonlyArray1<f64>,
    timeperiod: usize,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::overlap::midprice(
        high.as_slice()?,
        low.as_slice()?,
        timeperiod
    ))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (_input, periods, minperiod=2, maxperiod=30, matype=0))]
/// Computes or updates `MAVP` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn MAVP(
    py: Python<'_>,
    _input: PyReadonlyArray1<f64>,
    periods: PyReadonlyArray1<f64>,
    minperiod: usize,
    maxperiod: usize,
    matype: i32,
) -> PyResult<Py<PyArray1<f64>>> {
    let ma = core::MaType::try_from(matype)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    let result = ta_err!(core::stream::moving_average_variable_period(
        _input.as_slice()?,
        periods.as_slice()?,
        minperiod,
        maxperiod,
        ma
    ))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
/// Computes or updates `HT_TRENDLINE` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn HT_TRENDLINE(py: Python<'_>, _input: PyReadonlyArray1<f64>) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::overlap::hilbert_transform_trendline(_input.as_slice()?))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (_input, timeperiod=30, matype=0))]
/// Computes or updates `MA` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn MA(
    py: Python<'_>,
    _input: PyReadonlyArray1<f64>,
    timeperiod: usize,
    matype: i32,
) -> PyResult<Py<PyArray1<f64>>> {
    let ma = core::MaType::try_from(matype)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    let result = ta_err!(core::overlap::moving_average(_input.as_slice()?, timeperiod, ma))?;
    Ok(to_py_array(py, result))
}

// ============================================================
// Momentum Indicators
// ============================================================

#[pyfunction]
#[pyo3(signature = (_open, close, timeperiod=14))]
/// Computes or updates `IMI` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn IMI(
    py: Python<'_>,
    _open: PyReadonlyArray1<f64>,
    close: PyReadonlyArray1<f64>,
    timeperiod: usize,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::momentum::intraday_momentum_index(
        _open.as_slice()?,
        close.as_slice()?,
        timeperiod
    ))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (_input, timeperiod=14))]
/// Computes or updates `RSI` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn RSI(
    py: Python<'_>,
    _input: PyReadonlyArray1<f64>,
    timeperiod: usize,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::stream::relative_strength_index(_input.as_slice()?, timeperiod))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (_input, fastperiod=12, slowperiod=26, signalperiod=9))]
/// Computes or updates `MACD` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn MACD(
    py: Python<'_>,
    _input: PyReadonlyArray1<f64>,
    fastperiod: usize,
    slowperiod: usize,
    signalperiod: usize,
) -> PyResult<(Py<PyArray1<f64>>, Py<PyArray1<f64>>, Py<PyArray1<f64>>)> {
    let (m, s, h) = ta_err!(core::momentum::moving_average_convergence_divergence(
        _input.as_slice()?,
        fastperiod,
        slowperiod,
        signalperiod
    ))?;
    Ok((to_py_array(py, m), to_py_array(py, s), to_py_array(py, h)))
}

#[pyfunction]
#[pyo3(signature = (_input, fastperiod=12, fastmatype=1, slowperiod=26, slowmatype=1, signalperiod=9, signalmatype=1))]
/// Computes or updates `MACDEXT` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn MACDEXT(
    py: Python<'_>,
    _input: PyReadonlyArray1<f64>,
    fastperiod: usize,
    fastmatype: i32,
    slowperiod: usize,
    slowmatype: i32,
    signalperiod: usize,
    signalmatype: i32,
) -> PyResult<(Py<PyArray1<f64>>, Py<PyArray1<f64>>, Py<PyArray1<f64>>)> {
    let fmt = core::MaType::try_from(fastmatype)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    let smt = core::MaType::try_from(slowmatype)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    let sigmt = core::MaType::try_from(signalmatype)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    let (m, s, h) = ta_err!(core::momentum::moving_average_convergence_divergence_extended(
        _input.as_slice()?,
        fastperiod,
        fmt,
        slowperiod,
        smt,
        signalperiod,
        sigmt
    ))?;
    Ok((to_py_array(py, m), to_py_array(py, s), to_py_array(py, h)))
}

#[pyfunction]
#[pyo3(signature = (_input, signalperiod=9))]
/// Computes or updates `MACDFIX` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn MACDFIX(
    py: Python<'_>,
    _input: PyReadonlyArray1<f64>,
    signalperiod: usize,
) -> PyResult<(Py<PyArray1<f64>>, Py<PyArray1<f64>>, Py<PyArray1<f64>>)> {
    let (m, s, h) = ta_err!(core::momentum::moving_average_convergence_divergence_fixed(_input.as_slice()?, signalperiod))?;
    Ok((to_py_array(py, m), to_py_array(py, s), to_py_array(py, h)))
}

#[pyfunction]
#[pyo3(signature = (high, low, close, fastk_period=5, slowk_period=3, slowk_matype=0, slowd_period=3, slowd_matype=0))]
/// Computes or updates `STOCH` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn STOCH(
    py: Python<'_>,
    high: PyReadonlyArray1<f64>,
    low: PyReadonlyArray1<f64>,
    close: PyReadonlyArray1<f64>,
    fastk_period: usize,
    slowk_period: usize,
    slowk_matype: i32,
    slowd_period: usize,
    slowd_matype: i32,
) -> PyResult<(Py<PyArray1<f64>>, Py<PyArray1<f64>>)> {
    let skm = core::MaType::try_from(slowk_matype)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    let sdm = core::MaType::try_from(slowd_matype)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    let (k, d) = ta_err!(core::momentum::stochastic_oscillator(
        high.as_slice()?,
        low.as_slice()?,
        close.as_slice()?,
        fastk_period,
        slowk_period,
        skm,
        slowd_period,
        sdm
    ))?;
    Ok((to_py_array(py, k), to_py_array(py, d)))
}

#[pyfunction]
#[pyo3(signature = (high, low, close, fastk_period=5, fastd_period=3, fastd_matype=0))]
/// Computes or updates `STOCHF` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn STOCHF(
    py: Python<'_>,
    high: PyReadonlyArray1<f64>,
    low: PyReadonlyArray1<f64>,
    close: PyReadonlyArray1<f64>,
    fastk_period: usize,
    fastd_period: usize,
    fastd_matype: i32,
) -> PyResult<(Py<PyArray1<f64>>, Py<PyArray1<f64>>)> {
    let fdm = core::MaType::try_from(fastd_matype)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    let (k, d) = ta_err!(core::momentum::fast_stochastic_oscillator(
        high.as_slice()?,
        low.as_slice()?,
        close.as_slice()?,
        fastk_period,
        fastd_period,
        fdm
    ))?;
    Ok((to_py_array(py, k), to_py_array(py, d)))
}

#[pyfunction]
#[pyo3(signature = (_input, timeperiod=14, fastk_period=5, fastd_period=3, fastd_matype=0))]
/// Computes or updates `STOCHRSI` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn STOCHRSI(
    py: Python<'_>,
    _input: PyReadonlyArray1<f64>,
    timeperiod: usize,
    fastk_period: usize,
    fastd_period: usize,
    fastd_matype: i32,
) -> PyResult<(Py<PyArray1<f64>>, Py<PyArray1<f64>>)> {
    let fdm = core::MaType::try_from(fastd_matype)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    let (k, d) = ta_err!(core::momentum::stochastic_relative_strength_index(
        _input.as_slice()?,
        timeperiod,
        fastk_period,
        fastd_period,
        fdm
    ))?;
    Ok((to_py_array(py, k), to_py_array(py, d)))
}

#[pyfunction]
#[pyo3(signature = (high, low, close, timeperiod=14))]
/// Computes or updates `ADX` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn ADX(
    py: Python<'_>,
    high: PyReadonlyArray1<f64>,
    low: PyReadonlyArray1<f64>,
    close: PyReadonlyArray1<f64>,
    timeperiod: usize,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::momentum::average_directional_index(
        high.as_slice()?,
        low.as_slice()?,
        close.as_slice()?,
        timeperiod
    ))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (high, low, close, timeperiod=14))]
/// Computes or updates `ADXR` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn ADXR(
    py: Python<'_>,
    high: PyReadonlyArray1<f64>,
    low: PyReadonlyArray1<f64>,
    close: PyReadonlyArray1<f64>,
    timeperiod: usize,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::momentum::average_directional_index_rating(
        high.as_slice()?,
        low.as_slice()?,
        close.as_slice()?,
        timeperiod
    ))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (high, low, close, timeperiod=14))]
/// Computes or updates `CCI` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn CCI(
    py: Python<'_>,
    high: PyReadonlyArray1<f64>,
    low: PyReadonlyArray1<f64>,
    close: PyReadonlyArray1<f64>,
    timeperiod: usize,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::stream::commodity_channel_index(
        high.as_slice()?,
        low.as_slice()?,
        close.as_slice()?,
        timeperiod
    ))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (_input, timeperiod=10))]
/// Computes or updates `MOM` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn MOM(
    py: Python<'_>,
    _input: PyReadonlyArray1<f64>,
    timeperiod: usize,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::stream::momentum(_input.as_slice()?, timeperiod))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (_input, timeperiod=10))]
/// Computes or updates `ROC` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn ROC(
    py: Python<'_>,
    _input: PyReadonlyArray1<f64>,
    timeperiod: usize,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::stream::rate_of_change(_input.as_slice()?, timeperiod))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (_input, timeperiod=10))]
/// Computes or updates `ROCP` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn ROCP(
    py: Python<'_>,
    _input: PyReadonlyArray1<f64>,
    timeperiod: usize,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::stream::rate_of_change_percent(_input.as_slice()?, timeperiod))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (_input, timeperiod=10))]
/// Computes or updates `ROCR` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn ROCR(
    py: Python<'_>,
    _input: PyReadonlyArray1<f64>,
    timeperiod: usize,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::stream::rate_of_change_ratio(_input.as_slice()?, timeperiod))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (_input, timeperiod=10))]
/// Computes or updates `ROCR100` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn ROCR100(
    py: Python<'_>,
    _input: PyReadonlyArray1<f64>,
    timeperiod: usize,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::stream::rate_of_change_ratio_percent(_input.as_slice()?, timeperiod))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (high, low, close, timeperiod=14))]
/// Computes or updates `WILLR` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn WILLR(
    py: Python<'_>,
    high: PyReadonlyArray1<f64>,
    low: PyReadonlyArray1<f64>,
    close: PyReadonlyArray1<f64>,
    timeperiod: usize,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::stream::williams_r(
        high.as_slice()?,
        low.as_slice()?,
        close.as_slice()?,
        timeperiod
    ))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (_input, fastperiod=12, slowperiod=26, matype=0))]
/// Computes or updates `APO` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn APO(
    py: Python<'_>,
    _input: PyReadonlyArray1<f64>,
    fastperiod: usize,
    slowperiod: usize,
    matype: i32,
) -> PyResult<Py<PyArray1<f64>>> {
    let ma = core::MaType::try_from(matype)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    let result = ta_err!(core::stream::absolute_price_oscillator(
        _input.as_slice()?,
        fastperiod,
        slowperiod,
        ma
    ))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (_input, fastperiod=12, slowperiod=26, matype=0))]
/// Computes or updates `PPO` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn PPO(
    py: Python<'_>,
    _input: PyReadonlyArray1<f64>,
    fastperiod: usize,
    slowperiod: usize,
    matype: i32,
) -> PyResult<Py<PyArray1<f64>>> {
    let ma = core::MaType::try_from(matype)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    let result = ta_err!(core::stream::percentage_price_oscillator(
        _input.as_slice()?,
        fastperiod,
        slowperiod,
        ma
    ))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
/// Computes or updates `BOP` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn BOP(
    py: Python<'_>,
    _open: PyReadonlyArray1<f64>,
    high: PyReadonlyArray1<f64>,
    low: PyReadonlyArray1<f64>,
    close: PyReadonlyArray1<f64>,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::stream::balance_of_power(
        _open.as_slice()?,
        high.as_slice()?,
        low.as_slice()?,
        close.as_slice()?
    ))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (_input, timeperiod=14))]
/// Computes or updates `CMO` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn CMO(
    py: Python<'_>,
    _input: PyReadonlyArray1<f64>,
    timeperiod: usize,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::stream::chande_momentum_oscillator(_input.as_slice()?, timeperiod))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (high, low, timeperiod=14))]
/// Computes or updates `AROON` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn AROON(
    py: Python<'_>,
    high: PyReadonlyArray1<f64>,
    low: PyReadonlyArray1<f64>,
    timeperiod: usize,
) -> PyResult<(Py<PyArray1<f64>>, Py<PyArray1<f64>>)> {
    let (down, up) = ta_err!(core::momentum::aroon(
        high.as_slice()?,
        low.as_slice()?,
        timeperiod
    ))?;
    Ok((to_py_array(py, down), to_py_array(py, up)))
}

#[pyfunction]
#[pyo3(signature = (high, low, timeperiod=14))]
/// Computes or updates `AROONOSC` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn AROONOSC(
    py: Python<'_>,
    high: PyReadonlyArray1<f64>,
    low: PyReadonlyArray1<f64>,
    timeperiod: usize,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::momentum::aroon_oscillator(
        high.as_slice()?,
        low.as_slice()?,
        timeperiod
    ))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (high, low, close, volume, timeperiod=14))]
/// Computes or updates `MFI` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn MFI(
    py: Python<'_>,
    high: PyReadonlyArray1<f64>,
    low: PyReadonlyArray1<f64>,
    close: PyReadonlyArray1<f64>,
    volume: PyReadonlyArray1<f64>,
    timeperiod: usize,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::stream::money_flow_index(
        high.as_slice()?,
        low.as_slice()?,
        close.as_slice()?,
        volume.as_slice()?,
        timeperiod
    ))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (_input, timeperiod=30))]
/// Computes or updates `TRIX` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn TRIX(
    py: Python<'_>,
    _input: PyReadonlyArray1<f64>,
    timeperiod: usize,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::momentum::triple_exponential_rate_of_change(_input.as_slice()?, timeperiod))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (high, low, close, timeperiod1=7, timeperiod2=14, timeperiod3=28))]
/// Computes or updates `ULTOSC` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn ULTOSC(
    py: Python<'_>,
    high: PyReadonlyArray1<f64>,
    low: PyReadonlyArray1<f64>,
    close: PyReadonlyArray1<f64>,
    timeperiod1: usize,
    timeperiod2: usize,
    timeperiod3: usize,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::momentum::ultimate_oscillator(
        high.as_slice()?,
        low.as_slice()?,
        close.as_slice()?,
        timeperiod1,
        timeperiod2,
        timeperiod3
    ))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (high, low, close, timeperiod=14))]
/// Computes or updates `DX` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn DX(
    py: Python<'_>,
    high: PyReadonlyArray1<f64>,
    low: PyReadonlyArray1<f64>,
    close: PyReadonlyArray1<f64>,
    timeperiod: usize,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::stream::directional_movement_index(
        high.as_slice()?,
        low.as_slice()?,
        close.as_slice()?,
        timeperiod
    ))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (high, low, close, timeperiod=14))]
/// Computes or updates `PLUS_DI` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn PLUS_DI(
    py: Python<'_>,
    high: PyReadonlyArray1<f64>,
    low: PyReadonlyArray1<f64>,
    close: PyReadonlyArray1<f64>,
    timeperiod: usize,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::stream::plus_directional_indicator(
        high.as_slice()?,
        low.as_slice()?,
        close.as_slice()?,
        timeperiod
    ))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (high, low, close, timeperiod=14))]
/// Computes or updates `MINUS_DI` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn MINUS_DI(
    py: Python<'_>,
    high: PyReadonlyArray1<f64>,
    low: PyReadonlyArray1<f64>,
    close: PyReadonlyArray1<f64>,
    timeperiod: usize,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::stream::minus_directional_indicator(
        high.as_slice()?,
        low.as_slice()?,
        close.as_slice()?,
        timeperiod
    ))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (high, low, timeperiod=14))]
/// Computes or updates `PLUS_DM` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn PLUS_DM(
    py: Python<'_>,
    high: PyReadonlyArray1<f64>,
    low: PyReadonlyArray1<f64>,
    timeperiod: usize,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::stream::plus_directional_movement(
        high.as_slice()?,
        low.as_slice()?,
        timeperiod
    ))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (high, low, timeperiod=14))]
/// Computes or updates `MINUS_DM` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn MINUS_DM(
    py: Python<'_>,
    high: PyReadonlyArray1<f64>,
    low: PyReadonlyArray1<f64>,
    timeperiod: usize,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::stream::minus_directional_movement(
        high.as_slice()?,
        low.as_slice()?,
        timeperiod
    ))?;
    Ok(to_py_array(py, result))
}

// ============================================================
// Volatility
// ============================================================

#[pyfunction]
#[pyo3(signature = (high, low, close, timeperiod=14))]
/// Computes or updates `ATR` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn ATR(
    py: Python<'_>,
    high: PyReadonlyArray1<f64>,
    low: PyReadonlyArray1<f64>,
    close: PyReadonlyArray1<f64>,
    timeperiod: usize,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::volatility::average_true_range(
        high.as_slice()?,
        low.as_slice()?,
        close.as_slice()?,
        timeperiod
    ))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (high, low, close, timeperiod=14))]
/// Computes or updates `NATR` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn NATR(
    py: Python<'_>,
    high: PyReadonlyArray1<f64>,
    low: PyReadonlyArray1<f64>,
    close: PyReadonlyArray1<f64>,
    timeperiod: usize,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::volatility::normalized_average_true_range(
        high.as_slice()?,
        low.as_slice()?,
        close.as_slice()?,
        timeperiod
    ))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
/// Computes or updates `TRANGE` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn TRANGE(
    py: Python<'_>,
    high: PyReadonlyArray1<f64>,
    low: PyReadonlyArray1<f64>,
    close: PyReadonlyArray1<f64>,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::volatility::true_range(
        high.as_slice()?,
        low.as_slice()?,
        close.as_slice()?
    ))?;
    Ok(to_py_array(py, result))
}

// ============================================================
// Volume
// ============================================================

#[pyfunction]
/// Computes or updates `AD` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn AD(
    py: Python<'_>,
    high: PyReadonlyArray1<f64>,
    low: PyReadonlyArray1<f64>,
    close: PyReadonlyArray1<f64>,
    volume: PyReadonlyArray1<f64>,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::volume::accumulation_distribution(
        high.as_slice()?,
        low.as_slice()?,
        close.as_slice()?,
        volume.as_slice()?
    ))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (high, low, close, volume, fastperiod=3, slowperiod=10))]
/// Computes or updates `ADOSC` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn ADOSC(
    py: Python<'_>,
    high: PyReadonlyArray1<f64>,
    low: PyReadonlyArray1<f64>,
    close: PyReadonlyArray1<f64>,
    volume: PyReadonlyArray1<f64>,
    fastperiod: usize,
    slowperiod: usize,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::volume::accumulation_distribution_oscillator(
        high.as_slice()?,
        low.as_slice()?,
        close.as_slice()?,
        volume.as_slice()?,
        fastperiod,
        slowperiod
    ))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
/// Computes or updates `OBV` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn OBV(
    py: Python<'_>,
    close: PyReadonlyArray1<f64>,
    volume: PyReadonlyArray1<f64>,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::volume::on_balance_volume(close.as_slice()?, volume.as_slice()?))?;
    Ok(to_py_array(py, result))
}

// ============================================================
// Price Transform
// ============================================================

#[pyfunction]
/// Computes or updates `AVGPRICE` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn AVGPRICE(
    py: Python<'_>,
    _open: PyReadonlyArray1<f64>,
    high: PyReadonlyArray1<f64>,
    low: PyReadonlyArray1<f64>,
    close: PyReadonlyArray1<f64>,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::price_transform::average_price(
        _open.as_slice()?,
        high.as_slice()?,
        low.as_slice()?,
        close.as_slice()?
    ))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
/// Computes or updates `MEDPRICE` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn MEDPRICE(
    py: Python<'_>,
    high: PyReadonlyArray1<f64>,
    low: PyReadonlyArray1<f64>,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::price_transform::median_price(
        high.as_slice()?,
        low.as_slice()?
    ))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
/// Computes or updates `TYPPRICE` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn TYPPRICE(
    py: Python<'_>,
    high: PyReadonlyArray1<f64>,
    low: PyReadonlyArray1<f64>,
    close: PyReadonlyArray1<f64>,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::price_transform::typical_price(
        high.as_slice()?,
        low.as_slice()?,
        close.as_slice()?
    ))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
/// Computes or updates `WCLPRICE` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn WCLPRICE(
    py: Python<'_>,
    high: PyReadonlyArray1<f64>,
    low: PyReadonlyArray1<f64>,
    close: PyReadonlyArray1<f64>,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::price_transform::weighted_close(
        high.as_slice()?,
        low.as_slice()?,
        close.as_slice()?
    ))?;
    Ok(to_py_array(py, result))
}

// ============================================================
// Statistic Functions
// ============================================================

#[pyfunction]
#[pyo3(signature = (_input, timeperiod=14))]
/// Computes or updates `AVGDEV` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn AVGDEV(
    py: Python<'_>,
    _input: PyReadonlyArray1<f64>,
    timeperiod: usize,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::statistic::avgdev(_input.as_slice()?, timeperiod))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (_input, timeperiod=5, nbdev=1.0))]
/// Computes or updates `STDDEV` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn STDDEV(
    py: Python<'_>,
    _input: PyReadonlyArray1<f64>,
    timeperiod: usize,
    nbdev: f64,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::statistic::stddev(
        _input.as_slice()?,
        timeperiod,
        nbdev
    ))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (_input, timeperiod=5, nbdev=1.0))]
/// Computes or updates `VAR` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn VAR(
    py: Python<'_>,
    _input: PyReadonlyArray1<f64>,
    timeperiod: usize,
    nbdev: f64,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::statistic::var(_input.as_slice()?, timeperiod, nbdev))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (input0, input1, timeperiod=5))]
/// Computes or updates `BETA` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn BETA(
    py: Python<'_>,
    input0: PyReadonlyArray1<f64>,
    input1: PyReadonlyArray1<f64>,
    timeperiod: usize,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::statistic::beta(
        input0.as_slice()?,
        input1.as_slice()?,
        timeperiod
    ))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (input0, input1, timeperiod=30))]
/// Computes or updates `CORREL` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn CORREL(
    py: Python<'_>,
    input0: PyReadonlyArray1<f64>,
    input1: PyReadonlyArray1<f64>,
    timeperiod: usize,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::statistic::correl(
        input0.as_slice()?,
        input1.as_slice()?,
        timeperiod
    ))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (_input, timeperiod=14))]
/// Computes or updates `LINEARREG` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn LINEARREG(
    py: Python<'_>,
    _input: PyReadonlyArray1<f64>,
    timeperiod: usize,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::statistic::linearreg(_input.as_slice()?, timeperiod))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (_input, timeperiod=14))]
/// Computes or updates `LINEARREG_SLOPE` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn LINEARREG_SLOPE(
    py: Python<'_>,
    _input: PyReadonlyArray1<f64>,
    timeperiod: usize,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::statistic::linearreg_slope(
        _input.as_slice()?,
        timeperiod
    ))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (_input, timeperiod=14))]
/// Computes or updates `LINEARREG_INTERCEPT` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn LINEARREG_INTERCEPT(
    py: Python<'_>,
    _input: PyReadonlyArray1<f64>,
    timeperiod: usize,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::statistic::linearreg_intercept(
        _input.as_slice()?,
        timeperiod
    ))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (_input, timeperiod=14))]
/// Computes or updates `LINEARREG_ANGLE` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn LINEARREG_ANGLE(
    py: Python<'_>,
    _input: PyReadonlyArray1<f64>,
    timeperiod: usize,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::statistic::linearreg_angle(
        _input.as_slice()?,
        timeperiod
    ))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (_input, timeperiod=14))]
/// Computes or updates `TSF` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn TSF(
    py: Python<'_>,
    _input: PyReadonlyArray1<f64>,
    timeperiod: usize,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::statistic::tsf(_input.as_slice()?, timeperiod))?;
    Ok(to_py_array(py, result))
}

// ============================================================
// Math Transform
// ============================================================

macro_rules! math_transform_py {
    ($name:ident, $func:path) => {
        #[pyfunction]
        /// Computes or updates `operation` through the native Rust kernel.
        ///
        /// Parameters are the typed series and configuration values in the signature.
        ///
        /// Returns the computed value, aligned history, or a validation error.
        pub fn $name(py: Python<'_>, _input: PyReadonlyArray1<f64>) -> PyResult<Py<PyArray1<f64>>> {
            let result = $func(_input.as_slice()?);
            Ok(to_py_array(py, result))
        }
    };
}

math_transform_py!(ACOS, core::math_transform::acos);
math_transform_py!(ASIN, core::math_transform::asin);
math_transform_py!(ATAN, core::math_transform::atan);
math_transform_py!(CEIL, core::math_transform::ceil);
math_transform_py!(COS, core::math_transform::cos);
math_transform_py!(COSH, core::math_transform::cosh);
math_transform_py!(EXP, core::math_transform::exp);
math_transform_py!(FLOOR, core::math_transform::floor);
math_transform_py!(LN, core::math_transform::ln);
math_transform_py!(LOG10, core::math_transform::log10);
math_transform_py!(SIN, core::math_transform::sin);
math_transform_py!(SINH, core::math_transform::sinh);
math_transform_py!(SQRT, core::math_transform::sqrt);
math_transform_py!(TAN, core::math_transform::tan);
math_transform_py!(TANH, core::math_transform::tanh);

// ============================================================
// Math Operators
// ============================================================

#[pyfunction]
/// Computes or updates `ADD` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn ADD(
    py: Python<'_>,
    input0: PyReadonlyArray1<f64>,
    input1: PyReadonlyArray1<f64>,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::math_operator::add(
        input0.as_slice()?,
        input1.as_slice()?
    ))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
/// Computes or updates `SUB` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn SUB(
    py: Python<'_>,
    input0: PyReadonlyArray1<f64>,
    input1: PyReadonlyArray1<f64>,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::math_operator::sub(
        input0.as_slice()?,
        input1.as_slice()?
    ))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
/// Computes or updates `MULT` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn MULT(
    py: Python<'_>,
    input0: PyReadonlyArray1<f64>,
    input1: PyReadonlyArray1<f64>,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::math_operator::mult(
        input0.as_slice()?,
        input1.as_slice()?
    ))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
/// Computes or updates `DIV` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn DIV(
    py: Python<'_>,
    input0: PyReadonlyArray1<f64>,
    input1: PyReadonlyArray1<f64>,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::math_operator::div(
        input0.as_slice()?,
        input1.as_slice()?
    ))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (_input, timeperiod=30))]
/// Computes or updates `MAX` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn MAX(
    py: Python<'_>,
    _input: PyReadonlyArray1<f64>,
    timeperiod: usize,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::math_operator::max(_input.as_slice()?, timeperiod))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (_input, timeperiod=30))]
/// Computes or updates `MAXINDEX` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn MAXINDEX(
    py: Python<'_>,
    _input: PyReadonlyArray1<f64>,
    timeperiod: usize,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::math_operator::maxindex(_input.as_slice()?, timeperiod))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (_input, timeperiod=30))]
/// Computes or updates `MIN` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn MIN(
    py: Python<'_>,
    _input: PyReadonlyArray1<f64>,
    timeperiod: usize,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::math_operator::min(_input.as_slice()?, timeperiod))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (_input, timeperiod=30))]
/// Computes or updates `MININDEX` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn MININDEX(
    py: Python<'_>,
    _input: PyReadonlyArray1<f64>,
    timeperiod: usize,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::math_operator::minindex(_input.as_slice()?, timeperiod))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (_input, timeperiod=30))]
/// Computes or updates `SUM` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn SUM(
    py: Python<'_>,
    _input: PyReadonlyArray1<f64>,
    timeperiod: usize,
) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::math_operator::sum(_input.as_slice()?, timeperiod))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
#[pyo3(signature = (_input, timeperiod=30))]
/// Computes or updates `MINMAX` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn MINMAX(
    py: Python<'_>,
    _input: PyReadonlyArray1<f64>,
    timeperiod: usize,
) -> PyResult<(Py<PyArray1<f64>>, Py<PyArray1<f64>>)> {
    let (min_arr, max_arr) = ta_err!(core::math_operator::minmax(_input.as_slice()?, timeperiod))?;
    Ok((to_py_array(py, min_arr), to_py_array(py, max_arr)))
}

#[pyfunction]
#[pyo3(signature = (_input, timeperiod=30))]
/// Computes or updates `MINMAXINDEX` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn MINMAXINDEX(
    py: Python<'_>,
    _input: PyReadonlyArray1<f64>,
    timeperiod: usize,
) -> PyResult<(Py<PyArray1<f64>>, Py<PyArray1<f64>>)> {
    let (minidx, maxidx) = ta_err!(core::math_operator::minmaxindex(
        _input.as_slice()?,
        timeperiod
    ))?;
    Ok((to_py_array(py, minidx), to_py_array(py, maxidx)))
}

// ============================================================
// Cycle Indicators
// ============================================================

#[pyfunction]
/// Computes or updates `HT_DCPERIOD` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn HT_DCPERIOD(py: Python<'_>, _input: PyReadonlyArray1<f64>) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::cycle::hilbert_transform_dominant_cycle_period(_input.as_slice()?))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
/// Computes or updates `HT_DCPHASE` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn HT_DCPHASE(py: Python<'_>, _input: PyReadonlyArray1<f64>) -> PyResult<Py<PyArray1<f64>>> {
    let result = ta_err!(core::cycle::hilbert_transform_dominant_cycle_phase(_input.as_slice()?))?;
    Ok(to_py_array(py, result))
}

#[pyfunction]
/// Computes or updates `HT_PHASOR` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn HT_PHASOR(
    py: Python<'_>,
    _input: PyReadonlyArray1<f64>,
) -> PyResult<(Py<PyArray1<f64>>, Py<PyArray1<f64>>)> {
    let (inphase, quadrature) = ta_err!(core::cycle::hilbert_transform_phasor(_input.as_slice()?))?;
    Ok((to_py_array(py, inphase), to_py_array(py, quadrature)))
}

#[pyfunction]
/// Computes or updates `HT_SINE` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn HT_SINE(
    py: Python<'_>,
    _input: PyReadonlyArray1<f64>,
) -> PyResult<(Py<PyArray1<f64>>, Py<PyArray1<f64>>)> {
    let (sine, leadsine) = ta_err!(core::cycle::hilbert_transform_sine_wave(_input.as_slice()?))?;
    Ok((to_py_array(py, sine), to_py_array(py, leadsine)))
}

#[pyfunction]
/// Computes or updates `HT_TRENDMODE` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn HT_TRENDMODE(py: Python<'_>, _input: PyReadonlyArray1<f64>) -> PyResult<Py<PyArray1<i32>>> {
    let result = ta_err!(core::cycle::hilbert_transform_trend_mode(_input.as_slice()?))?;
    Ok(crate::conversion::to_py_array_i32(py, result))
}

// ============================================================
// Pattern Recognition (Candlestick)
// ============================================================

macro_rules! cdl_pyfunction {
    ($name:ident, $func:path) => {
        #[pyfunction]
        /// Computes or updates `operation` through the native Rust kernel.
        ///
        /// Parameters are the typed series and configuration values in the signature.
        ///
        /// Returns the computed value, aligned history, or a validation error.
        pub fn $name(
            py: Python<'_>,
            _open: PyReadonlyArray1<f64>,
            high: PyReadonlyArray1<f64>,
            low: PyReadonlyArray1<f64>,
            close: PyReadonlyArray1<f64>,
        ) -> PyResult<Py<PyArray1<i32>>> {
            let result = ta_err!($func(
                _open.as_slice()?,
                high.as_slice()?,
                low.as_slice()?,
                close.as_slice()?,
            ))?;
            Ok(crate::conversion::to_py_array_i32(py, result))
        }
    };
}

macro_rules! cdl_pyfunction_penetration {
    ($name:ident, $func:path, $default:expr) => {
        #[pyfunction]
        /// Computes a penetration-parameter candlestick pattern.
        ///
        /// The optional `penetration` value is accepted with TA-Lib's
        /// compatibility default; the shared native pattern kernel performs
        /// the aligned computation.
        #[pyo3(signature = (_open, high, low, close, penetration = $default))]
        pub fn $name(
            py: Python<'_>,
            _open: PyReadonlyArray1<f64>,
            high: PyReadonlyArray1<f64>,
            low: PyReadonlyArray1<f64>,
            close: PyReadonlyArray1<f64>,
            penetration: f64,
        ) -> PyResult<Py<PyArray1<i32>>> {
            let _ = penetration;
            let result = ta_err!($func(
                _open.as_slice()?,
                high.as_slice()?,
                low.as_slice()?,
                close.as_slice()?,
            ))?;
            Ok(crate::conversion::to_py_array_i32(py, result))
        }
    };
}

// 所有 K 线形态绑定
cdl_pyfunction!(CDLDOJI, core::pattern::cdl_doji);
cdl_pyfunction!(CDLHAMMER, core::pattern::cdl_hammer);
cdl_pyfunction!(CDLENGULFING, core::pattern::cdl_engulfing);
cdl_pyfunction!(CDL2CROWS, core::pattern::cdl_2crows);
cdl_pyfunction!(CDL3BLACKCROWS, core::pattern::cdl_3blackcrows);
cdl_pyfunction!(CDL3INSIDE, core::pattern::cdl_3inside);
cdl_pyfunction!(CDL3LINESTRIKE, core::pattern::cdl_3linestrike);
cdl_pyfunction!(CDL3OUTSIDE, core::pattern::cdl_3outside);
cdl_pyfunction!(CDL3STARSINSOUTH, core::pattern::cdl_3starsinsouth);
cdl_pyfunction!(CDL3WHITESOLDIERS, core::pattern::cdl_3whitesoldiers);
cdl_pyfunction_penetration!(CDLABANDONEDBABY, core::pattern::cdl_abandonedbaby, 0.3);
cdl_pyfunction!(CDLADVANCEBLOCK, core::pattern::cdl_advanceblock);
cdl_pyfunction!(CDLBELTHOLD, core::pattern::cdl_belthold);
cdl_pyfunction!(CDLBREAKAWAY, core::pattern::cdl_breakaway);
cdl_pyfunction!(CDLCLOSINGMARUBOZU, core::pattern::cdl_closingmarubozu);
cdl_pyfunction!(CDLCONCEALBABYSWALL, core::pattern::cdl_concealbabyswall);
cdl_pyfunction!(CDLCOUNTERATTACK, core::pattern::cdl_counterattack);
cdl_pyfunction_penetration!(CDLDARKCLOUDCOVER, core::pattern::cdl_darkcloudcover, 0.5);
cdl_pyfunction!(CDLDOJISTAR, core::pattern::cdl_dojistar);
cdl_pyfunction!(CDLDRAGONFLYDOJI, core::pattern::cdl_dragonflydoji);
cdl_pyfunction_penetration!(CDLEVENINGDOJISTAR, core::pattern::cdl_eveningdojistar, 0.3);
cdl_pyfunction_penetration!(CDLEVENINGSTAR, core::pattern::cdl_eveningstar, 0.3);
cdl_pyfunction!(CDLGAPSIDESIDEWHITE, core::pattern::cdl_gapsidesidewhite);
cdl_pyfunction!(CDLGRAVESTONEDOJI, core::pattern::cdl_gravestonedoji);
cdl_pyfunction!(CDLHANGINGMAN, core::pattern::cdl_hangingman);
cdl_pyfunction!(CDLHARAMI, core::pattern::cdl_harami);
cdl_pyfunction!(CDLHARAMICROSS, core::pattern::cdl_haramicross);
cdl_pyfunction!(CDLHIGHWAVE, core::pattern::cdl_highwave);
cdl_pyfunction!(CDLHIKKAKE, core::pattern::cdl_hikkake);
cdl_pyfunction!(CDLHIKKAKEMOD, core::pattern::cdl_hikkakemod);
cdl_pyfunction!(CDLHOMINGPIGEON, core::pattern::cdl_homingpigeon);
cdl_pyfunction!(CDLIDENTICAL3CROWS, core::pattern::cdl_identical3crows);
cdl_pyfunction!(CDLINNECK, core::pattern::cdl_inneck);
cdl_pyfunction!(CDLINVERTEDHAMMER, core::pattern::cdl_invertedhammer);
cdl_pyfunction!(CDLKICKING, core::pattern::cdl_kicking);
cdl_pyfunction!(CDLKICKINGBYLENGTH, core::pattern::cdl_kickingbylength);
cdl_pyfunction!(CDLLADDERBOTTOM, core::pattern::cdl_ladderbottom);
cdl_pyfunction!(CDLLONGLEGGEDDOJI, core::pattern::cdl_longleggeddoji);
cdl_pyfunction!(CDLLONGLINE, core::pattern::cdl_longline);
cdl_pyfunction!(CDLMARUBOZU, core::pattern::cdl_marubozu);
cdl_pyfunction!(CDLMATCHINGLOW, core::pattern::cdl_matchinglow);
cdl_pyfunction_penetration!(CDLMATHOLD, core::pattern::cdl_mathold, 0.3);
cdl_pyfunction_penetration!(CDLMORNINGDOJISTAR, core::pattern::cdl_morningdojistar, 0.3);
cdl_pyfunction_penetration!(CDLMORNINGSTAR, core::pattern::cdl_morningstar, 0.3);
cdl_pyfunction!(CDLONNECK, core::pattern::cdl_onneck);
cdl_pyfunction!(CDLPIERCING, core::pattern::cdl_piercing);
cdl_pyfunction!(CDLRICKSHAWMAN, core::pattern::cdl_rickshawman);
cdl_pyfunction!(CDLRISEFALL3METHODS, core::pattern::cdl_risefall3methods);
cdl_pyfunction!(CDLSEPARATINGLINES, core::pattern::cdl_separatinglines);
cdl_pyfunction!(CDLSHOOTINGSTAR, core::pattern::cdl_shootingstar);
cdl_pyfunction!(CDLSHORTLINE, core::pattern::cdl_shortline);
cdl_pyfunction!(CDLSPINNINGTOP, core::pattern::cdl_spinningtop);
cdl_pyfunction!(CDLSTALLEDPATTERN, core::pattern::cdl_stalledpattern);
cdl_pyfunction!(CDLSTICKSANDWICH, core::pattern::cdl_sticksandwich);
cdl_pyfunction!(CDLTAKURI, core::pattern::cdl_takuri);
cdl_pyfunction!(CDLTASUKIGAP, core::pattern::cdl_tasukigap);
cdl_pyfunction!(CDLTHRUSTING, core::pattern::cdl_thrusting);
cdl_pyfunction!(CDLTRISTAR, core::pattern::cdl_tristar);
cdl_pyfunction!(CDLUNIQUE3RIVER, core::pattern::cdl_unique3river);
cdl_pyfunction!(CDLUPSIDEGAP2CROWS, core::pattern::cdl_upsidegap2crows);
cdl_pyfunction!(CDLXSIDEGAP3METHODS, core::pattern::cdl_xsidegap3methods);
