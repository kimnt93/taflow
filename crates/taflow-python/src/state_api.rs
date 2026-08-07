//! PyO3 adapters for the incremental core API.

use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::{
    self, Adx, Adxr, AverageTrueRange, Cci, Dema, Dx, Ema, HtTrendline, Imi, Macd, MacdExt, MacdFix, Mama,
    Mavp,
    RollingMidpoint, RollingMidprice, Mom, NormalizedAverageTrueRange, Roc, Rocp, Rocr, Rocr100, Rsi, Sma, Stoch, Stochf, Stochrsi,
    StreamingIndicator, Tema, TrueRange, Trima, Wma, RelativeMomentumIndex,
    VariableIndexDynamicAverage,
    LaguerreRelativeStrengthIndex,
    EvenBetterSinewave,
    JurikMovingAverage,
    SmoothedTrendChannel, PremiumDiscount, HeikinAshi,
    FibonacciRetracement,
    OpeningRange,
    SessionVolumeLevels,
    KlingerVolumeOscillator,
    ParabolicMovingAverageStop,
    TomDeMarkSequential,
    AnchoredVolumeWeightedAveragePrice,
    PivotPoints,
};
use taflow::MaType;

use crate::conversion::to_py_array;

fn py_value_error(error: impl ToString) -> PyErr {
    PyValueError::new_err(error.to_string())
}

fn values_from<I>(values: I) -> Vec<f64>
where
    I: IntoIterator<Item = Option<f64>>,
{
    values
        .into_iter()
        .map(|value| value.unwrap_or(f64::NAN))
        .collect()
}

macro_rules! scalar_state_class {
    ($class:ident, $inner:ty, $default_period:literal) => {
        #[pyclass]
        pub struct $class {
            inner: $inner,
        }

        #[pymethods]
        impl $class {
            #[new]
            #[pyo3(signature = (timeperiod=$default_period))]
            fn new(timeperiod: usize) -> PyResult<Self> {
                Ok(Self {
                    inner: <$inner>::new(timeperiod).map_err(py_value_error)?,
                })
            }

            fn append(&mut self, input: f64) -> Option<f64> {
                self.inner.append(input)
            }

            fn extend(
                &mut self,
                py: Python<'_>,
                input: PyReadonlyArray1<f64>,
            ) -> PyResult<Py<PyArray1<f64>>> {
                let values = self.inner.extend(input.as_slice()?.iter().copied());
                Ok(to_py_array(py, values_from(values)))
            }

            #[getter]
            fn value(&self) -> Option<f64> {
                self.inner.value()
            }

            fn reset(&mut self) {
                self.inner.reset();
            }
        }
    };
}

scalar_state_class!(StatefulMom, Mom, 10);
scalar_state_class!(StatefulRoc, Roc, 10);
scalar_state_class!(StatefulRocp, Rocp, 10);
scalar_state_class!(StatefulRocr, Rocr, 10);
scalar_state_class!(StatefulRocr100, Rocr100, 10);
scalar_state_class!(StatefulMidpoint, RollingMidpoint, 14);
scalar_state_class!(StatefulMax, stream::RollingMax, 30);
scalar_state_class!(StatefulMaxindex, stream::RollingArgmax, 30);
scalar_state_class!(StatefulMin, stream::RollingMin, 30);
scalar_state_class!(StatefulMinindex, stream::RollingArgmin, 30);
scalar_state_class!(StatefulSum, stream::RollingSum, 30);
scalar_state_class!(StatefulAvgdev, stream::RollingAverageDeviation, 14);
scalar_state_class!(StatefulCmo, stream::Cmo, 14);
scalar_state_class!(StatefulKama, stream::Kama, 30);
scalar_state_class!(StatefulLinearreg, stream::Linearreg, 14);
scalar_state_class!(StatefulLinearregSlope, stream::LinearregSlope, 14);
scalar_state_class!(StatefulLinearregIntercept, stream::LinearregIntercept, 14);
scalar_state_class!(StatefulLinearregAngle, stream::LinearregAngle, 14);
scalar_state_class!(StatefulTsf, stream::Tsf, 14);
/// Native state adapter for the two-parameter Relative Momentum Index.
#[pyclass]
pub struct StatefulRelativeMomentumIndex {
    inner: RelativeMomentumIndex,
    output: Vec<f64>,
}

/// Native state adapter for Variable Index Dynamic Average.
#[pyclass]
pub struct StatefulVariableIndexDynamicAverage {
    inner: VariableIndexDynamicAverage,
    output: Vec<f64>,
}

/// Native state adapter for Laguerre Relative Strength Index.
#[pyclass]
pub struct StatefulLaguerreRelativeStrengthIndex {
    inner: LaguerreRelativeStrengthIndex,
    output: Vec<f64>,
}

/// Native state adapter for Even Better Sinewave.
#[pyclass]
pub struct StatefulEvenBetterSinewave {
    inner: EvenBetterSinewave,
    output: Vec<f64>,
}

/// Native state adapter for Jurik-like adaptive moving average.
#[pyclass]
pub struct StatefulJurikMovingAverage {
    inner: JurikMovingAverage,
    output: Vec<f64>,
}

/// Native state adapter for SSL Channel.
#[pyclass]
pub struct StatefulSmoothedTrendChannel {
    inner: SmoothedTrendChannel,
    lower: Vec<f64>,
    upper: Vec<f64>,
}

/// Native state adapter for premium/discount zones.
#[pyclass]
pub struct StatefulPremiumDiscount {
    inner: PremiumDiscount,
    zones: Vec<i32>,
    equilibrium: Vec<f64>,
}

#[pymethods]
impl StatefulPremiumDiscount {
    #[new]
    #[pyo3(signature = (window=20))]
    fn new(window: usize) -> PyResult<Self> {
        Ok(Self { inner: PremiumDiscount::new(window).map_err(py_value_error)?, zones: Vec::new(), equilibrium: Vec::new() })
    }
    fn append(&mut self, close: f64) -> (i32, f64) {
        let value = self.inner.append(close);
        self.zones.push(value.0); self.equilibrium.push(value.1); value
    }
    fn extend(&mut self, close: PyReadonlyArray1<f64>) -> PyResult<()> {
        for &value in close.as_slice()? { self.append(value); } Ok(())
    }
    fn compute<'py>(&self, py: Python<'py>) -> (Bound<'py, PyArray1<i32>>, Bound<'py, PyArray1<f64>>) {
        (PyArray1::from_vec(py, self.zones.clone()), PyArray1::from_vec(py, self.equilibrium.clone()))
    }
    #[getter]
    fn value(&self) -> Option<(i32, f64)> { self.inner.value() }
    fn reset(&mut self) { self.inner.reset(); self.zones.clear(); self.equilibrium.clear(); }
}

/// Native state adapter for Heikin-Ashi OHLC transformation.
#[pyclass]
pub struct StatefulHeikinAshi {
    inner: HeikinAshi,
    open: Vec<f64>,
    high: Vec<f64>,
    low: Vec<f64>,
    close: Vec<f64>,
}

/// Native state adapter for rolling Fibonacci retracement levels.
#[pyclass]
pub struct StatefulFibonacciRetracement {
    inner: FibonacciRetracement,
    levels: [Vec<f64>; 7],
}

/// Native state adapter for opening range and breakout flags.
#[pyclass]
pub struct StatefulOpeningRange {
    inner: OpeningRange,
    highs: Vec<f64>,
    lows: Vec<f64>,
    breakouts: Vec<i32>,
}

/// Native state adapter for session volume-profile levels.
#[pyclass]
pub struct StatefulSessionVolumeLevels {
    inner: SessionVolumeLevels,
    poc: Vec<f64>,
    value_area_high: Vec<f64>,
    value_area_low: Vec<f64>,
}

/// Native state adapter for Klinger volume oscillator.
#[pyclass]
pub struct StatefulKlingerVolumeOscillator {
    inner: KlingerVolumeOscillator,
    oscillator: Vec<f64>,
    signal: Vec<f64>,
}

/// Native state adapter for Parabolic Moving Average Stop.
#[pyclass]
pub struct StatefulParabolicMovingAverageStop {
    inner: ParabolicMovingAverageStop,
    stops: Vec<f64>,
    trends: Vec<i32>,
}

/// Native state adapter for Tom DeMark Sequential setup counts.
#[pyclass]
pub struct StatefulTomDeMarkSequential {
    inner: TomDeMarkSequential,
    buys: Vec<i32>,
    sells: Vec<i32>,
}

/// Native state adapter for anchored volume-weighted average price bands.
#[pyclass]
pub struct StatefulAnchoredVolumeWeightedAveragePrice {
    inner: AnchoredVolumeWeightedAveragePrice,
    means: Vec<f64>,
    uppers: Vec<f64>,
    lowers: Vec<f64>,
}

/// Native state adapter for classic pivot levels.
#[pyclass]
pub struct StatefulPivotPoints {
    inner: PivotPoints,
    levels: [Vec<f64>; 5],
}

#[pymethods]
impl StatefulPivotPoints {
    #[new]
    fn new() -> Self { Self { inner: PivotPoints::new(), levels: std::array::from_fn(|_| Vec::new()) } }
    fn append(&mut self, high: f64, low: f64, close: f64, anchor: bool) -> (f64, f64, f64, f64, f64) {
        let value = self.inner.append(high, low, close, anchor);
        let values = [value.0, value.1, value.2, value.3, value.4];
        for (index, level) in values.iter().enumerate() { self.levels[index].push(*level); }
        value
    }
    fn extend(&mut self, high: PyReadonlyArray1<f64>, low: PyReadonlyArray1<f64>, close: PyReadonlyArray1<f64>, anchor: PyReadonlyArray1<bool>) -> PyResult<()> {
        let (high, low, close, anchor) = (high.as_slice()?, low.as_slice()?, close.as_slice()?, anchor.as_slice()?);
        if high.len() != low.len() || high.len() != close.len() || high.len() != anchor.len() { return Err(PyValueError::new_err("inputs must have equal lengths")); }
        for (((&high, &low), &close), &anchor) in high.iter().zip(low).zip(close).zip(anchor) { self.append(high, low, close, anchor); }
        Ok(())
    }
    fn compute<'py>(&self, py: Python<'py>) -> (Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>) {
        (PyArray1::from_vec(py, self.levels[0].clone()), PyArray1::from_vec(py, self.levels[1].clone()), PyArray1::from_vec(py, self.levels[2].clone()), PyArray1::from_vec(py, self.levels[3].clone()), PyArray1::from_vec(py, self.levels[4].clone()))
    }
    #[getter]
    fn value(&self) -> (f64, f64, f64, f64, f64) { self.inner.value() }
    fn reset(&mut self) { self.inner.reset(); for level in &mut self.levels { level.clear(); } }
}

#[pymethods]
impl StatefulAnchoredVolumeWeightedAveragePrice {
    #[new]
    #[pyo3(signature = (stdev=1.0))]
    fn new(stdev: f64) -> Self { Self { inner: AnchoredVolumeWeightedAveragePrice::new(stdev), means: Vec::new(), uppers: Vec::new(), lowers: Vec::new() } }
    fn append(&mut self, high: f64, low: f64, close: f64, volume: f64, anchor: bool) -> (f64, f64, f64) {
        let value = self.inner.append(high, low, close, volume, anchor); self.means.push(value.0); self.uppers.push(value.1); self.lowers.push(value.2); value
    }
    fn extend(&mut self, high: PyReadonlyArray1<f64>, low: PyReadonlyArray1<f64>, close: PyReadonlyArray1<f64>, volume: PyReadonlyArray1<f64>, anchor: PyReadonlyArray1<bool>) -> PyResult<()> {
        let (high, low, close, volume, anchor) = (high.as_slice()?, low.as_slice()?, close.as_slice()?, volume.as_slice()?, anchor.as_slice()?);
        if high.len() != low.len() || high.len() != close.len() || high.len() != volume.len() || high.len() != anchor.len() { return Err(PyValueError::new_err("inputs must have equal lengths")); }
        for ((((&high, &low), &close), &volume), &anchor) in high.iter().zip(low).zip(close).zip(volume).zip(anchor) { self.append(high, low, close, volume, anchor); }
        Ok(())
    }
    fn compute<'py>(&self, py: Python<'py>) -> (Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>) {
        (PyArray1::from_vec(py, self.means.clone()), PyArray1::from_vec(py, self.uppers.clone()), PyArray1::from_vec(py, self.lowers.clone()))
    }
    #[getter]
    fn value(&self) -> Option<(f64, f64, f64)> { self.inner.value() }
    fn reset(&mut self) { self.inner.reset(); self.means.clear(); self.uppers.clear(); self.lowers.clear(); }
}

#[pymethods]
impl StatefulTomDeMarkSequential {
    #[new]
    fn new() -> Self { Self { inner: TomDeMarkSequential::new(), buys: Vec::new(), sells: Vec::new() } }
    fn append(&mut self, close: f64) -> (i32, i32) {
        let value = self.inner.append(close); self.buys.push(value.0); self.sells.push(value.1); value
    }
    fn extend(&mut self, close: PyReadonlyArray1<f64>) -> PyResult<()> {
        for &value in close.as_slice()? { self.append(value); } Ok(())
    }
    fn compute<'py>(&self, py: Python<'py>) -> (Bound<'py, PyArray1<i32>>, Bound<'py, PyArray1<i32>>) {
        (PyArray1::from_vec(py, self.buys.clone()), PyArray1::from_vec(py, self.sells.clone()))
    }
    #[getter]
    fn value(&self) -> Option<(i32, i32)> { self.inner.value() }
    fn reset(&mut self) { self.inner.reset(); self.buys.clear(); self.sells.clear(); }
}

#[pymethods]
impl StatefulParabolicMovingAverageStop {
    #[new]
    #[pyo3(signature = (length=10, multiplier=3.0))]
    fn new(length: usize, multiplier: f64) -> PyResult<Self> {
        Ok(Self { inner: ParabolicMovingAverageStop::new(length, multiplier).map_err(py_value_error)?, stops: Vec::new(), trends: Vec::new() })
    }
    fn append(&mut self, high: f64, low: f64, close: f64) -> (f64, i32) {
        let value = self.inner.append(high, low, close); self.stops.push(value.0); self.trends.push(value.1); value
    }
    fn extend(&mut self, high: PyReadonlyArray1<f64>, low: PyReadonlyArray1<f64>, close: PyReadonlyArray1<f64>) -> PyResult<()> {
        let (high, low, close) = (high.as_slice()?, low.as_slice()?, close.as_slice()?);
        if high.len() != low.len() || high.len() != close.len() { return Err(PyValueError::new_err("inputs must have equal lengths")); }
        for ((&high, &low), &close) in high.iter().zip(low).zip(close) { self.append(high, low, close); }
        Ok(())
    }
    fn compute<'py>(&self, py: Python<'py>) -> (Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<i32>>) {
        (PyArray1::from_vec(py, self.stops.clone()), PyArray1::from_vec(py, self.trends.clone()))
    }
    #[getter]
    fn value(&self) -> Option<(f64, i32)> { self.inner.value() }
    fn reset(&mut self) { self.inner.reset(); self.stops.clear(); self.trends.clear(); }
}

#[pymethods]
impl StatefulKlingerVolumeOscillator {
    #[new]
    #[pyo3(signature = (fast=34, slow=55, signal=13))]
    fn new(fast: usize, slow: usize, signal: usize) -> PyResult<Self> {
        Ok(Self { inner: KlingerVolumeOscillator::new(fast, slow, signal).map_err(py_value_error)?, oscillator: Vec::new(), signal: Vec::new() })
    }
    fn append(&mut self, high: f64, low: f64, close: f64, volume: f64) -> (f64, f64) {
        let value = self.inner.append(high, low, close, volume); self.oscillator.push(value.0); self.signal.push(value.1); value
    }
    fn extend(&mut self, high: PyReadonlyArray1<f64>, low: PyReadonlyArray1<f64>, close: PyReadonlyArray1<f64>, volume: PyReadonlyArray1<f64>) -> PyResult<()> {
        let (high, low, close, volume) = (high.as_slice()?, low.as_slice()?, close.as_slice()?, volume.as_slice()?);
        if high.len() != low.len() || high.len() != close.len() || high.len() != volume.len() { return Err(PyValueError::new_err("inputs must have equal lengths")); }
        for (((&high, &low), &close), &volume) in high.iter().zip(low).zip(close).zip(volume) { self.append(high, low, close, volume); }
        Ok(())
    }
    fn compute<'py>(&self, py: Python<'py>) -> (Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>) {
        (PyArray1::from_vec(py, self.oscillator.clone()), PyArray1::from_vec(py, self.signal.clone()))
    }
    #[getter]
    fn value(&self) -> Option<(f64, f64)> { self.inner.value() }
    fn reset(&mut self) { self.inner.reset(); self.oscillator.clear(); self.signal.clear(); }
}

#[pymethods]
impl StatefulSessionVolumeLevels {
    #[new]
    #[pyo3(signature = (bins=24, value_area=0.7))]
    fn new(bins: usize, value_area: f64) -> PyResult<Self> {
        Ok(Self { inner: SessionVolumeLevels::new(bins, value_area).map_err(py_value_error)?, poc: Vec::new(), value_area_high: Vec::new(), value_area_low: Vec::new() })
    }
    fn append(&mut self, high: f64, low: f64, close: f64, volume: f64, anchor: bool) -> (f64, f64, f64) {
        let value = self.inner.append(high, low, close, volume, anchor);
        self.poc.push(value.0); self.value_area_high.push(value.1); self.value_area_low.push(value.2); value
    }
    fn extend(&mut self, high: PyReadonlyArray1<f64>, low: PyReadonlyArray1<f64>, close: PyReadonlyArray1<f64>, volume: PyReadonlyArray1<f64>, anchor: PyReadonlyArray1<bool>) -> PyResult<()> {
        let (high, low, close, volume, anchor) = (high.as_slice()?, low.as_slice()?, close.as_slice()?, volume.as_slice()?, anchor.as_slice()?);
        if high.len() != low.len() || high.len() != close.len() || high.len() != volume.len() || high.len() != anchor.len() { return Err(PyValueError::new_err("inputs must have equal lengths")); }
        for ((((&high, &low), &close), &volume), &anchor) in high.iter().zip(low).zip(close).zip(volume).zip(anchor) { self.append(high, low, close, volume, anchor); }
        Ok(())
    }
    fn compute<'py>(&self, py: Python<'py>) -> (Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>) {
        (PyArray1::from_vec(py, self.poc.clone()), PyArray1::from_vec(py, self.value_area_high.clone()), PyArray1::from_vec(py, self.value_area_low.clone()))
    }
    #[getter]
    fn value(&self) -> Option<(f64, f64, f64)> { self.inner.value() }
    fn reset(&mut self) { self.inner.reset(); self.poc.clear(); self.value_area_high.clear(); self.value_area_low.clear(); }
}

#[pymethods]
impl StatefulOpeningRange {
    #[new]
    #[pyo3(signature = (bars=30))]
    fn new(bars: usize) -> Self { Self { inner: OpeningRange::new(bars), highs: Vec::new(), lows: Vec::new(), breakouts: Vec::new() } }
    fn append(&mut self, high: f64, low: f64, close: f64, anchor: bool) -> (f64, f64, i32) {
        let value = self.inner.append(high, low, close, anchor);
        self.highs.push(value.0); self.lows.push(value.1); self.breakouts.push(value.2); value
    }
    fn extend(&mut self, high: PyReadonlyArray1<f64>, low: PyReadonlyArray1<f64>, close: PyReadonlyArray1<f64>, anchor: PyReadonlyArray1<bool>) -> PyResult<()> {
        let (high, low, close, anchor) = (high.as_slice()?, low.as_slice()?, close.as_slice()?, anchor.as_slice()?);
        if high.len() != low.len() || high.len() != close.len() || high.len() != anchor.len() { return Err(PyValueError::new_err("inputs must have equal lengths")); }
        for (((&high, &low), &close), &anchor) in high.iter().zip(low).zip(close).zip(anchor) { self.append(high, low, close, anchor); }
        Ok(())
    }
    fn compute<'py>(&self, py: Python<'py>) -> (Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<i32>>) {
        (PyArray1::from_vec(py, self.highs.clone()), PyArray1::from_vec(py, self.lows.clone()), PyArray1::from_vec(py, self.breakouts.clone()))
    }
    #[getter]
    fn value(&self) -> Option<(f64, f64, i32)> { self.inner.value() }
    fn reset(&mut self) { self.inner.reset(); self.highs.clear(); self.lows.clear(); self.breakouts.clear(); }
}

#[pymethods]
impl StatefulFibonacciRetracement {
    #[new]
    #[pyo3(signature = (window=120))]
    fn new(window: usize) -> PyResult<Self> {
        Ok(Self { inner: FibonacciRetracement::new(window).map_err(py_value_error)?, levels: std::array::from_fn(|_| Vec::new()) })
    }
    fn append(&mut self, close: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let value = self.inner.append(close);
        for (index, level) in value.iter().enumerate() { self.levels[index].push(*level); }
        (value[0], value[1], value[2], value[3], value[4], value[5], value[6])
    }
    fn extend(&mut self, close: PyReadonlyArray1<f64>) -> PyResult<()> {
        for &value in close.as_slice()? { self.append(value); } Ok(())
    }
    fn compute<'py>(&self, py: Python<'py>) -> (Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>) {
        (PyArray1::from_vec(py, self.levels[0].clone()), PyArray1::from_vec(py, self.levels[1].clone()), PyArray1::from_vec(py, self.levels[2].clone()), PyArray1::from_vec(py, self.levels[3].clone()), PyArray1::from_vec(py, self.levels[4].clone()), PyArray1::from_vec(py, self.levels[5].clone()), PyArray1::from_vec(py, self.levels[6].clone()))
    }
    #[getter]
    fn value(&self) -> Option<(f64, f64, f64, f64, f64, f64, f64)> { self.inner.value().map(|v| (v[0], v[1], v[2], v[3], v[4], v[5], v[6])) }
    fn reset(&mut self) { self.inner.reset(); for level in &mut self.levels { level.clear(); } }
}

#[pymethods]
impl StatefulHeikinAshi {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self { inner: HeikinAshi::new().map_err(py_value_error)?, open: Vec::new(), high: Vec::new(), low: Vec::new(), close: Vec::new() })
    }
    fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> (f64, f64, f64, f64) {
        let value = self.inner.append(open, high, low, close);
        self.open.push(value.0); self.high.push(value.1); self.low.push(value.2); self.close.push(value.3); value
    }
    fn extend(&mut self, open: PyReadonlyArray1<f64>, high: PyReadonlyArray1<f64>, low: PyReadonlyArray1<f64>, close: PyReadonlyArray1<f64>) -> PyResult<()> {
        let (open, high, low, close) = (open.as_slice()?, high.as_slice()?, low.as_slice()?, close.as_slice()?);
        if open.len() != high.len() || open.len() != low.len() || open.len() != close.len() { return Err(PyValueError::new_err("inputs must have equal lengths")); }
        for (((&open, &high), &low), &close) in open.iter().zip(high).zip(low).zip(close) { self.append(open, high, low, close); }
        Ok(())
    }
    fn compute<'py>(&self, py: Python<'py>) -> (Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>) {
        (PyArray1::from_vec(py, self.open.clone()), PyArray1::from_vec(py, self.high.clone()), PyArray1::from_vec(py, self.low.clone()), PyArray1::from_vec(py, self.close.clone()))
    }
    #[getter]
    fn value(&self) -> Option<(f64, f64, f64, f64)> { self.inner.value() }
    fn reset(&mut self) { self.inner.reset(); self.open.clear(); self.high.clear(); self.low.clear(); self.close.clear(); }
}

#[pymethods]
impl StatefulSmoothedTrendChannel {
    #[new]
    #[pyo3(signature = (length=10))]
    fn new(length: usize) -> PyResult<Self> {
        Ok(Self { inner: SmoothedTrendChannel::new(length).map_err(py_value_error)?, lower: Vec::new(), upper: Vec::new() })
    }
    fn append(&mut self, high: f64, low: f64, close: f64) -> (f64, f64) {
        let value = self.inner.append(high, low, close).unwrap_or((f64::NAN, f64::NAN));
        self.lower.push(value.0); self.upper.push(value.1); value
    }
    fn extend(&mut self, high: PyReadonlyArray1<f64>, low: PyReadonlyArray1<f64>, close: PyReadonlyArray1<f64>) -> PyResult<()> {
        let (high, low, close) = (high.as_slice()?, low.as_slice()?, close.as_slice()?);
        if high.len() != low.len() || high.len() != close.len() { return Err(PyValueError::new_err("inputs must have equal lengths")); }
        for ((&high, &low), &close) in high.iter().zip(low).zip(close) { self.append(high, low, close); }
        Ok(())
    }
    fn compute<'py>(&self, py: Python<'py>) -> (Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>) {
        (PyArray1::from_vec(py, self.lower.clone()), PyArray1::from_vec(py, self.upper.clone()))
    }
    #[getter]
    fn value(&self) -> Option<(f64, f64)> { self.inner.value() }
    fn reset(&mut self) { self.inner.reset(); self.lower.clear(); self.upper.clear(); }
}

#[pymethods]
impl StatefulJurikMovingAverage {
    #[new]
    #[pyo3(signature = (length=7, phase=0.0))]
    fn new(length: usize, phase: f64) -> PyResult<Self> {
        Ok(Self { inner: JurikMovingAverage::new(length, phase).map_err(py_value_error)?, output: Vec::new() })
    }
    fn append(&mut self, input: f64) -> Option<f64> {
        let value = self.inner.append(input); self.output.push(value.unwrap_or(f64::NAN)); value
    }
    fn extend(&mut self, input: PyReadonlyArray1<f64>) -> PyResult<()> {
        for &value in input.as_slice()? { self.append(value); } Ok(())
    }
    fn compute<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> { PyArray1::from_vec(py, self.output.clone()) }
    #[getter]
    fn value(&self) -> Option<f64> { self.inner.value() }
    fn reset(&mut self) { self.inner.reset(); self.output.clear(); }
}

#[pymethods]
impl StatefulEvenBetterSinewave {
    #[new]
    #[pyo3(signature = (length=40))]
    fn new(length: usize) -> PyResult<Self> {
        Ok(Self { inner: EvenBetterSinewave::new(length).map_err(py_value_error)?, output: Vec::new() })
    }
    fn append(&mut self, input: f64) -> Option<f64> {
        let value = self.inner.append(input); self.output.push(value.unwrap_or(f64::NAN)); value
    }
    fn extend(&mut self, input: PyReadonlyArray1<f64>) -> PyResult<()> {
        for &value in input.as_slice()? { self.append(value); } Ok(())
    }
    fn compute<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> { PyArray1::from_vec(py, self.output.clone()) }
    #[getter]
    fn value(&self) -> Option<f64> { self.inner.value() }
    fn reset(&mut self) { self.inner.reset(); self.output.clear(); }
}

#[pymethods]
impl StatefulLaguerreRelativeStrengthIndex {
    #[new]
    #[pyo3(signature = (gamma=0.5))]
    fn new(gamma: f64) -> PyResult<Self> {
        Ok(Self { inner: LaguerreRelativeStrengthIndex::new(gamma).map_err(py_value_error)?, output: Vec::new() })
    }
    fn append(&mut self, input: f64) -> Option<f64> {
        let value = self.inner.append(input); self.output.push(value.unwrap_or(f64::NAN)); value
    }
    fn extend(&mut self, input: PyReadonlyArray1<f64>) -> PyResult<()> {
        for &value in input.as_slice()? { self.append(value); } Ok(())
    }
    fn compute<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> { PyArray1::from_vec(py, self.output.clone()) }
    #[getter]
    fn value(&self) -> Option<f64> { self.inner.value() }
    fn reset(&mut self) { self.inner.reset(); self.output.clear(); }
}

#[pymethods]
impl StatefulVariableIndexDynamicAverage {
    #[new]
    #[pyo3(signature = (length=14, alpha=None))]
    fn new(length: usize, alpha: Option<f64>) -> PyResult<Self> {
        let alpha = alpha.unwrap_or(2.0 / (length as f64 + 1.0));
        Ok(Self {
            inner: VariableIndexDynamicAverage::new(length, alpha).map_err(py_value_error)?,
            output: Vec::new(),
        })
    }

    fn append(&mut self, input: f64) -> Option<f64> {
        let value = self.inner.append(input);
        self.output.push(value.unwrap_or(f64::NAN));
        value
    }

    fn extend(&mut self, input: PyReadonlyArray1<f64>) -> PyResult<()> {
        for &value in input.as_slice()? { self.append(value); }
        Ok(())
    }

    fn compute<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        PyArray1::from_vec(py, self.output.clone())
    }

    #[getter]
    fn value(&self) -> Option<f64> { self.inner.value() }

    fn reset(&mut self) { self.inner.reset(); self.output.clear(); }
}

#[pymethods]
impl StatefulRelativeMomentumIndex {
    #[new]
    #[pyo3(signature = (timeperiod=14, momentum=5))]
    fn new(timeperiod: usize, momentum: usize) -> PyResult<Self> {
        Ok(Self { inner: RelativeMomentumIndex::new(timeperiod, momentum).map_err(py_value_error)?, output: Vec::new() })
    }

    fn append(&mut self, input: f64) -> Option<f64> {
        let value = self.inner.append(input);
        self.output.push(value.unwrap_or(f64::NAN));
        value
    }

    fn extend(&mut self, py: Python<'_>, input: PyReadonlyArray1<f64>) -> PyResult<Py<PyArray1<f64>>> {
        for &value in input.as_slice()? { self.append(value); }
        Ok(to_py_array(py, self.output.clone()))
    }

    fn compute<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        PyArray1::from_vec(py, self.output.clone())
    }

    #[getter]
    fn value(&self) -> Option<f64> { self.inner.value() }

    fn reset(&mut self) { self.inner.reset(); self.output.clear(); }
}

#[pyclass]
pub struct StatefulCci {
    inner: Cci,
}

#[pymethods]
impl StatefulCci {
    #[new]
    #[pyo3(signature = (timeperiod=14))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: Cci::new(timeperiod).map_err(py_value_error)?,
        })
    }

    fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        self.inner.append(high, low, close)
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<Py<PyArray1<f64>>> {
        let values = self
            .inner
            .extend_slice(high.as_slice()?, low.as_slice()?, close.as_slice()?)
            .map_err(py_value_error)?;
        Ok(to_py_array(py, values_from(values)))
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn reset(&mut self) {
        self.inner.reset();
    }
}

#[pyclass]
pub struct StatefulImi {
    inner: Imi,
}

#[pymethods]
impl StatefulImi {
    #[new]
    #[pyo3(signature = (timeperiod=14))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: Imi::new(timeperiod).map_err(py_value_error)?,
        })
    }

    fn append(&mut self, open: f64, close: f64) -> Option<f64> {
        self.inner.append(open, close)
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        open: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<Py<PyArray1<f64>>> {
        let open = open.as_slice()?;
        let close = close.as_slice()?;
        if open.len() != close.len() {
            return Err(PyValueError::new_err(
                "open and close must have equal lengths",
            ));
        }
        Ok(to_py_array(
            py,
            values_from(
                open.iter()
                    .zip(close)
                    .map(|(&open, &close)| self.inner.append(open, close)),
            ),
        ))
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn reset(&mut self) {
        self.inner.reset();
    }
}

#[pyclass]
pub struct StatefulT3 {
    inner: stream::T3,
}

#[pymethods]
impl StatefulT3 {
    #[new]
    #[pyo3(signature = (timeperiod=5, vfactor=0.7))]
    fn new(timeperiod: usize, vfactor: f64) -> PyResult<Self> {
        Ok(Self {
            inner: stream::T3::new(timeperiod, vfactor).map_err(py_value_error)?,
        })
    }

    fn append(&mut self, input: f64) -> Option<f64> {
        self.inner.append(input)
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        input: PyReadonlyArray1<f64>,
    ) -> PyResult<Py<PyArray1<f64>>> {
        let values = self.inner.extend(input.as_slice()?.iter().copied());
        Ok(to_py_array(py, values_from(values)))
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn reset(&mut self) {
        self.inner.reset();
    }
}

macro_rules! oscillator_state_class {
    ($class:ident, $inner:ty) => {
        #[pyclass]
        pub struct $class {
            inner: $inner,
        }

        #[pymethods]
        impl $class {
            #[new]
            #[pyo3(signature = (fastperiod=12, slowperiod=26, matype=0))]
            fn new(fastperiod: usize, slowperiod: usize, matype: i32) -> PyResult<Self> {
                Ok(Self {
                    inner: <$inner>::new(
                        fastperiod,
                        slowperiod,
                        MaType::try_from(matype).map_err(py_value_error)?,
                    )
                    .map_err(py_value_error)?,
                })
            }

            fn append(&mut self, input: f64) -> Option<f64> {
                self.inner.append(input)
            }

            fn extend(
                &mut self,
                py: Python<'_>,
                input: PyReadonlyArray1<f64>,
            ) -> PyResult<Py<PyArray1<f64>>> {
                let values = self.inner.extend(input.as_slice()?.iter().copied());
                Ok(to_py_array(py, values_from(values)))
            }

            #[getter]
            fn value(&self) -> Option<f64> {
                self.inner.value()
            }

            fn reset(&mut self) {
                self.inner.reset();
            }
        }
    };
}

oscillator_state_class!(StatefulApo, stream::Apo);
oscillator_state_class!(StatefulPpo, stream::Ppo);

#[pyclass]
pub struct StatefulMa {
    inner: stream::Ma,
}

#[pymethods]
impl StatefulMa {
    #[new]
    #[pyo3(signature = (timeperiod=30, matype=0))]
    fn new(timeperiod: usize, matype: i32) -> PyResult<Self> {
        Ok(Self {
            inner: stream::Ma::new(
                timeperiod,
                MaType::try_from(matype).map_err(py_value_error)?,
            )
            .map_err(py_value_error)?,
        })
    }

    fn append(&mut self, input: f64) -> Option<f64> {
        self.inner.append(input)
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        input: PyReadonlyArray1<f64>,
    ) -> PyResult<Py<PyArray1<f64>>> {
        let values = self.inner.extend(input.as_slice()?.iter().copied());
        Ok(to_py_array(py, values_from(values)))
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn reset(&mut self) {
        self.inner.reset();
    }
}

#[pyclass]
pub struct StatefulBbands {
    inner: stream::Bbands,
}

#[pymethods]
impl StatefulBbands {
    #[new]
    #[pyo3(signature = (timeperiod=5, nbdevup=2.0, nbdevdn=2.0, matype=0))]
    fn new(timeperiod: usize, nbdevup: f64, nbdevdn: f64, matype: i32) -> PyResult<Self> {
        Ok(Self {
            inner: stream::Bbands::new(
                timeperiod,
                nbdevup,
                nbdevdn,
                MaType::try_from(matype).map_err(py_value_error)?,
            )
            .map_err(py_value_error)?,
        })
    }

    fn append(&mut self, input: f64) -> Option<(f64, f64, f64)> {
        self.inner
            .append(input)
            .map(|value| (value.upper, value.middle, value.lower))
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        input: PyReadonlyArray1<f64>,
    ) -> PyResult<(Py<PyArray1<f64>>, Py<PyArray1<f64>>, Py<PyArray1<f64>>)> {
        let mut upper = Vec::with_capacity(input.len()?);
        let mut middle = Vec::with_capacity(input.len()?);
        let mut lower = Vec::with_capacity(input.len()?);
        for input in input.as_slice()?.iter().copied() {
            if let Some(value) = self.inner.append(input) {
                upper.push(value.upper);
                middle.push(value.middle);
                lower.push(value.lower);
            } else {
                upper.push(f64::NAN);
                middle.push(f64::NAN);
                lower.push(f64::NAN);
            }
        }
        Ok((
            to_py_array(py, upper),
            to_py_array(py, middle),
            to_py_array(py, lower),
        ))
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64, f64)> {
        self.inner
            .value()
            .map(|value| (value.upper, value.middle, value.lower))
    }

    fn reset(&mut self) {
        self.inner.reset();
    }
}

#[pyclass]
pub struct StatefulAccbands {
    inner: stream::Accbands,
}

#[pymethods]
impl StatefulAccbands {
    #[new]
    #[pyo3(signature = (timeperiod=20))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: stream::Accbands::new(timeperiod).map_err(py_value_error)?,
        })
    }

    fn append(&mut self, high: f64, low: f64, close: f64) -> Option<(f64, f64, f64)> {
        self.inner
            .append(high, low, close)
            .map(|value| (value.upper, value.middle, value.lower))
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<(Py<PyArray1<f64>>, Py<PyArray1<f64>>, Py<PyArray1<f64>>)> {
        let high = high.as_slice()?;
        let low = low.as_slice()?;
        let close = close.as_slice()?;
        if high.len() != low.len() || high.len() != close.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        let mut upper = Vec::with_capacity(high.len());
        let mut middle = Vec::with_capacity(high.len());
        let mut lower = Vec::with_capacity(high.len());
        for ((&high, &low), &close) in high.iter().zip(low).zip(close) {
            if let Some(value) = self.inner.append(high, low, close) {
                upper.push(value.upper);
                middle.push(value.middle);
                lower.push(value.lower);
            } else {
                upper.push(f64::NAN);
                middle.push(f64::NAN);
                lower.push(f64::NAN);
            }
        }
        Ok((
            to_py_array(py, upper),
            to_py_array(py, middle),
            to_py_array(py, lower),
        ))
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64, f64)> {
        self.inner
            .value()
            .map(|value| (value.upper, value.middle, value.lower))
    }

    fn reset(&mut self) {
        self.inner.reset();
    }
}

#[pyclass]
pub struct StatefulSar {
    inner: stream::Sar,
}

#[pymethods]
impl StatefulSar {
    #[new]
    #[pyo3(signature = (acceleration=0.02, maximum=0.2))]
    fn new(acceleration: f64, maximum: f64) -> Self {
        Self {
            inner: stream::Sar::new(acceleration, maximum),
        }
    }

    fn append(&mut self, high: f64, low: f64) -> Option<f64> {
        self.inner.append(high, low)
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
    ) -> PyResult<Py<PyArray1<f64>>> {
        let high = high.as_slice()?;
        let low = low.as_slice()?;
        if high.len() != low.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        Ok(to_py_array(
            py,
            values_from(
                high.iter()
                    .zip(low)
                    .map(|(&high, &low)| self.inner.append(high, low)),
            ),
        ))
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn reset(&mut self) {
        self.inner.reset();
    }
}

#[pyclass]
pub struct StatefulSarext {
    inner: stream::Sarext,
}

#[pymethods]
impl StatefulSarext {
    #[new]
    #[pyo3(signature = (startvalue=0.0, offsetonreverse=0.0, accelerationinitlong=0.02, accelerationlong=0.02, accelerationmaxlong=0.2, accelerationinitshort=0.02, accelerationshort=0.02, accelerationmaxshort=0.2))]
    #[allow(clippy::too_many_arguments)]
    fn new(
        startvalue: f64,
        offsetonreverse: f64,
        accelerationinitlong: f64,
        accelerationlong: f64,
        accelerationmaxlong: f64,
        accelerationinitshort: f64,
        accelerationshort: f64,
        accelerationmaxshort: f64,
    ) -> Self {
        Self {
            inner: stream::Sarext::new(
                startvalue,
                offsetonreverse,
                accelerationinitlong,
                accelerationlong,
                accelerationmaxlong,
                accelerationinitshort,
                accelerationshort,
                accelerationmaxshort,
            ),
        }
    }

    fn append(&mut self, high: f64, low: f64) -> Option<f64> {
        self.inner.append(high, low)
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
    ) -> PyResult<Py<PyArray1<f64>>> {
        let high = high.as_slice()?;
        let low = low.as_slice()?;
        if high.len() != low.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        Ok(to_py_array(
            py,
            values_from(
                high.iter()
                    .zip(low)
                    .map(|(&high, &low)| self.inner.append(high, low)),
            ),
        ))
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn reset(&mut self) {
        self.inner.reset();
    }
}

macro_rules! deviation_state_class {
    ($class:ident, $inner:ident) => {
        #[pyclass]
        pub struct $class {
            inner: stream::$inner,
        }

        #[pymethods]
        impl $class {
            #[new]
            #[pyo3(signature = (timeperiod=5, nbdev=1.0))]
            fn new(timeperiod: usize, nbdev: f64) -> PyResult<Self> {
                Ok(Self {
                    inner: stream::$inner::new(timeperiod, nbdev).map_err(py_value_error)?,
                })
            }

            fn append(&mut self, input: f64) -> Option<f64> {
                self.inner.append(input)
            }

            fn extend(
                &mut self,
                py: Python<'_>,
                input: PyReadonlyArray1<f64>,
            ) -> PyResult<Py<PyArray1<f64>>> {
                let values = self.inner.extend(input.as_slice()?.iter().copied());
                Ok(to_py_array(py, values_from(values)))
            }

            #[getter]
            fn value(&self) -> Option<f64> {
                self.inner.value()
            }

            fn reset(&mut self) {
                self.inner.reset();
            }
        }
    };
}

deviation_state_class!(StatefulVar, RollingVariance);
deviation_state_class!(StatefulStddev, RollingStandardDeviation);

macro_rules! bivariate_statistic_class {
    ($class:ident, $inner:ident) => {
        #[pyclass]
        pub struct $class {
            inner: stream::$inner,
        }

        #[pymethods]
        impl $class {
            #[new]
            #[pyo3(signature = (timeperiod=5))]
            fn new(timeperiod: usize) -> PyResult<Self> {
                Ok(Self {
                    inner: stream::$inner::new(timeperiod).map_err(py_value_error)?,
                })
            }

            fn append(&mut self, input0: f64, input1: f64) -> Option<f64> {
                self.inner.append(input0, input1)
            }

            fn extend(
                &mut self,
                py: Python<'_>,
                input0: PyReadonlyArray1<f64>,
                input1: PyReadonlyArray1<f64>,
            ) -> PyResult<Py<PyArray1<f64>>> {
                let input0 = input0.as_slice()?;
                let input1 = input1.as_slice()?;
                if input0.len() != input1.len() {
                    return Err(PyValueError::new_err("inputs must have equal lengths"));
                }
                let values = input0
                    .iter()
                    .zip(input1)
                    .map(|(&input0, &input1)| self.inner.append(input0, input1));
                Ok(to_py_array(py, values_from(values)))
            }

            #[getter]
            fn value(&self) -> Option<f64> {
                self.inner.value()
            }

            fn reset(&mut self) {
                self.inner.reset();
            }
        }
    };
}

bivariate_statistic_class!(StatefulBeta, RollingBeta);
bivariate_statistic_class!(StatefulCorrel, RollingCorrelation);

#[pyclass]
pub struct StatefulAd {
    inner: stream::AccumulationDistribution,
}

#[pymethods]
impl StatefulAd {
    #[new]
    fn new() -> Self {
        Self {
            inner: stream::AccumulationDistribution::new(),
        }
    }

    fn append(&mut self, high: f64, low: f64, close: f64, volume: f64) -> f64 {
        self.inner.append(high, low, close, volume)
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
        volume: PyReadonlyArray1<f64>,
    ) -> PyResult<Py<PyArray1<f64>>> {
        let high = high.as_slice()?;
        let low = low.as_slice()?;
        let close = close.as_slice()?;
        let volume = volume.as_slice()?;
        if high.len() != low.len() || high.len() != close.len() || high.len() != volume.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        let values = high
            .iter()
            .zip(low)
            .zip(close)
            .zip(volume)
            .map(|(((&h, &l), &c), &v)| self.inner.append(h, l, c, v))
            .collect();
        Ok(to_py_array(py, values))
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn reset(&mut self) {
        self.inner.reset();
    }
}

#[pyclass]
pub struct StatefulAdosc {
    inner: stream::AccumulationDistributionOscillator,
}

#[pymethods]
impl StatefulAdosc {
    #[new]
    #[pyo3(signature = (fastperiod=3, slowperiod=10))]
    fn new(fastperiod: usize, slowperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: stream::AccumulationDistributionOscillator::new(fastperiod, slowperiod).map_err(py_value_error)?,
        })
    }

    fn append(&mut self, high: f64, low: f64, close: f64, volume: f64) -> Option<f64> {
        self.inner.append(high, low, close, volume)
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
        volume: PyReadonlyArray1<f64>,
    ) -> PyResult<Py<PyArray1<f64>>> {
        let high = high.as_slice()?;
        let low = low.as_slice()?;
        let close = close.as_slice()?;
        let volume = volume.as_slice()?;
        if high.len() != low.len() || high.len() != close.len() || high.len() != volume.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        let values = high
            .iter()
            .zip(low)
            .zip(close)
            .zip(volume)
            .map(|(((&h, &l), &c), &v)| self.inner.append(h, l, c, v));
        Ok(to_py_array(py, values_from(values)))
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn reset(&mut self) {
        self.inner.reset();
    }
}

#[pyclass]
pub struct StatefulObv {
    inner: stream::OnBalanceVolume,
}

#[pymethods]
impl StatefulObv {
    #[new]
    fn new() -> Self {
        Self {
            inner: stream::OnBalanceVolume::new(),
        }
    }

    fn append(&mut self, close: f64, volume: f64) -> f64 {
        self.inner.append(close, volume)
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        close: PyReadonlyArray1<f64>,
        volume: PyReadonlyArray1<f64>,
    ) -> PyResult<Py<PyArray1<f64>>> {
        let close = close.as_slice()?;
        let volume = volume.as_slice()?;
        if close.len() != volume.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        let values = close
            .iter()
            .zip(volume)
            .map(|(&close, &volume)| self.inner.append(close, volume))
            .collect();
        Ok(to_py_array(py, values))
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn reset(&mut self) {
        self.inner.reset();
    }
}

#[pyclass]
pub struct StatefulBop {
    inner: stream::BalanceOfPower,
}

#[pymethods]
impl StatefulBop {
    #[new]
    fn new() -> Self {
        Self {
            inner: stream::BalanceOfPower::new(),
        }
    }

    fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> f64 {
        self.inner.append(open, high, low, close)
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        open: PyReadonlyArray1<f64>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<Py<PyArray1<f64>>> {
        let open = open.as_slice()?;
        let high = high.as_slice()?;
        let low = low.as_slice()?;
        let close = close.as_slice()?;
        if open.len() != high.len() || open.len() != low.len() || open.len() != close.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        let values = open
            .iter()
            .zip(high)
            .zip(low)
            .zip(close)
            .map(|(((&o, &h), &l), &c)| self.inner.append(o, h, l, c))
            .collect();
        Ok(to_py_array(py, values))
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn reset(&mut self) {
        self.inner.reset();
    }
}

#[pyclass]
pub struct StatefulWillr {
    inner: stream::WilliamsPercentR,
}

#[pymethods]
impl StatefulWillr {
    #[new]
    #[pyo3(signature = (timeperiod=14))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: stream::WilliamsPercentR::new(timeperiod).map_err(py_value_error)?,
        })
    }

    fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        self.inner.append(high, low, close)
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<Py<PyArray1<f64>>> {
        let high = high.as_slice()?;
        let low = low.as_slice()?;
        let close = close.as_slice()?;
        if high.len() != low.len() || high.len() != close.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        let values = high
            .iter()
            .zip(low)
            .zip(close)
            .map(|((&h, &l), &c)| self.inner.append(h, l, c));
        Ok(to_py_array(py, values_from(values)))
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn reset(&mut self) {
        self.inner.reset();
    }
}

#[pyclass]
pub struct StatefulAroon {
    inner: stream::Aroon,
}

#[pymethods]
impl StatefulAroon {
    #[new]
    #[pyo3(signature = (timeperiod=14))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: stream::Aroon::new(timeperiod).map_err(py_value_error)?,
        })
    }

    fn append(&mut self, high: f64, low: f64) -> Option<(f64, f64)> {
        self.inner
            .append(high, low)
            .map(|value| (value.down, value.up))
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
    ) -> PyResult<(Py<PyArray1<f64>>, Py<PyArray1<f64>>)> {
        let high = high.as_slice()?;
        let low = low.as_slice()?;
        if high.len() != low.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        let mut down = Vec::with_capacity(high.len());
        let mut up = Vec::with_capacity(high.len());
        for (&high, &low) in high.iter().zip(low) {
            if let Some(value) = self.inner.append(high, low) {
                down.push(value.down);
                up.push(value.up);
            } else {
                down.push(f64::NAN);
                up.push(f64::NAN);
            }
        }
        Ok((to_py_array(py, down), to_py_array(py, up)))
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64)> {
        self.inner.value().map(|value| (value.down, value.up))
    }

    fn reset(&mut self) {
        self.inner.reset();
    }
}

#[pyclass]
pub struct StatefulAroonosc {
    inner: stream::AroonOscillator,
}

#[pymethods]
impl StatefulAroonosc {
    #[new]
    #[pyo3(signature = (timeperiod=14))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: stream::AroonOscillator::new(timeperiod).map_err(py_value_error)?,
        })
    }

    fn append(&mut self, high: f64, low: f64) -> Option<f64> {
        self.inner.append(high, low)
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
    ) -> PyResult<Py<PyArray1<f64>>> {
        let high = high.as_slice()?;
        let low = low.as_slice()?;
        if high.len() != low.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        let values = high.iter().zip(low).map(|(&h, &l)| self.inner.append(h, l));
        Ok(to_py_array(py, values_from(values)))
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn reset(&mut self) {
        self.inner.reset();
    }
}

#[pyclass]
pub struct StatefulMinmax {
    inner: stream::RollingMinmax,
}

#[pymethods]
impl StatefulMinmax {
    #[new]
    #[pyo3(signature = (timeperiod=30))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: stream::RollingMinmax::new(timeperiod).map_err(py_value_error)?,
        })
    }

    fn append(&mut self, input: f64) -> Option<(f64, f64)> {
        self.inner
            .append(input)
            .map(|value| (value.minimum, value.maximum))
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        input: PyReadonlyArray1<f64>,
    ) -> PyResult<(Py<PyArray1<f64>>, Py<PyArray1<f64>>)> {
        let mut minimum = Vec::with_capacity(input.len()?);
        let mut maximum = Vec::with_capacity(input.len()?);
        for input in input.as_slice()? {
            if let Some(value) = self.inner.append(*input) {
                minimum.push(value.minimum);
                maximum.push(value.maximum);
            } else {
                minimum.push(f64::NAN);
                maximum.push(f64::NAN);
            }
        }
        Ok((to_py_array(py, minimum), to_py_array(py, maximum)))
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64)> {
        self.inner
            .value()
            .map(|value| (value.minimum, value.maximum))
    }

    fn reset(&mut self) {
        self.inner.reset();
    }
}

#[pyclass]
pub struct StatefulMinmaxindex {
    inner: stream::RollingMinmaxIndex,
}

#[pymethods]
impl StatefulMinmaxindex {
    #[new]
    #[pyo3(signature = (timeperiod=30))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: stream::RollingMinmaxIndex::new(timeperiod).map_err(py_value_error)?,
        })
    }

    fn append(&mut self, input: f64) -> (usize, usize) {
        let value = self.inner.append(input);
        (value.minimum, value.maximum)
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        input: PyReadonlyArray1<f64>,
    ) -> PyResult<(Py<PyArray1<f64>>, Py<PyArray1<f64>>)> {
        let mut minimum = Vec::with_capacity(input.len()?);
        let mut maximum = Vec::with_capacity(input.len()?);
        for input in input.as_slice()? {
            let value = self.inner.append(*input);
            minimum.push(value.minimum as f64);
            maximum.push(value.maximum as f64);
        }
        Ok((to_py_array(py, minimum), to_py_array(py, maximum)))
    }

    #[getter]
    fn value(&self) -> Option<(usize, usize)> {
        self.inner
            .value()
            .map(|value| (value.minimum, value.maximum))
    }

    fn reset(&mut self) {
        self.inner.reset();
    }
}

macro_rules! unary_state_class {
    ($class:ident, $inner:ident) => {
        #[pyclass]
        pub struct $class {
            inner: stream::$inner,
        }

        #[pymethods]
        impl $class {
            #[new]
            fn new() -> Self {
                Self {
                    inner: stream::$inner::new(),
                }
            }

            fn append(&mut self, input: f64) -> f64 {
                self.inner
                    .append(input)
                    .expect("stateless transform is warm")
            }

            fn extend(
                &mut self,
                py: Python<'_>,
                input: PyReadonlyArray1<f64>,
            ) -> PyResult<Py<PyArray1<f64>>> {
                let values = self
                    .inner
                    .extend(input.as_slice()?.iter().copied())
                    .into_iter()
                    .map(|value| value.expect("stateless transform is warm"));
                Ok(to_py_array(py, values.collect()))
            }

            #[getter]
            fn value(&self) -> Option<f64> {
                self.inner.value()
            }

            fn reset(&mut self) {
                self.inner.reset();
            }
        }
    };
}

unary_state_class!(StatefulAcos, Acos);
unary_state_class!(StatefulAsin, Asin);
unary_state_class!(StatefulAtan, Atan);
unary_state_class!(StatefulCeil, Ceil);
unary_state_class!(StatefulCos, Cos);
unary_state_class!(StatefulCosh, Cosh);
unary_state_class!(StatefulExp, Exp);
unary_state_class!(StatefulFloor, Floor);
unary_state_class!(StatefulLn, Ln);
unary_state_class!(StatefulLog10, Log10);
unary_state_class!(StatefulSin, Sin);
unary_state_class!(StatefulSinh, Sinh);
unary_state_class!(StatefulSqrt, Sqrt);
unary_state_class!(StatefulTan, Tan);
unary_state_class!(StatefulTanh, Tanh);

macro_rules! binary_state_class {
    ($class:ident, $inner:ident) => {
        #[pyclass]
        pub struct $class {
            inner: stream::$inner,
        }

        #[pymethods]
        impl $class {
            #[new]
            fn new() -> Self {
                Self {
                    inner: stream::$inner::new(),
                }
            }

            fn append(&mut self, left: f64, right: f64) -> f64 {
                self.inner.append(left, right)
            }

            fn extend(
                &mut self,
                py: Python<'_>,
                left: PyReadonlyArray1<f64>,
                right: PyReadonlyArray1<f64>,
            ) -> PyResult<Py<PyArray1<f64>>> {
                let left = left.as_slice()?;
                let right = right.as_slice()?;
                if left.len() != right.len() {
                    return Err(PyValueError::new_err("inputs must have equal lengths"));
                }
                let values = left
                    .iter()
                    .zip(right)
                    .map(|(&left, &right)| self.inner.append(left, right));
                Ok(to_py_array(py, values.collect()))
            }

            #[getter]
            fn value(&self) -> Option<f64> {
                self.inner.value()
            }

            fn reset(&mut self) {
                self.inner.reset();
            }
        }
    };
}

binary_state_class!(StatefulAdd, Add);
binary_state_class!(StatefulSub, Sub);
binary_state_class!(StatefulMult, Mult);
binary_state_class!(StatefulDiv, Div);
binary_state_class!(StatefulMedprice, Medprice);

macro_rules! price3_state_class {
    ($class:ident, $inner:ident) => {
        #[pyclass]
        pub struct $class {
            inner: stream::$inner,
        }

        #[pymethods]
        impl $class {
            #[new]
            fn new() -> Self {
                Self {
                    inner: stream::$inner::new(),
                }
            }

            fn append(&mut self, high: f64, low: f64, close: f64) -> f64 {
                self.inner.append(high, low, close)
            }

            fn extend(
                &mut self,
                py: Python<'_>,
                high: PyReadonlyArray1<f64>,
                low: PyReadonlyArray1<f64>,
                close: PyReadonlyArray1<f64>,
            ) -> PyResult<Py<PyArray1<f64>>> {
                let high = high.as_slice()?;
                let low = low.as_slice()?;
                let close = close.as_slice()?;
                if high.len() != low.len() || high.len() != close.len() {
                    return Err(PyValueError::new_err("inputs must have equal lengths"));
                }
                let values = high
                    .iter()
                    .zip(low)
                    .zip(close)
                    .map(|((&high, &low), &close)| self.inner.append(high, low, close));
                Ok(to_py_array(py, values.collect()))
            }

            #[getter]
            fn value(&self) -> Option<f64> {
                self.inner.value()
            }

            fn reset(&mut self) {
                self.inner.reset();
            }
        }
    };
}

price3_state_class!(StatefulTypprice, Typprice);
price3_state_class!(StatefulWclprice, Wclprice);

#[pyclass]
pub struct StatefulAvgprice {
    inner: stream::AveragePrice,
}

#[pymethods]
impl StatefulAvgprice {
    #[new]
    fn new() -> Self {
        Self {
            inner: stream::AveragePrice::new(),
        }
    }

    fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> f64 {
        self.inner.append(open, high, low, close)
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        open: PyReadonlyArray1<f64>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<Py<PyArray1<f64>>> {
        let open = open.as_slice()?;
        let high = high.as_slice()?;
        let low = low.as_slice()?;
        let close = close.as_slice()?;
        if open.len() != high.len() || open.len() != low.len() || open.len() != close.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        let values = open
            .iter()
            .zip(high)
            .zip(low)
            .zip(close)
            .map(|(((&open, &high), &low), &close)| self.inner.append(open, high, low, close));
        Ok(to_py_array(py, values.collect()))
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn reset(&mut self) {
        self.inner.reset();
    }
}

#[pyclass]
pub struct StatefulMidprice {
    inner: RollingMidprice,
}

#[pymethods]
impl StatefulMidprice {
    #[new]
    #[pyo3(signature = (timeperiod=14))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: RollingMidprice::new(timeperiod).map_err(py_value_error)?,
        })
    }

    fn append(&mut self, high: f64, low: f64) -> Option<f64> {
        self.inner.append(high, low)
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
    ) -> PyResult<Py<PyArray1<f64>>> {
        let high = high.as_slice()?;
        let low = low.as_slice()?;
        if high.len() != low.len() {
            return Err(PyValueError::new_err(
                "high and low must have equal lengths",
            ));
        }
        let values = high
            .iter()
            .zip(low)
            .map(|(&high, &low)| self.inner.append(high, low));
        Ok(to_py_array(py, values_from(values)))
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn reset(&mut self) {
        self.inner.reset();
    }
}

#[pyclass]
pub struct StatefulSma {
    inner: Sma,
}

#[pymethods]
impl StatefulSma {
    #[new]
    #[pyo3(signature = (timeperiod=30))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: Sma::new(timeperiod).map_err(py_value_error)?,
        })
    }

    fn append(&mut self, input: f64) -> Option<f64> {
        self.inner.append(input)
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        input: PyReadonlyArray1<f64>,
    ) -> PyResult<Py<PyArray1<f64>>> {
        let values = self.inner.extend(input.as_slice()?.iter().copied());
        Ok(to_py_array(py, values_from(values)))
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn reset(&mut self) {
        self.inner.reset();
    }
}

#[pyclass]
pub struct StatefulEma {
    inner: Ema,
}

#[pymethods]
impl StatefulEma {
    #[new]
    #[pyo3(signature = (timeperiod=30))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: Ema::new(timeperiod).map_err(py_value_error)?,
        })
    }

    fn append(&mut self, input: f64) -> Option<f64> {
        self.inner.append(input)
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        input: PyReadonlyArray1<f64>,
    ) -> PyResult<Py<PyArray1<f64>>> {
        let values = self.inner.extend(input.as_slice()?.iter().copied());
        Ok(to_py_array(py, values_from(values)))
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn reset(&mut self) {
        self.inner.reset();
    }
}

#[pyclass]
pub struct StatefulWma {
    inner: Wma,
}

#[pymethods]
impl StatefulWma {
    #[new]
    #[pyo3(signature = (timeperiod=30))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: Wma::new(timeperiod).map_err(py_value_error)?,
        })
    }

    fn append(&mut self, input: f64) -> Option<f64> {
        self.inner.append(input)
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        input: PyReadonlyArray1<f64>,
    ) -> PyResult<Py<PyArray1<f64>>> {
        let values = self.inner.extend(input.as_slice()?.iter().copied());
        Ok(to_py_array(py, values_from(values)))
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn reset(&mut self) {
        self.inner.reset();
    }
}

#[pyclass]
pub struct StatefulDema {
    inner: Dema,
}

#[pymethods]
impl StatefulDema {
    #[new]
    #[pyo3(signature = (timeperiod=30))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: Dema::new(timeperiod).map_err(py_value_error)?,
        })
    }

    fn append(&mut self, input: f64) -> Option<f64> {
        self.inner.append(input)
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        input: PyReadonlyArray1<f64>,
    ) -> PyResult<Py<PyArray1<f64>>> {
        let values = self.inner.extend(input.as_slice()?.iter().copied());
        Ok(to_py_array(py, values_from(values)))
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn reset(&mut self) {
        self.inner.reset();
    }
}

#[pyclass]
pub struct StatefulTema {
    inner: Tema,
}

#[pymethods]
impl StatefulTema {
    #[new]
    #[pyo3(signature = (timeperiod=30))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: Tema::new(timeperiod).map_err(py_value_error)?,
        })
    }

    fn append(&mut self, input: f64) -> Option<f64> {
        self.inner.append(input)
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        input: PyReadonlyArray1<f64>,
    ) -> PyResult<Py<PyArray1<f64>>> {
        let values = self.inner.extend(input.as_slice()?.iter().copied());
        Ok(to_py_array(py, values_from(values)))
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn reset(&mut self) {
        self.inner.reset();
    }
}

#[pyclass]
pub struct StatefulTrima {
    inner: Trima,
}

#[pymethods]
impl StatefulTrima {
    #[new]
    #[pyo3(signature = (timeperiod=30))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: Trima::new(timeperiod).map_err(py_value_error)?,
        })
    }

    fn append(&mut self, input: f64) -> Option<f64> {
        self.inner.append(input)
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        input: PyReadonlyArray1<f64>,
    ) -> PyResult<Py<PyArray1<f64>>> {
        let values = self.inner.extend(input.as_slice()?.iter().copied());
        Ok(to_py_array(py, values_from(values)))
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn reset(&mut self) {
        self.inner.reset();
    }
}

#[pyclass]
pub struct StatefulRsi {
    inner: Rsi,
}

#[pymethods]
impl StatefulRsi {
    #[new]
    #[pyo3(signature = (timeperiod=14))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: Rsi::new(timeperiod).map_err(py_value_error)?,
        })
    }

    fn append(&mut self, input: f64) -> Option<f64> {
        self.inner.append(input)
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        input: PyReadonlyArray1<f64>,
    ) -> PyResult<Py<PyArray1<f64>>> {
        let values = self.inner.extend(input.as_slice()?.iter().copied());
        Ok(to_py_array(py, values_from(values)))
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn reset(&mut self) {
        self.inner.reset();
    }
}

#[pyclass]
pub struct StatefulAtr {
    inner: AverageTrueRange,
}

#[pymethods]
impl StatefulAtr {
    #[new]
    #[pyo3(signature = (timeperiod=14))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: AverageTrueRange::new(timeperiod).map_err(py_value_error)?,
        })
    }

    fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        self.inner.append(high, low, close)
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<Py<PyArray1<f64>>> {
        let high = high.as_slice()?;
        let low = low.as_slice()?;
        let close = close.as_slice()?;
        if high.len() != low.len() || high.len() != close.len() {
            return Err(PyValueError::new_err(
                "high, low, and close must have equal lengths",
            ));
        }
        let values = high
            .iter()
            .zip(low.iter())
            .zip(close.iter())
            .map(|((&high, &low), &close)| self.inner.append(high, low, close));
        Ok(to_py_array(py, values_from(values)))
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn reset(&mut self) {
        self.inner.reset();
    }
}

#[pyclass]
pub struct StatefulTrange {
    inner: TrueRange,
}

#[pymethods]
impl StatefulTrange {
    #[new]
    fn new() -> Self {
        Self {
            inner: TrueRange::new(),
        }
    }

    fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        self.inner.append(high, low, close)
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<Py<PyArray1<f64>>> {
        let high = high.as_slice()?;
        let low = low.as_slice()?;
        let close = close.as_slice()?;
        if high.len() != low.len() || high.len() != close.len() {
            return Err(PyValueError::new_err(
                "high, low, and close must have equal lengths",
            ));
        }
        Ok(to_py_array(
            py,
            values_from(
                high.iter()
                    .zip(low)
                    .zip(close)
                    .map(|((&high, &low), &close)| self.inner.append(high, low, close)),
            ),
        ))
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }
    fn reset(&mut self) {
        self.inner.reset();
    }
}

#[pyclass]
pub struct StatefulNatr {
    inner: NormalizedAverageTrueRange,
}

#[pymethods]
impl StatefulNatr {
    #[new]
    #[pyo3(signature = (timeperiod=14))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: NormalizedAverageTrueRange::new(timeperiod).map_err(py_value_error)?,
        })
    }

    fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        self.inner.append(high, low, close)
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<Py<PyArray1<f64>>> {
        let high = high.as_slice()?;
        let low = low.as_slice()?;
        let close = close.as_slice()?;
        if high.len() != low.len() || high.len() != close.len() {
            return Err(PyValueError::new_err(
                "high, low, and close must have equal lengths",
            ));
        }
        Ok(to_py_array(
            py,
            values_from(
                high.iter()
                    .zip(low)
                    .zip(close)
                    .map(|((&high, &low), &close)| self.inner.append(high, low, close)),
            ),
        ))
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }
    fn reset(&mut self) {
        self.inner.reset();
    }
}

#[pyclass]
pub struct StatefulMacd {
    inner: Macd,
}

#[pyclass]
pub struct StatefulMacdFix {
    inner: MacdFix,
}

#[pyclass]
pub struct StatefulMacdExt {
    inner: MacdExt,
}

#[pyclass]
pub struct StatefulMavp {
    inner: Mavp,
}

#[pyclass]
pub struct StatefulHtTrendline {
    inner: HtTrendline,
}

#[pyclass]
pub struct StatefulAdx {
    inner: Adx,
}

#[pyclass]
pub struct StatefulAdxr {
    inner: Adxr,
}

#[pyclass]
pub struct StatefulDx {
    inner: Dx,
}

#[pyclass]
pub struct StatefulStochf {
    inner: Stochf,
}

#[pyclass]
pub struct StatefulStoch {
    inner: Stoch,
}

#[pyclass]
pub struct StatefulStochrsi {
    inner: Stochrsi,
}

#[pyclass]
pub struct StatefulMama {
    inner: Mama,
}

#[pymethods]
impl StatefulMama {
    #[new]
    #[pyo3(signature = (fastlimit=0.5, slowlimit=0.05))]
    fn new(fastlimit: f64, slowlimit: f64) -> PyResult<Self> {
        Ok(Self {
            inner: Mama::new(fastlimit, slowlimit).map_err(py_value_error)?,
        })
    }

    fn append(&mut self, input: f64) -> Option<(f64, f64)> {
        self.inner
            .append(input)
            .map(|value| (value.mama, value.fama))
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        input: PyReadonlyArray1<f64>,
    ) -> PyResult<(Py<PyArray1<f64>>, Py<PyArray1<f64>>)> {
        let mut mama = Vec::with_capacity(input.len()?);
        let mut fama = Vec::with_capacity(input.len()?);
        for input in input.as_slice()?.iter().copied() {
            if let Some(value) = self.inner.append(input) {
                mama.push(value.mama);
                fama.push(value.fama);
            } else {
                mama.push(f64::NAN);
                fama.push(f64::NAN);
            }
        }
        Ok((to_py_array(py, mama), to_py_array(py, fama)))
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64)> {
        self.inner.value().map(|value| (value.mama, value.fama))
    }

    fn reset(&mut self) {
        self.inner.reset();
    }
}

#[pymethods]
impl StatefulMacd {
    #[new]
    #[pyo3(signature = (fastperiod=12, slowperiod=26, signalperiod=9))]
    fn new(fastperiod: usize, slowperiod: usize, signalperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: Macd::new(fastperiod, slowperiod, signalperiod).map_err(py_value_error)?,
        })
    }

    fn append(&mut self, input: f64) -> Option<(f64, f64, f64)> {
        self.inner
            .append(input)
            .map(|value| (value.macd, value.signal, value.histogram))
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        input: PyReadonlyArray1<f64>,
    ) -> PyResult<(Py<PyArray1<f64>>, Py<PyArray1<f64>>, Py<PyArray1<f64>>)> {
        let mut macd = Vec::with_capacity(input.len()?);
        let mut signal = Vec::with_capacity(input.len()?);
        let mut histogram = Vec::with_capacity(input.len()?);
        for value in input.as_slice()?.iter().copied() {
            match self.inner.append(value) {
                Some(value) => {
                    macd.push(value.macd);
                    signal.push(value.signal);
                    histogram.push(value.histogram);
                }
                None => {
                    macd.push(f64::NAN);
                    signal.push(f64::NAN);
                    histogram.push(f64::NAN);
                }
            }
        }
        Ok((
            to_py_array(py, macd),
            to_py_array(py, signal),
            to_py_array(py, histogram),
        ))
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64, f64)> {
        self.inner
            .value()
            .map(|value| (value.macd, value.signal, value.histogram))
    }

    fn reset(&mut self) {
        self.inner.reset();
    }
}

#[pymethods]
impl StatefulMacdFix {
    #[new]
    #[pyo3(signature = (signalperiod=9))]
    fn new(signalperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: MacdFix::new(signalperiod).map_err(py_value_error)?,
        })
    }

    fn append(&mut self, input: f64) -> Option<(f64, f64, f64)> {
        self.inner
            .append(input)
            .map(|value| (value.macd, value.signal, value.histogram))
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        input: PyReadonlyArray1<f64>,
    ) -> PyResult<(Py<PyArray1<f64>>, Py<PyArray1<f64>>, Py<PyArray1<f64>>)> {
        let mut macd = Vec::with_capacity(input.len()?);
        let mut signal = Vec::with_capacity(input.len()?);
        let mut histogram = Vec::with_capacity(input.len()?);
        for value in input.as_slice()?.iter().copied() {
            match self.inner.append(value) {
                Some(value) => {
                    macd.push(value.macd);
                    signal.push(value.signal);
                    histogram.push(value.histogram);
                }
                None => {
                    macd.push(f64::NAN);
                    signal.push(f64::NAN);
                    histogram.push(f64::NAN);
                }
            }
        }
        Ok((
            to_py_array(py, macd),
            to_py_array(py, signal),
            to_py_array(py, histogram),
        ))
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64, f64)> {
        self.inner
            .value()
            .map(|value| (value.macd, value.signal, value.histogram))
    }

    fn reset(&mut self) {
        self.inner.reset();
    }
}

#[pymethods]
impl StatefulStochf {
    #[new]
    #[pyo3(signature = (fastk_period=5, fastd_period=3, fastd_matype=0))]
    fn new(fastk_period: usize, fastd_period: usize, fastd_matype: i32) -> PyResult<Self> {
        let ma_type = MaType::try_from(fastd_matype).map_err(py_value_error)?;
        Ok(Self {
            inner: Stochf::new(fastk_period, fastd_period, ma_type).map_err(py_value_error)?,
        })
    }

    fn append(&mut self, high: f64, low: f64, close: f64) -> Option<(f64, f64)> {
        self.inner
            .append(high, low, close)
            .map(|value| (value.fastk, value.fastd))
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<(Py<PyArray1<f64>>, Py<PyArray1<f64>>)> {
        let high = high.as_slice()?;
        let low = low.as_slice()?;
        let close = close.as_slice()?;
        if high.len() != low.len() || high.len() != close.len() {
            return Err(PyValueError::new_err(
                "high, low, and close must have equal lengths",
            ));
        }
        let mut fastk = Vec::with_capacity(high.len());
        let mut fastd = Vec::with_capacity(high.len());
        for ((&high, &low), &close) in high.iter().zip(low).zip(close) {
            match self.inner.append(high, low, close) {
                Some(value) => {
                    fastk.push(value.fastk);
                    fastd.push(value.fastd);
                }
                None => {
                    fastk.push(f64::NAN);
                    fastd.push(f64::NAN);
                }
            }
        }
        Ok((to_py_array(py, fastk), to_py_array(py, fastd)))
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64)> {
        self.inner.value().map(|value| (value.fastk, value.fastd))
    }

    fn reset(&mut self) {
        self.inner.reset();
    }
}

#[pymethods]
impl StatefulStoch {
    #[new]
    #[pyo3(signature = (fastk_period=5, slowk_period=3, slowk_matype=0, slowd_period=3, slowd_matype=0))]
    fn new(
        fastk_period: usize,
        slowk_period: usize,
        slowk_matype: i32,
        slowd_period: usize,
        slowd_matype: i32,
    ) -> PyResult<Self> {
        let slowk_type = MaType::try_from(slowk_matype).map_err(py_value_error)?;
        let slowd_type = MaType::try_from(slowd_matype).map_err(py_value_error)?;
        Ok(Self {
            inner: Stoch::new(
                fastk_period,
                slowk_period,
                slowk_type,
                slowd_period,
                slowd_type,
            )
            .map_err(py_value_error)?,
        })
    }

    fn append(&mut self, high: f64, low: f64, close: f64) -> Option<(f64, f64)> {
        self.inner
            .append(high, low, close)
            .map(|value| (value.slowk, value.slowd))
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<(Py<PyArray1<f64>>, Py<PyArray1<f64>>)> {
        let high = high.as_slice()?;
        let low = low.as_slice()?;
        let close = close.as_slice()?;
        if high.len() != low.len() || high.len() != close.len() {
            return Err(PyValueError::new_err(
                "high, low, and close must have equal lengths",
            ));
        }
        let mut slowk = Vec::with_capacity(high.len());
        let mut slowd = Vec::with_capacity(high.len());
        for ((&high, &low), &close) in high.iter().zip(low).zip(close) {
            match self.inner.append(high, low, close) {
                Some(value) => {
                    slowk.push(value.slowk);
                    slowd.push(value.slowd);
                }
                None => {
                    slowk.push(f64::NAN);
                    slowd.push(f64::NAN);
                }
            }
        }
        Ok((to_py_array(py, slowk), to_py_array(py, slowd)))
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64)> {
        self.inner.value().map(|value| (value.slowk, value.slowd))
    }

    fn reset(&mut self) {
        self.inner.reset();
    }
}

#[pymethods]
impl StatefulStochrsi {
    #[new]
    #[pyo3(signature = (timeperiod=14, fastk_period=5, fastd_period=3, fastd_matype=0))]
    fn new(
        timeperiod: usize,
        fastk_period: usize,
        fastd_period: usize,
        fastd_matype: i32,
    ) -> PyResult<Self> {
        let ma_type = MaType::try_from(fastd_matype).map_err(py_value_error)?;
        Ok(Self {
            inner: Stochrsi::new(timeperiod, fastk_period, fastd_period, ma_type)
                .map_err(py_value_error)?,
        })
    }

    fn append(&mut self, input: f64) -> Option<(f64, f64)> {
        self.inner
            .append(input)
            .map(|value| (value.fastk, value.fastd))
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        input: PyReadonlyArray1<f64>,
    ) -> PyResult<(Py<PyArray1<f64>>, Py<PyArray1<f64>>)> {
        let mut fastk = Vec::with_capacity(input.len()?);
        let mut fastd = Vec::with_capacity(input.len()?);
        for input in input.as_slice()?.iter().copied() {
            match self.inner.append(input) {
                Some(value) => {
                    fastk.push(value.fastk);
                    fastd.push(value.fastd);
                }
                None => {
                    fastk.push(f64::NAN);
                    fastd.push(f64::NAN);
                }
            }
        }
        Ok((to_py_array(py, fastk), to_py_array(py, fastd)))
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64)> {
        self.inner.value().map(|value| (value.fastk, value.fastd))
    }

    fn reset(&mut self) {
        self.inner.reset();
    }
}

#[pymethods]
impl StatefulMacdExt {
    #[new]
    #[pyo3(signature = (fastperiod=12, fastmatype=1, slowperiod=26, slowmatype=1, signalperiod=9, signalmatype=1))]
    fn new(
        fastperiod: usize,
        fastmatype: i32,
        slowperiod: usize,
        slowmatype: i32,
        signalperiod: usize,
        signalmatype: i32,
    ) -> PyResult<Self> {
        let fast_type = MaType::try_from(fastmatype).map_err(py_value_error)?;
        let slow_type = MaType::try_from(slowmatype).map_err(py_value_error)?;
        let signal_type = MaType::try_from(signalmatype).map_err(py_value_error)?;
        Ok(Self {
            inner: MacdExt::new(
                fastperiod,
                fast_type,
                slowperiod,
                slow_type,
                signalperiod,
                signal_type,
            )
            .map_err(py_value_error)?,
        })
    }

    fn append(&mut self, input: f64) -> Option<(f64, f64, f64)> {
        self.inner
            .append(input)
            .map(|value| (value.macd, value.signal, value.histogram))
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        input: PyReadonlyArray1<f64>,
    ) -> PyResult<(Py<PyArray1<f64>>, Py<PyArray1<f64>>, Py<PyArray1<f64>>)> {
        let mut macd = Vec::with_capacity(input.len()?);
        let mut signal = Vec::with_capacity(input.len()?);
        let mut histogram = Vec::with_capacity(input.len()?);
        for input in input.as_slice()?.iter().copied() {
            match self.inner.append(input) {
                Some(value) => {
                    macd.push(value.macd);
                    signal.push(value.signal);
                    histogram.push(value.histogram);
                }
                None => {
                    macd.push(f64::NAN);
                    signal.push(f64::NAN);
                    histogram.push(f64::NAN);
                }
            }
        }
        Ok((
            to_py_array(py, macd),
            to_py_array(py, signal),
            to_py_array(py, histogram),
        ))
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64, f64)> {
        self.inner
            .value()
            .map(|value| (value.macd, value.signal, value.histogram))
    }

    fn reset(&mut self) {
        self.inner.reset();
    }
}

#[pymethods]
impl StatefulMavp {
    #[new]
    #[pyo3(signature = (minperiod=2, maxperiod=30, matype=0))]
    fn new(minperiod: usize, maxperiod: usize, matype: i32) -> PyResult<Self> {
        let ma_type = MaType::try_from(matype).map_err(py_value_error)?;
        Ok(Self {
            inner: Mavp::new(minperiod, maxperiod, ma_type).map_err(py_value_error)?,
        })
    }

    fn append(&mut self, input: f64, period: f64) -> Option<f64> {
        self.inner.append(input, period)
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        input: PyReadonlyArray1<f64>,
        periods: PyReadonlyArray1<f64>,
    ) -> PyResult<Py<PyArray1<f64>>> {
        let input = input.as_slice()?;
        let periods = periods.as_slice()?;
        if input.len() != periods.len() {
            return Err(PyValueError::new_err(
                "input and periods must have equal lengths",
            ));
        }
        Ok(to_py_array(
            py,
            input
                .iter()
                .zip(periods)
                .map(|(&input, &period)| self.inner.append(input, period).unwrap_or(f64::NAN))
                .collect(),
        ))
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn reset(&mut self) {
        self.inner.reset();
    }
}

#[pymethods]
impl StatefulHtTrendline {
    #[new]
    fn new() -> Self {
        Self {
            inner: HtTrendline::new(),
        }
    }

    fn append(&mut self, input: f64) -> Option<f64> {
        self.inner.append(input)
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        input: PyReadonlyArray1<f64>,
    ) -> PyResult<Py<PyArray1<f64>>> {
        Ok(to_py_array(
            py,
            input
                .as_slice()?
                .iter()
                .map(|&input| self.inner.append(input).unwrap_or(f64::NAN))
                .collect(),
        ))
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn reset(&mut self) {
        self.inner.reset();
    }
}

#[pymethods]
impl StatefulAdx {
    #[new]
    #[pyo3(signature = (timeperiod=14))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: Adx::new(timeperiod).map_err(py_value_error)?,
        })
    }

    fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        self.inner.append(high, low, close)
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<Py<PyArray1<f64>>> {
        let high = high.as_slice()?;
        let low = low.as_slice()?;
        let close = close.as_slice()?;
        if high.len() != low.len() || high.len() != close.len() {
            return Err(PyValueError::new_err(
                "high, low, and close must have equal lengths",
            ));
        }
        Ok(to_py_array(
            py,
            high.iter()
                .zip(low)
                .zip(close)
                .map(|((&high, &low), &close)| {
                    self.inner.append(high, low, close).unwrap_or(f64::NAN)
                })
                .collect(),
        ))
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn reset(&mut self) {
        self.inner.reset();
    }
}

#[pymethods]
impl StatefulAdxr {
    #[new]
    #[pyo3(signature = (timeperiod=14))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: Adxr::new(timeperiod).map_err(py_value_error)?,
        })
    }

    fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        self.inner.append(high, low, close)
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<Py<PyArray1<f64>>> {
        let high = high.as_slice()?;
        let low = low.as_slice()?;
        let close = close.as_slice()?;
        if high.len() != low.len() || high.len() != close.len() {
            return Err(PyValueError::new_err(
                "high, low, and close must have equal lengths",
            ));
        }
        Ok(to_py_array(
            py,
            high.iter()
                .zip(low)
                .zip(close)
                .map(|((&high, &low), &close)| {
                    self.inner.append(high, low, close).unwrap_or(f64::NAN)
                })
                .collect(),
        ))
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn reset(&mut self) {
        self.inner.reset();
    }
}

#[pymethods]
impl StatefulDx {
    #[new]
    #[pyo3(signature = (timeperiod=14))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: Dx::new(timeperiod).map_err(py_value_error)?,
        })
    }

    fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        self.inner.append(high, low, close)
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<Py<PyArray1<f64>>> {
        let high = high.as_slice()?;
        let low = low.as_slice()?;
        let close = close.as_slice()?;
        if high.len() != low.len() || high.len() != close.len() {
            return Err(PyValueError::new_err(
                "high, low, and close must have equal lengths",
            ));
        }
        Ok(to_py_array(
            py,
            high.iter()
                .zip(low)
                .zip(close)
                .map(|((&high, &low), &close)| {
                    self.inner.append(high, low, close).unwrap_or(f64::NAN)
                })
                .collect(),
        ))
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn reset(&mut self) {
        self.inner.reset();
    }
}
