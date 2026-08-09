//! PyO3 adapters for the incremental core API.

use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::{
    self, AverageDirectionalIndex, AverageDirectionalIndexRating,
    AverageTrueRange as CoreAverageTrueRange, CommodityChannelIndex, DirectionalMovementIndex,
    DoubleExponentialMovingAverage, EvenBetterSinewave, ExponentialMovingAverage,
    FastStochasticOscillator, HilbertTransformTrendline, IntradayMomentumIndex, JurikMovingAverage,
    KlingerVolumeOscillator, MesaAdaptiveMovingAverage, Momentum as CoreMomentum,
    MovingAverageConvergenceDivergence, MovingAverageConvergenceDivergenceExtended,
    MovingAverageConvergenceDivergenceFixed,
    NormalizedAverageTrueRange as CoreNormalizedAverageTrueRange, OpeningRange,
    ParabolicMovingAverageStop, PivotPoints, PremiumDiscount, RateOfChange as CoreRateOfChange,
    RateOfChangePercent as CoreRateOfChangePercent, RateOfChangeRatio as CoreRateOfChangeRatio,
    RateOfChangeRatioPercent as CoreRateOfChangeRatioPercent, RelativeMomentumIndex,
    RollingMidpoint, RollingMidprice, SessionVolumeLevels, SimpleMovingAverage,
    SmoothedTrendChannel, StochasticOscillator, StochasticRelativeStrengthIndex,
    StreamingIndicator, TomDeMarkSequential, TriangularMovingAverage,
    TripleExponentialMovingAverage, TrueRange as CoreTrueRange,
    VariablePeriodMovingAverage as CoreVariablePeriodMovingAverage, WeightedMovingAverage,
};
use taflow::MaType;

use crate::conversion::to_py_array;

fn py_value_error(error: impl ToString) -> PyErr {
    PyValueError::new_err(error.to_string())
}

/// Appends `Option<f64>` results straight into a Rust-side output cache,
/// NaN-filling warm-up. One pass, no temporary `Vec`.
fn extend_from_options<I>(cache: &mut Vec<f64>, values: I)
where
    I: IntoIterator<Item = Option<f64>>,
{
    let values = values.into_iter();
    cache.reserve(values.size_hint().0);
    cache.extend(values.map(|value| value.unwrap_or(f64::NAN)));
}

fn push_option(cache: &mut Vec<f64>, value: Option<f64>) -> Option<f64> {
    cache.push(value.unwrap_or(f64::NAN));
    value
}

macro_rules! scalar_state_class {
    ($class:ident, $inner:ty, $default_period:literal) => {
        #[pyclass]
        pub struct $class {
            inner: $inner,
            outputs: Vec<f64>,
        }

        #[pymethods]
        impl $class {
            #[new]
            #[pyo3(signature = (timeperiod=$default_period))]
            fn new(timeperiod: usize) -> PyResult<Self> {
                Ok(Self {
                    inner: <$inner>::new(timeperiod).map_err(py_value_error)?,
                    outputs: Vec::new(),
                })
            }

            fn append(&mut self, input: f64) -> Option<f64> {
                push_option(&mut self.outputs, self.inner.append(input))
            }

            fn extend(&mut self, py: Python<'_>, input: PyReadonlyArray1<f64>) -> PyResult<()> {
                let input = input.as_slice()?;
                let outputs = &mut self.outputs;
                py.allow_threads(|| self.inner.extend_slice_into(input, outputs));
                Ok(())
            }

            fn compute(&self, py: Python<'_>) -> Py<PyArray1<f64>> {
                to_py_array(py, self.outputs.clone())
            }

            fn __len__(&self) -> usize {
                self.outputs.len()
            }

            #[getter]
            fn value(&self) -> Option<f64> {
                self.inner.value()
            }

            fn reset(&mut self) {
                self.inner.reset();
                self.outputs.clear();
            }
        }
    };
}

scalar_state_class!(Momentum, CoreMomentum, 14);
scalar_state_class!(RateOfChange, CoreRateOfChange, 14);
scalar_state_class!(RateOfChangePercent, CoreRateOfChangePercent, 14);
scalar_state_class!(RateOfChangeRatio, CoreRateOfChangeRatio, 14);
scalar_state_class!(RateOfChangeRatioPercent, CoreRateOfChangeRatioPercent, 14);
scalar_state_class!(StatefulMidpoint, RollingMidpoint, 14);
scalar_state_class!(StatefulMax, stream::RollingMax, 30);
scalar_state_class!(StatefulMaxindex, stream::RollingArgmax, 30);
scalar_state_class!(StatefulMin, stream::RollingMin, 30);
scalar_state_class!(StatefulMinindex, stream::RollingArgmin, 30);
scalar_state_class!(StatefulSum, stream::RollingSum, 30);
scalar_state_class!(StatefulAvgdev, stream::RollingAverageDeviation, 14);
scalar_state_class!(StatefulCmo, stream::ChandeMomentumOscillator, 14);
scalar_state_class!(StatefulKama, stream::KaufmanAdaptiveMovingAverage, 30);
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
        Ok(Self {
            inner: PremiumDiscount::new(window).map_err(py_value_error)?,
            zones: Vec::new(),
            equilibrium: Vec::new(),
        })
    }
    fn append(&mut self, close: f64) -> (i32, f64) {
        let value = self.inner.append(close);
        self.zones.push(value.0);
        self.equilibrium.push(value.1);
        value
    }
    fn extend(&mut self, close: PyReadonlyArray1<f64>) -> PyResult<()> {
        for &value in close.as_slice()? {
            self.append(value);
        }
        Ok(())
    }
    fn compute<'py>(
        &self,
        py: Python<'py>,
    ) -> (Bound<'py, PyArray1<i32>>, Bound<'py, PyArray1<f64>>) {
        (
            PyArray1::from_vec(py, self.zones.clone()),
            PyArray1::from_vec(py, self.equilibrium.clone()),
        )
    }
    #[getter]
    fn value(&self) -> Option<(i32, f64)> {
        self.inner.value()
    }
    fn __len__(&self) -> usize {
        self.zones.len()
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.zones.clear();
        self.equilibrium.clear();
    }
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

/// Native state adapter for classic pivot levels.
#[pyclass]
pub struct StatefulPivotPoints {
    inner: PivotPoints,
    levels: [Vec<f64>; 5],
}

#[pymethods]
impl StatefulPivotPoints {
    #[new]
    fn new() -> Self {
        Self {
            inner: PivotPoints::new(),
            levels: std::array::from_fn(|_| Vec::new()),
        }
    }
    fn append(
        &mut self,
        high: f64,
        low: f64,
        close: f64,
        anchor: bool,
    ) -> (f64, f64, f64, f64, f64) {
        let value = self.inner.append(high, low, close, anchor);
        let values = [value.0, value.1, value.2, value.3, value.4];
        for (index, level) in values.iter().enumerate() {
            self.levels[index].push(*level);
        }
        value
    }
    fn extend(
        &mut self,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
        anchor: PyReadonlyArray1<bool>,
    ) -> PyResult<()> {
        let (high, low, close, anchor) = (
            high.as_slice()?,
            low.as_slice()?,
            close.as_slice()?,
            anchor.as_slice()?,
        );
        if high.len() != low.len() || high.len() != close.len() || high.len() != anchor.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        for (((&high, &low), &close), &anchor) in high.iter().zip(low).zip(close).zip(anchor) {
            self.append(high, low, close, anchor);
        }
        Ok(())
    }
    fn compute<'py>(
        &self,
        py: Python<'py>,
    ) -> (
        Bound<'py, PyArray1<f64>>,
        Bound<'py, PyArray1<f64>>,
        Bound<'py, PyArray1<f64>>,
        Bound<'py, PyArray1<f64>>,
        Bound<'py, PyArray1<f64>>,
    ) {
        (
            PyArray1::from_vec(py, self.levels[0].clone()),
            PyArray1::from_vec(py, self.levels[1].clone()),
            PyArray1::from_vec(py, self.levels[2].clone()),
            PyArray1::from_vec(py, self.levels[3].clone()),
            PyArray1::from_vec(py, self.levels[4].clone()),
        )
    }
    #[getter]
    fn value(&self) -> (f64, f64, f64, f64, f64) {
        self.inner.value()
    }
    fn __len__(&self) -> usize {
        self.levels[0].len()
    }

    fn reset(&mut self) {
        self.inner.reset();
        for level in &mut self.levels {
            level.clear();
        }
    }
}

#[pymethods]
impl StatefulTomDeMarkSequential {
    #[new]
    fn new() -> Self {
        Self {
            inner: TomDeMarkSequential::new(),
            buys: Vec::new(),
            sells: Vec::new(),
        }
    }
    fn append(&mut self, close: f64) -> (i32, i32) {
        let value = self.inner.append(close);
        self.buys.push(value.0);
        self.sells.push(value.1);
        value
    }
    fn extend(&mut self, close: PyReadonlyArray1<f64>) -> PyResult<()> {
        for &value in close.as_slice()? {
            self.append(value);
        }
        Ok(())
    }
    fn compute<'py>(
        &self,
        py: Python<'py>,
    ) -> (Bound<'py, PyArray1<i32>>, Bound<'py, PyArray1<i32>>) {
        (
            PyArray1::from_vec(py, self.buys.clone()),
            PyArray1::from_vec(py, self.sells.clone()),
        )
    }
    #[getter]
    fn value(&self) -> Option<(i32, i32)> {
        self.inner.value()
    }
    fn __len__(&self) -> usize {
        self.buys.len()
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.buys.clear();
        self.sells.clear();
    }
}

#[pymethods]
impl StatefulParabolicMovingAverageStop {
    #[new]
    #[pyo3(signature = (length=10, multiplier=3.0))]
    fn new(length: usize, multiplier: f64) -> PyResult<Self> {
        Ok(Self {
            inner: ParabolicMovingAverageStop::new(length, multiplier).map_err(py_value_error)?,
            stops: Vec::new(),
            trends: Vec::new(),
        })
    }
    fn append(&mut self, high: f64, low: f64, close: f64) -> (f64, i32) {
        let value = self.inner.append(high, low, close);
        self.stops.push(value.0);
        self.trends.push(value.1);
        value
    }
    fn extend(
        &mut self,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (high, low, close) = (high.as_slice()?, low.as_slice()?, close.as_slice()?);
        if high.len() != low.len() || high.len() != close.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        for ((&high, &low), &close) in high.iter().zip(low).zip(close) {
            self.append(high, low, close);
        }
        Ok(())
    }
    fn compute<'py>(
        &self,
        py: Python<'py>,
    ) -> (Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<i32>>) {
        (
            PyArray1::from_vec(py, self.stops.clone()),
            PyArray1::from_vec(py, self.trends.clone()),
        )
    }
    #[getter]
    fn value(&self) -> Option<(f64, i32)> {
        self.inner.value()
    }
    fn __len__(&self) -> usize {
        self.stops.len()
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.stops.clear();
        self.trends.clear();
    }
}

#[pymethods]
impl StatefulKlingerVolumeOscillator {
    #[new]
    #[pyo3(signature = (fast=34, slow=55, signal=13))]
    fn new(fast: usize, slow: usize, signal: usize) -> PyResult<Self> {
        Ok(Self {
            inner: KlingerVolumeOscillator::new(fast, slow, signal).map_err(py_value_error)?,
            oscillator: Vec::new(),
            signal: Vec::new(),
        })
    }
    fn append(&mut self, high: f64, low: f64, close: f64, volume: f64) -> (f64, f64) {
        let value = self.inner.append(high, low, close, volume);
        self.oscillator.push(value.0);
        self.signal.push(value.1);
        value
    }
    fn extend(
        &mut self,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
        volume: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (high, low, close, volume) = (
            high.as_slice()?,
            low.as_slice()?,
            close.as_slice()?,
            volume.as_slice()?,
        );
        if high.len() != low.len() || high.len() != close.len() || high.len() != volume.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        for (((&high, &low), &close), &volume) in high.iter().zip(low).zip(close).zip(volume) {
            self.append(high, low, close, volume);
        }
        Ok(())
    }
    fn compute<'py>(
        &self,
        py: Python<'py>,
    ) -> (Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>) {
        (
            PyArray1::from_vec(py, self.oscillator.clone()),
            PyArray1::from_vec(py, self.signal.clone()),
        )
    }
    #[getter]
    fn value(&self) -> Option<(f64, f64)> {
        self.inner.value()
    }
    fn __len__(&self) -> usize {
        self.oscillator.len()
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.oscillator.clear();
        self.signal.clear();
    }
}

#[pymethods]
impl StatefulSessionVolumeLevels {
    #[new]
    #[pyo3(signature = (bins=24, value_area=0.7))]
    fn new(bins: usize, value_area: f64) -> PyResult<Self> {
        Ok(Self {
            inner: SessionVolumeLevels::new(bins, value_area).map_err(py_value_error)?,
            poc: Vec::new(),
            value_area_high: Vec::new(),
            value_area_low: Vec::new(),
        })
    }
    fn append(
        &mut self,
        high: f64,
        low: f64,
        close: f64,
        volume: f64,
        anchor: bool,
    ) -> (f64, f64, f64) {
        let value = self.inner.append(high, low, close, volume, anchor);
        self.poc.push(value.0);
        self.value_area_high.push(value.1);
        self.value_area_low.push(value.2);
        value
    }
    fn extend(
        &mut self,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
        volume: PyReadonlyArray1<f64>,
        anchor: PyReadonlyArray1<bool>,
    ) -> PyResult<()> {
        let (high, low, close, volume, anchor) = (
            high.as_slice()?,
            low.as_slice()?,
            close.as_slice()?,
            volume.as_slice()?,
            anchor.as_slice()?,
        );
        if high.len() != low.len()
            || high.len() != close.len()
            || high.len() != volume.len()
            || high.len() != anchor.len()
        {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        for ((((&high, &low), &close), &volume), &anchor) in
            high.iter().zip(low).zip(close).zip(volume).zip(anchor)
        {
            self.append(high, low, close, volume, anchor);
        }
        Ok(())
    }
    fn compute<'py>(
        &self,
        py: Python<'py>,
    ) -> (
        Bound<'py, PyArray1<f64>>,
        Bound<'py, PyArray1<f64>>,
        Bound<'py, PyArray1<f64>>,
    ) {
        (
            PyArray1::from_vec(py, self.poc.clone()),
            PyArray1::from_vec(py, self.value_area_high.clone()),
            PyArray1::from_vec(py, self.value_area_low.clone()),
        )
    }
    #[getter]
    fn value(&self) -> Option<(f64, f64, f64)> {
        self.inner.value()
    }
    fn __len__(&self) -> usize {
        self.poc.len()
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.poc.clear();
        self.value_area_high.clear();
        self.value_area_low.clear();
    }
}

#[pymethods]
impl StatefulOpeningRange {
    #[new]
    #[pyo3(signature = (bars=30))]
    fn new(bars: usize) -> Self {
        Self {
            inner: OpeningRange::new(bars),
            highs: Vec::new(),
            lows: Vec::new(),
            breakouts: Vec::new(),
        }
    }
    fn append(&mut self, high: f64, low: f64, close: f64, anchor: bool) -> (f64, f64, i32) {
        let value = self.inner.append(high, low, close, anchor);
        self.highs.push(value.0);
        self.lows.push(value.1);
        self.breakouts.push(value.2);
        value
    }
    fn extend(
        &mut self,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
        anchor: PyReadonlyArray1<bool>,
    ) -> PyResult<()> {
        let (high, low, close, anchor) = (
            high.as_slice()?,
            low.as_slice()?,
            close.as_slice()?,
            anchor.as_slice()?,
        );
        if high.len() != low.len() || high.len() != close.len() || high.len() != anchor.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        for (((&high, &low), &close), &anchor) in high.iter().zip(low).zip(close).zip(anchor) {
            self.append(high, low, close, anchor);
        }
        Ok(())
    }
    fn compute<'py>(
        &self,
        py: Python<'py>,
    ) -> (
        Bound<'py, PyArray1<f64>>,
        Bound<'py, PyArray1<f64>>,
        Bound<'py, PyArray1<i32>>,
    ) {
        (
            PyArray1::from_vec(py, self.highs.clone()),
            PyArray1::from_vec(py, self.lows.clone()),
            PyArray1::from_vec(py, self.breakouts.clone()),
        )
    }
    #[getter]
    fn value(&self) -> Option<(f64, f64, i32)> {
        self.inner.value()
    }
    fn __len__(&self) -> usize {
        self.highs.len()
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.highs.clear();
        self.lows.clear();
        self.breakouts.clear();
    }
}

#[pymethods]
impl StatefulSmoothedTrendChannel {
    #[new]
    #[pyo3(signature = (length=10))]
    fn new(length: usize) -> PyResult<Self> {
        Ok(Self {
            inner: SmoothedTrendChannel::new(length).map_err(py_value_error)?,
            lower: Vec::new(),
            upper: Vec::new(),
        })
    }
    fn append(&mut self, high: f64, low: f64, close: f64) -> (f64, f64) {
        let value = self
            .inner
            .append(high, low, close)
            .unwrap_or((f64::NAN, f64::NAN));
        self.lower.push(value.0);
        self.upper.push(value.1);
        value
    }
    fn extend(
        &mut self,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (high, low, close) = (high.as_slice()?, low.as_slice()?, close.as_slice()?);
        if high.len() != low.len() || high.len() != close.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        for ((&high, &low), &close) in high.iter().zip(low).zip(close) {
            self.append(high, low, close);
        }
        Ok(())
    }
    fn compute<'py>(
        &self,
        py: Python<'py>,
    ) -> (Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>) {
        (
            PyArray1::from_vec(py, self.lower.clone()),
            PyArray1::from_vec(py, self.upper.clone()),
        )
    }
    #[getter]
    fn value(&self) -> Option<(f64, f64)> {
        self.inner.value()
    }
    fn __len__(&self) -> usize {
        self.lower.len()
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.lower.clear();
        self.upper.clear();
    }
}

#[pymethods]
impl StatefulJurikMovingAverage {
    #[new]
    #[pyo3(signature = (length=7, phase=0.0))]
    fn new(length: usize, phase: f64) -> PyResult<Self> {
        Ok(Self {
            inner: JurikMovingAverage::new(length, phase).map_err(py_value_error)?,
            output: Vec::new(),
        })
    }
    fn append(&mut self, input: f64) -> Option<f64> {
        let value = self.inner.append(input);
        self.output.push(value.unwrap_or(f64::NAN));
        value
    }
    fn extend(&mut self, input: PyReadonlyArray1<f64>) -> PyResult<()> {
        for &value in input.as_slice()? {
            self.append(value);
        }
        Ok(())
    }
    fn compute<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        PyArray1::from_vec(py, self.output.clone())
    }
    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }
    fn __len__(&self) -> usize {
        self.output.len()
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.output.clear();
    }
}

#[pymethods]
impl StatefulEvenBetterSinewave {
    #[new]
    #[pyo3(signature = (length=40))]
    fn new(length: usize) -> PyResult<Self> {
        Ok(Self {
            inner: EvenBetterSinewave::new(length).map_err(py_value_error)?,
            output: Vec::new(),
        })
    }
    fn append(&mut self, input: f64) -> Option<f64> {
        let value = self.inner.append(input);
        self.output.push(value.unwrap_or(f64::NAN));
        value
    }
    fn extend(&mut self, input: PyReadonlyArray1<f64>) -> PyResult<()> {
        for &value in input.as_slice()? {
            self.append(value);
        }
        Ok(())
    }
    fn compute<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        PyArray1::from_vec(py, self.output.clone())
    }
    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }
    fn __len__(&self) -> usize {
        self.output.len()
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.output.clear();
    }
}

#[pymethods]
impl StatefulRelativeMomentumIndex {
    #[new]
    #[pyo3(signature = (timeperiod=14, momentum=5))]
    fn new(timeperiod: usize, momentum: usize) -> PyResult<Self> {
        Ok(Self {
            inner: RelativeMomentumIndex::new(timeperiod, momentum).map_err(py_value_error)?,
            output: Vec::new(),
        })
    }

    fn append(&mut self, input: f64) -> Option<f64> {
        let value = self.inner.append(input);
        self.output.push(value.unwrap_or(f64::NAN));
        value
    }

    fn extend(&mut self, py: Python<'_>, input: PyReadonlyArray1<f64>) -> PyResult<()> {
        let input = input.as_slice()?;
        let output = &mut self.output;
        py.allow_threads(|| {
            output.reserve(input.len());
            for &value in input {
                output.push(self.inner.append(value).unwrap_or(f64::NAN));
            }
        });
        Ok(())
    }

    fn compute<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        PyArray1::from_vec(py, self.output.clone())
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn __len__(&self) -> usize {
        self.output.len()
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.output.clear();
    }
}

#[pyclass]
pub struct StatefulCci {
    inner: CommodityChannelIndex,
    outputs: Vec<f64>,
}

#[pymethods]
impl StatefulCci {
    #[new]
    #[pyo3(signature = (timeperiod=14))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: CommodityChannelIndex::new(timeperiod).map_err(py_value_error)?,
            outputs: Vec::new(),
        })
    }

    fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        push_option(&mut self.outputs, self.inner.append(high, low, close))
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let high = high.as_slice()?;
        let low = low.as_slice()?;
        let close = close.as_slice()?;
        let outputs = &mut self.outputs;
        py.allow_threads(|| {
            self.inner
                .extend_slice(high, low, close)
                .map(|values| extend_from_options(outputs, values))
        })
        .map_err(py_value_error)?;
        Ok(())
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn compute(&self, py: Python<'_>) -> Py<PyArray1<f64>> {
        to_py_array(py, self.outputs.clone())
    }

    fn __len__(&self) -> usize {
        self.outputs.len()
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.outputs.clear();
    }
}

#[pyclass]
pub struct StatefulImi {
    inner: IntradayMomentumIndex,
    outputs: Vec<f64>,
}

#[pymethods]
impl StatefulImi {
    #[new]
    #[pyo3(signature = (timeperiod=14))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: IntradayMomentumIndex::new(timeperiod).map_err(py_value_error)?,
            outputs: Vec::new(),
        })
    }

    fn append(&mut self, open: f64, close: f64) -> Option<f64> {
        push_option(&mut self.outputs, self.inner.append(open, close))
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        open: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let open = open.as_slice()?;
        let close = close.as_slice()?;
        if open.len() != close.len() {
            return Err(PyValueError::new_err(
                "open and close must have equal lengths",
            ));
        }
        let outputs = &mut self.outputs;
        py.allow_threads(|| {
            extend_from_options(
                outputs,
                open.iter()
                    .zip(close)
                    .map(|(&open, &close)| self.inner.append(open, close)),
            )
        });
        Ok(())
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn compute(&self, py: Python<'_>) -> Py<PyArray1<f64>> {
        to_py_array(py, self.outputs.clone())
    }

    fn __len__(&self) -> usize {
        self.outputs.len()
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.outputs.clear();
    }
}

#[pyclass]
pub struct StatefulT3 {
    inner: stream::TripleExponentialAverage,
    outputs: Vec<f64>,
}

#[pymethods]
impl StatefulT3 {
    #[new]
    #[pyo3(signature = (timeperiod=5, vfactor=0.7))]
    fn new(timeperiod: usize, vfactor: f64) -> PyResult<Self> {
        Ok(Self {
            inner: stream::TripleExponentialAverage::new(timeperiod, vfactor)
                .map_err(py_value_error)?,
            outputs: Vec::new(),
        })
    }

    fn append(&mut self, input: f64) -> Option<f64> {
        push_option(&mut self.outputs, self.inner.append(input))
    }

    fn extend(&mut self, py: Python<'_>, input: PyReadonlyArray1<f64>) -> PyResult<()> {
        let input = input.as_slice()?;
        let outputs = &mut self.outputs;
        py.allow_threads(|| self.inner.extend_slice_into(input, outputs));
        Ok(())
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn compute(&self, py: Python<'_>) -> Py<PyArray1<f64>> {
        to_py_array(py, self.outputs.clone())
    }

    fn __len__(&self) -> usize {
        self.outputs.len()
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.outputs.clear();
    }
}

macro_rules! oscillator_state_class {
    ($class:ident, $inner:ty) => {
        #[pyclass]
        pub struct $class {
            inner: $inner,
            outputs: Vec<f64>,
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
                    outputs: Vec::new(),
                })
            }

            fn append(&mut self, input: f64) -> Option<f64> {
                push_option(&mut self.outputs, self.inner.append(input))
            }

            fn extend(&mut self, py: Python<'_>, input: PyReadonlyArray1<f64>) -> PyResult<()> {
                let input = input.as_slice()?;
                let outputs = &mut self.outputs;
                py.allow_threads(|| self.inner.extend_slice_into(input, outputs));
                Ok(())
            }

            #[getter]
            fn value(&self) -> Option<f64> {
                self.inner.value()
            }

            fn compute(&self, py: Python<'_>) -> Py<PyArray1<f64>> {
                to_py_array(py, self.outputs.clone())
            }

            fn __len__(&self) -> usize {
                self.outputs.len()
            }

            fn reset(&mut self) {
                self.inner.reset();
                self.outputs.clear();
            }
        }
    };
}

oscillator_state_class!(StatefulApo, stream::AbsolutePriceOscillator);
oscillator_state_class!(StatefulPpo, stream::PercentagePriceOscillator);

#[pyclass]
pub struct StatefulMa {
    inner: stream::MovingAverage,
    outputs: Vec<f64>,
}

#[pymethods]
impl StatefulMa {
    #[new]
    #[pyo3(signature = (timeperiod=30, matype=0))]
    fn new(timeperiod: usize, matype: i32) -> PyResult<Self> {
        Ok(Self {
            inner: stream::MovingAverage::new(
                timeperiod,
                MaType::try_from(matype).map_err(py_value_error)?,
            )
            .map_err(py_value_error)?,
            outputs: Vec::new(),
        })
    }

    fn append(&mut self, input: f64) -> Option<f64> {
        push_option(&mut self.outputs, self.inner.append(input))
    }

    fn extend(&mut self, py: Python<'_>, input: PyReadonlyArray1<f64>) -> PyResult<()> {
        let input = input.as_slice()?;
        let outputs = &mut self.outputs;
        py.allow_threads(|| self.inner.extend_slice_into(input, outputs));
        Ok(())
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn compute(&self, py: Python<'_>) -> Py<PyArray1<f64>> {
        to_py_array(py, self.outputs.clone())
    }

    fn __len__(&self) -> usize {
        self.outputs.len()
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.outputs.clear();
    }
}

#[pyclass]
pub struct StatefulBbands {
    inner: stream::BollingerBands,
    uppers: Vec<f64>,
    middles: Vec<f64>,
    lowers: Vec<f64>,
}

#[pymethods]
impl StatefulBbands {
    #[new]
    #[pyo3(signature = (timeperiod=5, nbdevup=2.0, nbdevdn=2.0, matype=0))]
    fn new(timeperiod: usize, nbdevup: f64, nbdevdn: f64, matype: i32) -> PyResult<Self> {
        Ok(Self {
            inner: stream::BollingerBands::new(
                timeperiod,
                nbdevup,
                nbdevdn,
                MaType::try_from(matype).map_err(py_value_error)?,
            )
            .map_err(py_value_error)?,
            uppers: Vec::new(),
            middles: Vec::new(),
            lowers: Vec::new(),
        })
    }

    fn append(&mut self, input: f64) -> Option<(f64, f64, f64)> {
        let value = self
            .inner
            .append(input)
            .map(|value| (value.upper, value.middle, value.lower));
        let (upper, middle, lower) = value.unwrap_or((f64::NAN, f64::NAN, f64::NAN));
        self.uppers.push(upper);
        self.middles.push(middle);
        self.lowers.push(lower);
        value
    }

    fn extend(&mut self, py: Python<'_>, input: PyReadonlyArray1<f64>) -> PyResult<()> {
        let input = input.as_slice()?;
        let (uppers, middles, lowers) = (&mut self.uppers, &mut self.middles, &mut self.lowers);
        py.allow_threads(|| {
            self.inner
                .extend_slices_into(input, uppers, middles, lowers)
        });
        Ok(())
    }

    fn compute(&self, py: Python<'_>) -> (Py<PyArray1<f64>>, Py<PyArray1<f64>>, Py<PyArray1<f64>>) {
        (
            to_py_array(py, self.uppers.clone()),
            to_py_array(py, self.middles.clone()),
            to_py_array(py, self.lowers.clone()),
        )
    }

    fn __len__(&self) -> usize {
        self.uppers.len()
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64, f64)> {
        self.inner
            .value()
            .map(|value| (value.upper, value.middle, value.lower))
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.uppers.clear();
        self.middles.clear();
        self.lowers.clear();
    }
}

#[pyclass]
pub struct StatefulAccbands {
    inner: stream::AccelerationBands,
    uppers: Vec<f64>,
    middles: Vec<f64>,
    lowers: Vec<f64>,
}

#[pymethods]
impl StatefulAccbands {
    #[new]
    #[pyo3(signature = (timeperiod=20))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: stream::AccelerationBands::new(timeperiod).map_err(py_value_error)?,
            uppers: Vec::new(),
            middles: Vec::new(),
            lowers: Vec::new(),
        })
    }

    fn append(&mut self, high: f64, low: f64, close: f64) -> Option<(f64, f64, f64)> {
        let value = self
            .inner
            .append(high, low, close)
            .map(|value| (value.upper, value.middle, value.lower));
        let (upper, middle, lower) = value.unwrap_or((f64::NAN, f64::NAN, f64::NAN));
        self.uppers.push(upper);
        self.middles.push(middle);
        self.lowers.push(lower);
        value
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let high = high.as_slice()?;
        let low = low.as_slice()?;
        let close = close.as_slice()?;
        if high.len() != low.len() || high.len() != close.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        let (uppers, middles, lowers) = (&mut self.uppers, &mut self.middles, &mut self.lowers);
        py.allow_threads(|| {
            self.inner
                .extend_slices_into(high, low, close, uppers, middles, lowers)
                .expect("lengths validated above")
        });
        Ok(())
    }

    fn compute(&self, py: Python<'_>) -> (Py<PyArray1<f64>>, Py<PyArray1<f64>>, Py<PyArray1<f64>>) {
        (
            to_py_array(py, self.uppers.clone()),
            to_py_array(py, self.middles.clone()),
            to_py_array(py, self.lowers.clone()),
        )
    }

    fn __len__(&self) -> usize {
        self.uppers.len()
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64, f64)> {
        self.inner
            .value()
            .map(|value| (value.upper, value.middle, value.lower))
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.uppers.clear();
        self.middles.clear();
        self.lowers.clear();
    }
}

#[pyclass]
pub struct StatefulSar {
    inner: stream::ParabolicSar,
    outputs: Vec<f64>,
}

#[pymethods]
impl StatefulSar {
    #[new]
    #[pyo3(signature = (acceleration=0.02, maximum=0.2))]
    fn new(acceleration: f64, maximum: f64) -> Self {
        Self {
            inner: stream::ParabolicSar::new(acceleration, maximum),
            outputs: Vec::new(),
        }
    }

    fn append(&mut self, high: f64, low: f64) -> Option<f64> {
        push_option(&mut self.outputs, self.inner.append(high, low))
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let high = high.as_slice()?;
        let low = low.as_slice()?;
        if high.len() != low.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        let outputs = &mut self.outputs;
        py.allow_threads(|| {
            extend_from_options(
                outputs,
                high.iter()
                    .zip(low)
                    .map(|(&high, &low)| self.inner.append(high, low)),
            )
        });
        Ok(())
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn compute(&self, py: Python<'_>) -> Py<PyArray1<f64>> {
        to_py_array(py, self.outputs.clone())
    }

    fn __len__(&self) -> usize {
        self.outputs.len()
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.outputs.clear();
    }
}

#[pyclass]
pub struct StatefulSarext {
    inner: stream::ParabolicSarExtended,
    outputs: Vec<f64>,
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
            inner: stream::ParabolicSarExtended::new(
                startvalue,
                offsetonreverse,
                accelerationinitlong,
                accelerationlong,
                accelerationmaxlong,
                accelerationinitshort,
                accelerationshort,
                accelerationmaxshort,
            ),
            outputs: Vec::new(),
        }
    }

    fn append(&mut self, high: f64, low: f64) -> Option<f64> {
        push_option(&mut self.outputs, self.inner.append(high, low))
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let high = high.as_slice()?;
        let low = low.as_slice()?;
        if high.len() != low.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        let outputs = &mut self.outputs;
        py.allow_threads(|| {
            extend_from_options(
                outputs,
                high.iter()
                    .zip(low)
                    .map(|(&high, &low)| self.inner.append(high, low)),
            )
        });
        Ok(())
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn compute(&self, py: Python<'_>) -> Py<PyArray1<f64>> {
        to_py_array(py, self.outputs.clone())
    }

    fn __len__(&self) -> usize {
        self.outputs.len()
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.outputs.clear();
    }
}

macro_rules! deviation_state_class {
    ($class:ident, $inner:ident) => {
        #[pyclass]
        pub struct $class {
            inner: stream::$inner,
            outputs: Vec<f64>,
        }

        #[pymethods]
        impl $class {
            #[new]
            #[pyo3(signature = (timeperiod=5, nbdev=1.0))]
            fn new(timeperiod: usize, nbdev: f64) -> PyResult<Self> {
                Ok(Self {
                    inner: stream::$inner::new(timeperiod, nbdev).map_err(py_value_error)?,
                    outputs: Vec::new(),
                })
            }

            fn append(&mut self, input: f64) -> Option<f64> {
                push_option(&mut self.outputs, self.inner.append(input))
            }

            fn extend(&mut self, py: Python<'_>, input: PyReadonlyArray1<f64>) -> PyResult<()> {
                let input = input.as_slice()?;
                let outputs = &mut self.outputs;
                py.allow_threads(|| self.inner.extend_slice_into(input, outputs));
                Ok(())
            }

            #[getter]
            fn value(&self) -> Option<f64> {
                self.inner.value()
            }

            fn compute(&self, py: Python<'_>) -> Py<PyArray1<f64>> {
                to_py_array(py, self.outputs.clone())
            }

            fn __len__(&self) -> usize {
                self.outputs.len()
            }

            fn reset(&mut self) {
                self.inner.reset();
                self.outputs.clear();
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
            outputs: Vec<f64>,
        }

        #[pymethods]
        impl $class {
            #[new]
            #[pyo3(signature = (timeperiod=5))]
            fn new(timeperiod: usize) -> PyResult<Self> {
                Ok(Self {
                    inner: stream::$inner::new(timeperiod).map_err(py_value_error)?,
                    outputs: Vec::new(),
                })
            }

            fn append(&mut self, input0: f64, input1: f64) -> Option<f64> {
                push_option(&mut self.outputs, self.inner.append(input0, input1))
            }

            fn extend(
                &mut self,
                py: Python<'_>,
                input0: PyReadonlyArray1<f64>,
                input1: PyReadonlyArray1<f64>,
            ) -> PyResult<()> {
                let input0 = input0.as_slice()?;
                let input1 = input1.as_slice()?;
                if input0.len() != input1.len() {
                    return Err(PyValueError::new_err("inputs must have equal lengths"));
                }
                let outputs = &mut self.outputs;
                let inner = &mut self.inner;
                py.allow_threads(|| inner.extend_slices_into(input0, input1, outputs))
                    .map_err(py_value_error)
            }

            #[getter]
            fn value(&self) -> Option<f64> {
                self.inner.value()
            }

            fn compute(&self, py: Python<'_>) -> Py<PyArray1<f64>> {
                to_py_array(py, self.outputs.clone())
            }

            fn __len__(&self) -> usize {
                self.outputs.len()
            }

            fn reset(&mut self) {
                self.inner.reset();
                self.outputs.clear();
            }
        }
    };
}

bivariate_statistic_class!(StatefulBeta, RollingBeta);
bivariate_statistic_class!(StatefulCorrel, RollingCorrelation);

#[pyclass]
pub struct AccumulationDistribution {
    inner: stream::AccumulationDistribution,
    outputs: Vec<f64>,
}

#[pymethods]
impl AccumulationDistribution {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            inner: stream::AccumulationDistribution::new().map_err(py_value_error)?,
            outputs: Vec::new(),
        })
    }

    fn append(&mut self, high: f64, low: f64, close: f64, volume: f64) -> f64 {
        let value = self.inner.append(high, low, close, volume);
        self.outputs.push(value);
        value
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
        volume: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let high = high.as_slice()?;
        let low = low.as_slice()?;
        let close = close.as_slice()?;
        let volume = volume.as_slice()?;
        py.allow_threads(|| {
            self.inner
                .extend_slices_into(high, low, close, volume, &mut self.outputs)
        })
        .map_err(py_value_error)
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn compute(&self, py: Python<'_>) -> Py<PyArray1<f64>> {
        to_py_array(py, self.outputs.clone())
    }

    fn __len__(&self) -> usize {
        self.outputs.len()
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.outputs.clear();
    }
}

#[pyclass]
pub struct AccumulationDistributionOscillator {
    inner: stream::AccumulationDistributionOscillator,
    outputs: Vec<f64>,
}

#[pymethods]
impl AccumulationDistributionOscillator {
    #[new]
    #[pyo3(signature = (fastperiod=3, slowperiod=10))]
    fn new(fastperiod: usize, slowperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: stream::AccumulationDistributionOscillator::new(fastperiod, slowperiod)
                .map_err(py_value_error)?,
            outputs: Vec::new(),
        })
    }

    fn append(&mut self, high: f64, low: f64, close: f64, volume: f64) -> Option<f64> {
        push_option(
            &mut self.outputs,
            self.inner.append(high, low, close, volume),
        )
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
        volume: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let high = high.as_slice()?;
        let low = low.as_slice()?;
        let close = close.as_slice()?;
        let volume = volume.as_slice()?;
        py.allow_threads(|| {
            self.inner
                .extend_slices_into(high, low, close, volume, &mut self.outputs)
        })
        .map_err(py_value_error)
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn compute(&self, py: Python<'_>) -> Py<PyArray1<f64>> {
        to_py_array(py, self.outputs.clone())
    }

    fn __len__(&self) -> usize {
        self.outputs.len()
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.outputs.clear();
    }
}

#[pyclass]
pub struct OnBalanceVolume {
    inner: stream::OnBalanceVolume,
    outputs: Vec<f64>,
}

#[pymethods]
impl OnBalanceVolume {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            inner: stream::OnBalanceVolume::new().map_err(py_value_error)?,
            outputs: Vec::new(),
        })
    }

    fn append(&mut self, close: f64, volume: f64) -> f64 {
        let value = self.inner.append(close, volume);
        self.outputs.push(value);
        value
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        close: PyReadonlyArray1<f64>,
        volume: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let close = close.as_slice()?;
        let volume = volume.as_slice()?;
        py.allow_threads(|| {
            self.inner
                .extend_slices_into(close, volume, &mut self.outputs)
        })
        .map_err(py_value_error)
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn compute(&self, py: Python<'_>) -> Py<PyArray1<f64>> {
        to_py_array(py, self.outputs.clone())
    }

    fn __len__(&self) -> usize {
        self.outputs.len()
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.outputs.clear();
    }
}

#[pyclass]
pub struct BalanceOfPower {
    inner: stream::BalanceOfPower,
    outputs: Vec<f64>,
}

#[pymethods]
impl BalanceOfPower {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            inner: stream::BalanceOfPower::new().map_err(py_value_error)?,
            outputs: Vec::new(),
        })
    }

    fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> f64 {
        let value = self.inner.append(open, high, low, close);
        self.outputs.push(value);
        value
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        open: PyReadonlyArray1<f64>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let open = open.as_slice()?;
        let high = high.as_slice()?;
        let low = low.as_slice()?;
        let close = close.as_slice()?;
        py.allow_threads(|| {
            self.inner
                .extend_slices_into(open, high, low, close, &mut self.outputs)
        })
        .map_err(py_value_error)
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn compute(&self, py: Python<'_>) -> Py<PyArray1<f64>> {
        to_py_array(py, self.outputs.clone())
    }

    fn __len__(&self) -> usize {
        self.outputs.len()
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.outputs.clear();
    }
}

#[pyclass]
pub struct WilliamsPercentR {
    inner: stream::WilliamsPercentR,
    outputs: Vec<f64>,
}

#[pymethods]
impl WilliamsPercentR {
    #[new]
    #[pyo3(signature = (timeperiod=14))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: stream::WilliamsPercentR::new(timeperiod).map_err(py_value_error)?,
            outputs: Vec::new(),
        })
    }

    fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        push_option(&mut self.outputs, self.inner.append(high, low, close))
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let high = high.as_slice()?;
        let low = low.as_slice()?;
        let close = close.as_slice()?;
        py.allow_threads(|| {
            self.inner
                .extend_slices_into(high, low, close, &mut self.outputs)
        })
        .map_err(py_value_error)
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn compute(&self, py: Python<'_>) -> Py<PyArray1<f64>> {
        to_py_array(py, self.outputs.clone())
    }

    fn __len__(&self) -> usize {
        self.outputs.len()
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.outputs.clear();
    }
}

#[pyclass]
pub struct Aroon {
    inner: stream::Aroon,
    downs: Vec<f64>,
    ups: Vec<f64>,
}

#[pymethods]
impl Aroon {
    #[new]
    #[pyo3(signature = (timeperiod=14))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: stream::Aroon::new(timeperiod).map_err(py_value_error)?,
            downs: Vec::new(),
            ups: Vec::new(),
        })
    }

    fn append(&mut self, high: f64, low: f64) -> Option<(f64, f64)> {
        let value = self
            .inner
            .append(high, low)
            .map(|value| (value.down, value.up));
        let (down, up) = value.unwrap_or((f64::NAN, f64::NAN));
        self.downs.push(down);
        self.ups.push(up);
        value
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let high = high.as_slice()?;
        let low = low.as_slice()?;
        let (downs, ups) = (&mut self.downs, &mut self.ups);
        py.allow_threads(|| self.inner.extend_slices_into(high, low, downs, ups))
            .map_err(py_value_error)
    }

    fn compute(&self, py: Python<'_>) -> (Py<PyArray1<f64>>, Py<PyArray1<f64>>) {
        (
            to_py_array(py, self.downs.clone()),
            to_py_array(py, self.ups.clone()),
        )
    }

    fn __len__(&self) -> usize {
        self.downs.len()
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64)> {
        self.inner.value().map(|value| (value.down, value.up))
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.downs.clear();
        self.ups.clear();
    }
}

#[pyclass]
pub struct AroonOscillator {
    inner: stream::AroonOscillator,
    outputs: Vec<f64>,
}

#[pymethods]
impl AroonOscillator {
    #[new]
    #[pyo3(signature = (timeperiod=14))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: stream::AroonOscillator::new(timeperiod).map_err(py_value_error)?,
            outputs: Vec::new(),
        })
    }

    fn append(&mut self, high: f64, low: f64) -> Option<f64> {
        push_option(&mut self.outputs, self.inner.append(high, low))
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let high = high.as_slice()?;
        let low = low.as_slice()?;
        py.allow_threads(|| self.inner.extend_slices_into(high, low, &mut self.outputs))
            .map_err(py_value_error)
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn compute(&self, py: Python<'_>) -> Py<PyArray1<f64>> {
        to_py_array(py, self.outputs.clone())
    }

    fn __len__(&self) -> usize {
        self.outputs.len()
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.outputs.clear();
    }
}

#[pyclass]
pub struct StatefulMinmax {
    inner: stream::RollingMinmax,
    minimums: Vec<f64>,
    maximums: Vec<f64>,
}

#[pymethods]
impl StatefulMinmax {
    #[new]
    #[pyo3(signature = (timeperiod=30))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: stream::RollingMinmax::new(timeperiod).map_err(py_value_error)?,
            minimums: Vec::new(),
            maximums: Vec::new(),
        })
    }

    fn append(&mut self, input: f64) -> Option<(f64, f64)> {
        let value = self
            .inner
            .append(input)
            .map(|value| (value.minimum, value.maximum));
        let (minimum, maximum) = value.unwrap_or((f64::NAN, f64::NAN));
        self.minimums.push(minimum);
        self.maximums.push(maximum);
        value
    }

    fn extend(&mut self, py: Python<'_>, input: PyReadonlyArray1<f64>) -> PyResult<()> {
        let input = input.as_slice()?;
        let (minimums, maximums) = (&mut self.minimums, &mut self.maximums);
        py.allow_threads(|| self.inner.extend_slices_into(input, minimums, maximums));
        Ok(())
    }

    fn compute(&self, py: Python<'_>) -> (Py<PyArray1<f64>>, Py<PyArray1<f64>>) {
        (
            to_py_array(py, self.minimums.clone()),
            to_py_array(py, self.maximums.clone()),
        )
    }

    fn __len__(&self) -> usize {
        self.minimums.len()
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64)> {
        self.inner
            .value()
            .map(|value| (value.minimum, value.maximum))
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.minimums.clear();
        self.maximums.clear();
    }
}

#[pyclass]
pub struct StatefulMinmaxindex {
    inner: stream::RollingMinmaxIndex,
    minimums: Vec<f64>,
    maximums: Vec<f64>,
}

#[pymethods]
impl StatefulMinmaxindex {
    #[new]
    #[pyo3(signature = (timeperiod=30))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: stream::RollingMinmaxIndex::new(timeperiod).map_err(py_value_error)?,
            minimums: Vec::new(),
            maximums: Vec::new(),
        })
    }

    fn append(&mut self, input: f64) -> (usize, usize) {
        let value = self.inner.append(input);
        self.minimums.push(value.minimum as f64);
        self.maximums.push(value.maximum as f64);
        (value.minimum, value.maximum)
    }

    fn extend(&mut self, py: Python<'_>, input: PyReadonlyArray1<f64>) -> PyResult<()> {
        let input = input.as_slice()?;
        let (minimums, maximums) = (&mut self.minimums, &mut self.maximums);
        py.allow_threads(|| self.inner.extend_slices_into(input, minimums, maximums));
        Ok(())
    }

    fn compute(&self, py: Python<'_>) -> (Py<PyArray1<f64>>, Py<PyArray1<f64>>) {
        (
            to_py_array(py, self.minimums.clone()),
            to_py_array(py, self.maximums.clone()),
        )
    }

    fn __len__(&self) -> usize {
        self.minimums.len()
    }

    #[getter]
    fn value(&self) -> Option<(usize, usize)> {
        self.inner
            .value()
            .map(|value| (value.minimum, value.maximum))
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.minimums.clear();
        self.maximums.clear();
    }
}

macro_rules! unary_state_class {
    ($class:ident) => {
        #[pyclass]
        pub struct $class {
            inner: stream::$class,
            outputs: Vec<f64>,
        }

        #[pymethods]
        impl $class {
            #[new]
            fn new() -> PyResult<Self> {
                Ok(Self {
                    inner: stream::$class::new().map_err(py_value_error)?,
                    outputs: Vec::new(),
                })
            }

            fn append(&mut self, input: f64) -> f64 {
                let value = self
                    .inner
                    .append(input)
                    .expect("stateless transform is warm");
                self.outputs.push(value);
                value
            }

            fn extend(&mut self, py: Python<'_>, input: PyReadonlyArray1<f64>) -> PyResult<()> {
                let input = input.as_slice()?;
                let outputs = &mut self.outputs;
                py.allow_threads(|| self.inner.extend_slice_into(input, outputs));
                Ok(())
            }

            #[getter]
            fn value(&self) -> Option<f64> {
                self.inner.value()
            }

            fn compute(&self, py: Python<'_>) -> Py<PyArray1<f64>> {
                to_py_array(py, self.outputs.clone())
            }

            fn __len__(&self) -> usize {
                self.outputs.len()
            }

            fn reset(&mut self) {
                self.inner.reset();
                self.outputs.clear();
            }
        }
    };
}

unary_state_class!(MathAbs);
unary_state_class!(MathAcos);
unary_state_class!(MathAcosh);
unary_state_class!(MathAsin);
unary_state_class!(MathAsinh);
unary_state_class!(MathAtan);
unary_state_class!(MathAtanh);
unary_state_class!(MathCbrt);
unary_state_class!(MathCeil);
unary_state_class!(MathCos);
unary_state_class!(MathCosh);
unary_state_class!(MathCot);
unary_state_class!(MathDegrees);
unary_state_class!(MathExp);
unary_state_class!(MathFloor);
unary_state_class!(MathLn);
unary_state_class!(MathLog10);
unary_state_class!(MathLog1p);
unary_state_class!(MathRadians);
unary_state_class!(MathSin);
unary_state_class!(MathSinh);
unary_state_class!(MathSqrt);
unary_state_class!(MathTan);
unary_state_class!(MathTanh);

macro_rules! binary_math_state_class {
    ($class:ident) => {
        #[pyclass]
        pub struct $class {
            inner: stream::$class,
            outputs: Vec<f64>,
        }

        #[pymethods]
        impl $class {
            #[new]
            fn new() -> PyResult<Self> {
                Ok(Self {
                    inner: stream::$class::new().map_err(py_value_error)?,
                    outputs: Vec::new(),
                })
            }

            fn append(&mut self, left: f64, right: f64) -> f64 {
                let value = self.inner.append(left, right);
                self.outputs.push(value);
                value
            }

            fn extend(
                &mut self,
                py: Python<'_>,
                left: PyReadonlyArray1<f64>,
                right: PyReadonlyArray1<f64>,
            ) -> PyResult<()> {
                let left = left.as_slice()?;
                let right = right.as_slice()?;
                let inner = &mut self.inner;
                let outputs = &mut self.outputs;
                py.allow_threads(|| inner.extend_slices_into(left, right, outputs))
                    .map_err(py_value_error)
            }

            #[getter]
            fn value(&self) -> Option<f64> {
                self.inner.value()
            }

            fn compute(&self, py: Python<'_>) -> Py<PyArray1<f64>> {
                to_py_array(py, self.outputs.clone())
            }

            fn __len__(&self) -> usize {
                self.outputs.len()
            }

            fn reset(&mut self) {
                self.inner.reset();
                self.outputs.clear();
            }
        }
    };
}

binary_math_state_class!(MathAdd);
binary_math_state_class!(MathSubtract);
binary_math_state_class!(MathMultiply);
binary_math_state_class!(MathDivide);

macro_rules! binary_state_class {
    ($class:ident) => {
        #[pyclass]
        pub struct $class {
            inner: stream::$class,
            outputs: Vec<f64>,
        }

        #[pymethods]
        impl $class {
            #[new]
            fn new() -> PyResult<Self> {
                Ok(Self {
                    inner: stream::$class::new().map_err(py_value_error)?,
                    outputs: Vec::new(),
                })
            }

            fn append(&mut self, left: f64, right: f64) -> f64 {
                let value = self.inner.append(left, right);
                self.outputs.push(value);
                value
            }

            fn extend(
                &mut self,
                py: Python<'_>,
                left: PyReadonlyArray1<f64>,
                right: PyReadonlyArray1<f64>,
            ) -> PyResult<()> {
                let left = left.as_slice()?;
                let right = right.as_slice()?;
                py.allow_threads(|| {
                    self.inner
                        .extend_slices_into(left, right, &mut self.outputs)
                })
                .map_err(py_value_error)
            }

            #[getter]
            fn value(&self) -> Option<f64> {
                self.inner.value()
            }

            fn compute(&self, py: Python<'_>) -> Py<PyArray1<f64>> {
                to_py_array(py, self.outputs.clone())
            }

            fn __len__(&self) -> usize {
                self.outputs.len()
            }

            fn reset(&mut self) {
                self.inner.reset();
                self.outputs.clear();
            }
        }
    };
}

binary_state_class!(MedianPrice);

macro_rules! price3_state_class {
    ($class:ident) => {
        #[pyclass]
        pub struct $class {
            inner: stream::$class,
            outputs: Vec<f64>,
        }

        #[pymethods]
        impl $class {
            #[new]
            fn new() -> PyResult<Self> {
                Ok(Self {
                    inner: stream::$class::new().map_err(py_value_error)?,
                    outputs: Vec::new(),
                })
            }

            fn append(&mut self, high: f64, low: f64, close: f64) -> f64 {
                let value = self.inner.append(high, low, close);
                self.outputs.push(value);
                value
            }

            fn extend(
                &mut self,
                py: Python<'_>,
                high: PyReadonlyArray1<f64>,
                low: PyReadonlyArray1<f64>,
                close: PyReadonlyArray1<f64>,
            ) -> PyResult<()> {
                let high = high.as_slice()?;
                let low = low.as_slice()?;
                let close = close.as_slice()?;
                py.allow_threads(|| {
                    self.inner
                        .extend_slices_into(high, low, close, &mut self.outputs)
                })
                .map_err(py_value_error)
            }

            #[getter]
            fn value(&self) -> Option<f64> {
                self.inner.value()
            }

            fn compute(&self, py: Python<'_>) -> Py<PyArray1<f64>> {
                to_py_array(py, self.outputs.clone())
            }

            fn __len__(&self) -> usize {
                self.outputs.len()
            }

            fn reset(&mut self) {
                self.inner.reset();
                self.outputs.clear();
            }
        }
    };
}

price3_state_class!(TypicalPrice);
price3_state_class!(WeightedClose);

#[pyclass]
pub struct AveragePrice {
    inner: stream::AveragePrice,
    outputs: Vec<f64>,
}

#[pymethods]
impl AveragePrice {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            inner: stream::AveragePrice::new().map_err(py_value_error)?,
            outputs: Vec::new(),
        })
    }

    fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> f64 {
        let value = self.inner.append(open, high, low, close);
        self.outputs.push(value);
        value
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        open: PyReadonlyArray1<f64>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let open = open.as_slice()?;
        let high = high.as_slice()?;
        let low = low.as_slice()?;
        let close = close.as_slice()?;
        py.allow_threads(|| {
            self.inner
                .extend_slices_into(open, high, low, close, &mut self.outputs)
        })
        .map_err(py_value_error)
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn compute(&self, py: Python<'_>) -> Py<PyArray1<f64>> {
        to_py_array(py, self.outputs.clone())
    }

    fn __len__(&self) -> usize {
        self.outputs.len()
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.outputs.clear();
    }
}

#[pyclass]
pub struct StatefulMidprice {
    inner: RollingMidprice,
    outputs: Vec<f64>,
}

#[pymethods]
impl StatefulMidprice {
    #[new]
    #[pyo3(signature = (timeperiod=14))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: RollingMidprice::new(timeperiod).map_err(py_value_error)?,
            outputs: Vec::new(),
        })
    }

    fn append(&mut self, high: f64, low: f64) -> Option<f64> {
        push_option(&mut self.outputs, self.inner.append(high, low))
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let high = high.as_slice()?;
        let low = low.as_slice()?;
        if high.len() != low.len() {
            return Err(PyValueError::new_err(
                "high and low must have equal lengths",
            ));
        }
        let outputs = &mut self.outputs;
        py.allow_threads(|| {
            self.inner
                .extend_slices_into(high, low, outputs)
                .expect("lengths validated above")
        });
        Ok(())
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn compute(&self, py: Python<'_>) -> Py<PyArray1<f64>> {
        to_py_array(py, self.outputs.clone())
    }

    fn __len__(&self) -> usize {
        self.outputs.len()
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.outputs.clear();
    }
}

#[pyclass]
pub struct StatefulSma {
    inner: SimpleMovingAverage,
    outputs: Vec<f64>,
}

#[pymethods]
impl StatefulSma {
    #[new]
    #[pyo3(signature = (timeperiod=30))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: SimpleMovingAverage::new(timeperiod).map_err(py_value_error)?,
            outputs: Vec::new(),
        })
    }

    fn append(&mut self, input: f64) -> Option<f64> {
        push_option(&mut self.outputs, self.inner.append(input))
    }

    fn extend(&mut self, py: Python<'_>, input: PyReadonlyArray1<f64>) -> PyResult<()> {
        let input = input.as_slice()?;
        let outputs = &mut self.outputs;
        py.allow_threads(|| self.inner.extend_slice_into(input, outputs));
        Ok(())
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn compute(&self, py: Python<'_>) -> Py<PyArray1<f64>> {
        to_py_array(py, self.outputs.clone())
    }

    fn __len__(&self) -> usize {
        self.outputs.len()
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.outputs.clear();
    }
}

#[pyclass]
pub struct StatefulEma {
    inner: ExponentialMovingAverage,
    outputs: Vec<f64>,
}

#[pymethods]
impl StatefulEma {
    #[new]
    #[pyo3(signature = (timeperiod=30))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: ExponentialMovingAverage::new(timeperiod).map_err(py_value_error)?,
            outputs: Vec::new(),
        })
    }

    fn append(&mut self, input: f64) -> Option<f64> {
        push_option(&mut self.outputs, self.inner.append(input))
    }

    fn extend(&mut self, py: Python<'_>, input: PyReadonlyArray1<f64>) -> PyResult<()> {
        let input = input.as_slice()?;
        let outputs = &mut self.outputs;
        py.allow_threads(|| self.inner.extend_slice_into(input, outputs));
        Ok(())
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn compute(&self, py: Python<'_>) -> Py<PyArray1<f64>> {
        to_py_array(py, self.outputs.clone())
    }

    fn __len__(&self) -> usize {
        self.outputs.len()
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.outputs.clear();
    }
}

#[pyclass]
pub struct StatefulWma {
    inner: WeightedMovingAverage,
    outputs: Vec<f64>,
}

#[pymethods]
impl StatefulWma {
    #[new]
    #[pyo3(signature = (timeperiod=30))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: WeightedMovingAverage::new(timeperiod).map_err(py_value_error)?,
            outputs: Vec::new(),
        })
    }

    fn append(&mut self, input: f64) -> Option<f64> {
        push_option(&mut self.outputs, self.inner.append(input))
    }

    fn extend(&mut self, py: Python<'_>, input: PyReadonlyArray1<f64>) -> PyResult<()> {
        let input = input.as_slice()?;
        let outputs = &mut self.outputs;
        py.allow_threads(|| self.inner.extend_slice_into(input, outputs));
        Ok(())
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn compute(&self, py: Python<'_>) -> Py<PyArray1<f64>> {
        to_py_array(py, self.outputs.clone())
    }

    fn __len__(&self) -> usize {
        self.outputs.len()
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.outputs.clear();
    }
}

#[pyclass]
pub struct StatefulDema {
    inner: DoubleExponentialMovingAverage,
    outputs: Vec<f64>,
}

#[pymethods]
impl StatefulDema {
    #[new]
    #[pyo3(signature = (timeperiod=30))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: DoubleExponentialMovingAverage::new(timeperiod).map_err(py_value_error)?,
            outputs: Vec::new(),
        })
    }

    fn append(&mut self, input: f64) -> Option<f64> {
        push_option(&mut self.outputs, self.inner.append(input))
    }

    fn extend(&mut self, py: Python<'_>, input: PyReadonlyArray1<f64>) -> PyResult<()> {
        let input = input.as_slice()?;
        let outputs = &mut self.outputs;
        py.allow_threads(|| self.inner.extend_slice_into(input, outputs));
        Ok(())
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn compute(&self, py: Python<'_>) -> Py<PyArray1<f64>> {
        to_py_array(py, self.outputs.clone())
    }

    fn __len__(&self) -> usize {
        self.outputs.len()
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.outputs.clear();
    }
}

#[pyclass]
pub struct StatefulTema {
    inner: TripleExponentialMovingAverage,
    outputs: Vec<f64>,
}

#[pymethods]
impl StatefulTema {
    #[new]
    #[pyo3(signature = (timeperiod=30))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: TripleExponentialMovingAverage::new(timeperiod).map_err(py_value_error)?,
            outputs: Vec::new(),
        })
    }

    fn append(&mut self, input: f64) -> Option<f64> {
        push_option(&mut self.outputs, self.inner.append(input))
    }

    fn extend(&mut self, py: Python<'_>, input: PyReadonlyArray1<f64>) -> PyResult<()> {
        let input = input.as_slice()?;
        let outputs = &mut self.outputs;
        py.allow_threads(|| self.inner.extend_slice_into(input, outputs));
        Ok(())
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn compute(&self, py: Python<'_>) -> Py<PyArray1<f64>> {
        to_py_array(py, self.outputs.clone())
    }

    fn __len__(&self) -> usize {
        self.outputs.len()
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.outputs.clear();
    }
}

#[pyclass]
pub struct StatefulTrima {
    inner: TriangularMovingAverage,
    outputs: Vec<f64>,
}

#[pymethods]
impl StatefulTrima {
    #[new]
    #[pyo3(signature = (timeperiod=30))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: TriangularMovingAverage::new(timeperiod).map_err(py_value_error)?,
            outputs: Vec::new(),
        })
    }

    fn append(&mut self, input: f64) -> Option<f64> {
        push_option(&mut self.outputs, self.inner.append(input))
    }

    fn extend(&mut self, py: Python<'_>, input: PyReadonlyArray1<f64>) -> PyResult<()> {
        let input = input.as_slice()?;
        let outputs = &mut self.outputs;
        py.allow_threads(|| self.inner.extend_slice_into(input, outputs));
        Ok(())
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn compute(&self, py: Python<'_>) -> Py<PyArray1<f64>> {
        to_py_array(py, self.outputs.clone())
    }

    fn __len__(&self) -> usize {
        self.outputs.len()
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.outputs.clear();
    }
}

#[pyclass]
pub struct AverageTrueRange {
    inner: CoreAverageTrueRange,
    outputs: Vec<f64>,
}

#[pymethods]
impl AverageTrueRange {
    #[new]
    #[pyo3(signature = (timeperiod=14))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: CoreAverageTrueRange::new(timeperiod).map_err(py_value_error)?,
            outputs: Vec::new(),
        })
    }

    fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        push_option(&mut self.outputs, self.inner.append(high, low, close))
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let high = high.as_slice()?;
        let low = low.as_slice()?;
        let close = close.as_slice()?;
        py.allow_threads(|| {
            self.inner
                .extend_slices_into(high, low, close, &mut self.outputs)
        })
        .map_err(py_value_error)
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn compute(&self, py: Python<'_>) -> Py<PyArray1<f64>> {
        to_py_array(py, self.outputs.clone())
    }

    fn __len__(&self) -> usize {
        self.outputs.len()
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.outputs.clear();
    }
}

#[pyclass]
pub struct TrueRange {
    inner: CoreTrueRange,
    outputs: Vec<f64>,
}

#[pymethods]
impl TrueRange {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            inner: CoreTrueRange::new().map_err(py_value_error)?,
            outputs: Vec::new(),
        })
    }

    fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        push_option(&mut self.outputs, self.inner.append(high, low, close))
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let high = high.as_slice()?;
        let low = low.as_slice()?;
        let close = close.as_slice()?;
        py.allow_threads(|| {
            self.inner
                .extend_slices_into(high, low, close, &mut self.outputs)
        })
        .map_err(py_value_error)
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }
    fn compute(&self, py: Python<'_>) -> Py<PyArray1<f64>> {
        to_py_array(py, self.outputs.clone())
    }

    fn __len__(&self) -> usize {
        self.outputs.len()
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.outputs.clear();
    }
}

#[pyclass]
pub struct NormalizedAverageTrueRange {
    inner: CoreNormalizedAverageTrueRange,
    outputs: Vec<f64>,
}

#[pymethods]
impl NormalizedAverageTrueRange {
    #[new]
    #[pyo3(signature = (timeperiod=14))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: CoreNormalizedAverageTrueRange::new(timeperiod).map_err(py_value_error)?,
            outputs: Vec::new(),
        })
    }

    fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        push_option(&mut self.outputs, self.inner.append(high, low, close))
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let high = high.as_slice()?;
        let low = low.as_slice()?;
        let close = close.as_slice()?;
        py.allow_threads(|| {
            self.inner
                .extend_slices_into(high, low, close, &mut self.outputs)
        })
        .map_err(py_value_error)
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }
    fn compute(&self, py: Python<'_>) -> Py<PyArray1<f64>> {
        to_py_array(py, self.outputs.clone())
    }

    fn __len__(&self) -> usize {
        self.outputs.len()
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.outputs.clear();
    }
}

#[pyclass]
pub struct StatefulMacd {
    inner: MovingAverageConvergenceDivergence,
    macds: Vec<f64>,
    signals: Vec<f64>,
    histograms: Vec<f64>,
}

#[pyclass]
pub struct StatefulMacdFix {
    inner: MovingAverageConvergenceDivergenceFixed,
    macds: Vec<f64>,
    signals: Vec<f64>,
    histograms: Vec<f64>,
}

#[pyclass]
pub struct StatefulMacdExt {
    inner: MovingAverageConvergenceDivergenceExtended,
    macds: Vec<f64>,
    signals: Vec<f64>,
    histograms: Vec<f64>,
}

#[pyclass]
pub struct VariablePeriodMovingAverage {
    inner: CoreVariablePeriodMovingAverage,
    outputs: Vec<f64>,
}

#[pyclass]
pub struct StatefulHtTrendline {
    inner: HilbertTransformTrendline,
    outputs: Vec<f64>,
}

#[pyclass]
pub struct StatefulAdx {
    inner: AverageDirectionalIndex,
    outputs: Vec<f64>,
}

#[pyclass]
pub struct StatefulAdxr {
    inner: AverageDirectionalIndexRating,
    outputs: Vec<f64>,
}

#[pyclass]
pub struct StatefulDx {
    inner: DirectionalMovementIndex,
    outputs: Vec<f64>,
}

#[pyclass]
pub struct StatefulStochf {
    inner: FastStochasticOscillator,
    fastks: Vec<f64>,
    fastds: Vec<f64>,
}

#[pyclass]
pub struct StatefulStoch {
    inner: StochasticOscillator,
    slowks: Vec<f64>,
    slowds: Vec<f64>,
}

#[pyclass]
pub struct StatefulStochrsi {
    inner: StochasticRelativeStrengthIndex,
    fastks: Vec<f64>,
    fastds: Vec<f64>,
}

#[pyclass]
pub struct StatefulMama {
    inner: MesaAdaptiveMovingAverage,
    mamas: Vec<f64>,
    famas: Vec<f64>,
}

#[pymethods]
impl StatefulMama {
    #[new]
    #[pyo3(signature = (fastlimit=0.5, slowlimit=0.05))]
    fn new(fastlimit: f64, slowlimit: f64) -> PyResult<Self> {
        Ok(Self {
            inner: MesaAdaptiveMovingAverage::new(fastlimit, slowlimit).map_err(py_value_error)?,
            mamas: Vec::new(),
            famas: Vec::new(),
        })
    }

    fn append(&mut self, input: f64) -> Option<(f64, f64)> {
        let value = self
            .inner
            .append(input)
            .map(|value| (value.mama, value.fama));
        let (mama, fama) = value.unwrap_or((f64::NAN, f64::NAN));
        self.mamas.push(mama);
        self.famas.push(fama);
        value
    }

    fn extend(&mut self, py: Python<'_>, input: PyReadonlyArray1<f64>) -> PyResult<()> {
        let input = input.as_slice()?;
        let (mamas, famas) = (&mut self.mamas, &mut self.famas);
        py.allow_threads(|| {
            mamas.reserve(input.len());
            famas.reserve(input.len());
            for input in input.iter().copied() {
                if let Some(value) = self.inner.append(input) {
                    mamas.push(value.mama);
                    famas.push(value.fama);
                } else {
                    mamas.push(f64::NAN);
                    famas.push(f64::NAN);
                }
            }
        });
        Ok(())
    }

    fn compute(&self, py: Python<'_>) -> (Py<PyArray1<f64>>, Py<PyArray1<f64>>) {
        (
            to_py_array(py, self.mamas.clone()),
            to_py_array(py, self.famas.clone()),
        )
    }

    fn __len__(&self) -> usize {
        self.mamas.len()
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64)> {
        self.inner.value().map(|value| (value.mama, value.fama))
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.mamas.clear();
        self.famas.clear();
    }
}

#[pymethods]
impl StatefulMacd {
    #[new]
    #[pyo3(signature = (fastperiod=12, slowperiod=26, signalperiod=9))]
    fn new(fastperiod: usize, slowperiod: usize, signalperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: MovingAverageConvergenceDivergence::new(fastperiod, slowperiod, signalperiod)
                .map_err(py_value_error)?,
            macds: Vec::new(),
            signals: Vec::new(),
            histograms: Vec::new(),
        })
    }

    fn append(&mut self, input: f64) -> Option<(f64, f64, f64)> {
        let value = self
            .inner
            .append(input)
            .map(|value| (value.macd, value.signal, value.histogram));
        let (macd, signal, histogram) = value.unwrap_or((f64::NAN, f64::NAN, f64::NAN));
        self.macds.push(macd);
        self.signals.push(signal);
        self.histograms.push(histogram);
        value
    }

    fn extend(&mut self, py: Python<'_>, input: PyReadonlyArray1<f64>) -> PyResult<()> {
        let input = input.as_slice()?;
        let (macds, signals, histograms) =
            (&mut self.macds, &mut self.signals, &mut self.histograms);
        py.allow_threads(|| {
            self.inner
                .extend_slices_into(input, macds, signals, histograms)
        });
        Ok(())
    }

    fn compute(&self, py: Python<'_>) -> (Py<PyArray1<f64>>, Py<PyArray1<f64>>, Py<PyArray1<f64>>) {
        (
            to_py_array(py, self.macds.clone()),
            to_py_array(py, self.signals.clone()),
            to_py_array(py, self.histograms.clone()),
        )
    }

    fn __len__(&self) -> usize {
        self.macds.len()
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64, f64)> {
        self.inner
            .value()
            .map(|value| (value.macd, value.signal, value.histogram))
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.macds.clear();
        self.signals.clear();
        self.histograms.clear();
    }
}

#[pymethods]
impl StatefulMacdFix {
    #[new]
    #[pyo3(signature = (signalperiod=9))]
    fn new(signalperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: MovingAverageConvergenceDivergenceFixed::new(signalperiod)
                .map_err(py_value_error)?,
            macds: Vec::new(),
            signals: Vec::new(),
            histograms: Vec::new(),
        })
    }

    fn append(&mut self, input: f64) -> Option<(f64, f64, f64)> {
        let value = self
            .inner
            .append(input)
            .map(|value| (value.macd, value.signal, value.histogram));
        let (macd, signal, histogram) = value.unwrap_or((f64::NAN, f64::NAN, f64::NAN));
        self.macds.push(macd);
        self.signals.push(signal);
        self.histograms.push(histogram);
        value
    }

    fn extend(&mut self, py: Python<'_>, input: PyReadonlyArray1<f64>) -> PyResult<()> {
        let input = input.as_slice()?;
        let (macds, signals, histograms) =
            (&mut self.macds, &mut self.signals, &mut self.histograms);
        py.allow_threads(|| {
            self.inner
                .extend_slices_into(input, macds, signals, histograms)
        });
        Ok(())
    }

    fn compute(&self, py: Python<'_>) -> (Py<PyArray1<f64>>, Py<PyArray1<f64>>, Py<PyArray1<f64>>) {
        (
            to_py_array(py, self.macds.clone()),
            to_py_array(py, self.signals.clone()),
            to_py_array(py, self.histograms.clone()),
        )
    }

    fn __len__(&self) -> usize {
        self.macds.len()
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64, f64)> {
        self.inner
            .value()
            .map(|value| (value.macd, value.signal, value.histogram))
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.macds.clear();
        self.signals.clear();
        self.histograms.clear();
    }
}

#[pymethods]
impl StatefulStochf {
    #[new]
    #[pyo3(signature = (fastk_period=5, fastd_period=3, fastd_matype=0))]
    fn new(fastk_period: usize, fastd_period: usize, fastd_matype: i32) -> PyResult<Self> {
        let ma_type = MaType::try_from(fastd_matype).map_err(py_value_error)?;
        Ok(Self {
            inner: FastStochasticOscillator::new(fastk_period, fastd_period, ma_type)
                .map_err(py_value_error)?,
            fastks: Vec::new(),
            fastds: Vec::new(),
        })
    }

    fn append(&mut self, high: f64, low: f64, close: f64) -> Option<(f64, f64)> {
        let value = self
            .inner
            .append(high, low, close)
            .map(|value| (value.fastk, value.fastd));
        let (fastk, fastd) = value.unwrap_or((f64::NAN, f64::NAN));
        self.fastks.push(fastk);
        self.fastds.push(fastd);
        value
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let high = high.as_slice()?;
        let low = low.as_slice()?;
        let close = close.as_slice()?;
        if high.len() != low.len() || high.len() != close.len() {
            return Err(PyValueError::new_err(
                "high, low, and close must have equal lengths",
            ));
        }
        let (fastks, fastds) = (&mut self.fastks, &mut self.fastds);
        py.allow_threads(|| {
            self.inner
                .extend_slices_into(high, low, close, fastks, fastds)
                .expect("lengths validated above")
        });
        Ok(())
    }

    fn compute(&self, py: Python<'_>) -> (Py<PyArray1<f64>>, Py<PyArray1<f64>>) {
        (
            to_py_array(py, self.fastks.clone()),
            to_py_array(py, self.fastds.clone()),
        )
    }

    fn __len__(&self) -> usize {
        self.fastks.len()
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64)> {
        self.inner.value().map(|value| (value.fastk, value.fastd))
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.fastks.clear();
        self.fastds.clear();
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
            inner: StochasticOscillator::new(
                fastk_period,
                slowk_period,
                slowk_type,
                slowd_period,
                slowd_type,
            )
            .map_err(py_value_error)?,
            slowks: Vec::new(),
            slowds: Vec::new(),
        })
    }

    fn append(&mut self, high: f64, low: f64, close: f64) -> Option<(f64, f64)> {
        let value = self
            .inner
            .append(high, low, close)
            .map(|value| (value.slowk, value.slowd));
        let (slowk, slowd) = value.unwrap_or((f64::NAN, f64::NAN));
        self.slowks.push(slowk);
        self.slowds.push(slowd);
        value
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let high = high.as_slice()?;
        let low = low.as_slice()?;
        let close = close.as_slice()?;
        if high.len() != low.len() || high.len() != close.len() {
            return Err(PyValueError::new_err(
                "high, low, and close must have equal lengths",
            ));
        }
        let (slowks, slowds) = (&mut self.slowks, &mut self.slowds);
        py.allow_threads(|| {
            self.inner
                .extend_slices_into(high, low, close, slowks, slowds)
                .expect("lengths validated above")
        });
        Ok(())
    }

    fn compute(&self, py: Python<'_>) -> (Py<PyArray1<f64>>, Py<PyArray1<f64>>) {
        (
            to_py_array(py, self.slowks.clone()),
            to_py_array(py, self.slowds.clone()),
        )
    }

    fn __len__(&self) -> usize {
        self.slowks.len()
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64)> {
        self.inner.value().map(|value| (value.slowk, value.slowd))
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.slowks.clear();
        self.slowds.clear();
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
            inner: StochasticRelativeStrengthIndex::new(
                timeperiod,
                fastk_period,
                fastd_period,
                ma_type,
            )
            .map_err(py_value_error)?,
            fastks: Vec::new(),
            fastds: Vec::new(),
        })
    }

    fn append(&mut self, input: f64) -> Option<(f64, f64)> {
        let value = self
            .inner
            .append(input)
            .map(|value| (value.fastk, value.fastd));
        let (fastk, fastd) = value.unwrap_or((f64::NAN, f64::NAN));
        self.fastks.push(fastk);
        self.fastds.push(fastd);
        value
    }

    fn extend(&mut self, py: Python<'_>, input: PyReadonlyArray1<f64>) -> PyResult<()> {
        let input = input.as_slice()?;
        let (fastks, fastds) = (&mut self.fastks, &mut self.fastds);
        py.allow_threads(|| self.inner.extend_slices_into(input, fastks, fastds));
        Ok(())
    }

    fn compute(&self, py: Python<'_>) -> (Py<PyArray1<f64>>, Py<PyArray1<f64>>) {
        (
            to_py_array(py, self.fastks.clone()),
            to_py_array(py, self.fastds.clone()),
        )
    }

    fn __len__(&self) -> usize {
        self.fastks.len()
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64)> {
        self.inner.value().map(|value| (value.fastk, value.fastd))
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.fastks.clear();
        self.fastds.clear();
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
            inner: MovingAverageConvergenceDivergenceExtended::new(
                fastperiod,
                fast_type,
                slowperiod,
                slow_type,
                signalperiod,
                signal_type,
            )
            .map_err(py_value_error)?,
            macds: Vec::new(),
            signals: Vec::new(),
            histograms: Vec::new(),
        })
    }

    fn append(&mut self, input: f64) -> Option<(f64, f64, f64)> {
        let value = self
            .inner
            .append(input)
            .map(|value| (value.macd, value.signal, value.histogram));
        let (macd, signal, histogram) = value.unwrap_or((f64::NAN, f64::NAN, f64::NAN));
        self.macds.push(macd);
        self.signals.push(signal);
        self.histograms.push(histogram);
        value
    }

    fn extend(&mut self, py: Python<'_>, input: PyReadonlyArray1<f64>) -> PyResult<()> {
        let input = input.as_slice()?;
        let (macds, signals, histograms) =
            (&mut self.macds, &mut self.signals, &mut self.histograms);
        py.allow_threads(|| {
            self.inner
                .extend_slices_into(input, macds, signals, histograms)
        });
        Ok(())
    }

    fn compute(&self, py: Python<'_>) -> (Py<PyArray1<f64>>, Py<PyArray1<f64>>, Py<PyArray1<f64>>) {
        (
            to_py_array(py, self.macds.clone()),
            to_py_array(py, self.signals.clone()),
            to_py_array(py, self.histograms.clone()),
        )
    }

    fn __len__(&self) -> usize {
        self.macds.len()
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64, f64)> {
        self.inner
            .value()
            .map(|value| (value.macd, value.signal, value.histogram))
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.macds.clear();
        self.signals.clear();
        self.histograms.clear();
    }
}

#[pymethods]
impl VariablePeriodMovingAverage {
    #[new]
    #[pyo3(signature = (minperiod=2, maxperiod=30, matype=0))]
    fn new(minperiod: usize, maxperiod: usize, matype: i32) -> PyResult<Self> {
        let ma_type = MaType::try_from(matype).map_err(py_value_error)?;
        Ok(Self {
            inner: CoreVariablePeriodMovingAverage::new(minperiod, maxperiod, ma_type)
                .map_err(py_value_error)?,
            outputs: Vec::new(),
        })
    }

    fn append(&mut self, input: f64, period: f64) -> Option<f64> {
        push_option(&mut self.outputs, self.inner.append(input, period))
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        input: PyReadonlyArray1<f64>,
        periods: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let input = input.as_slice()?;
        let periods = periods.as_slice()?;
        if input.len() != periods.len() {
            return Err(PyValueError::new_err(
                "input and periods must have equal lengths",
            ));
        }
        let inner = &mut self.inner;
        let outputs = &mut self.outputs;
        py.allow_threads(|| inner.extend_slices_into(input, periods, outputs))
            .map_err(py_value_error)?;
        Ok(())
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn compute(&self, py: Python<'_>) -> Py<PyArray1<f64>> {
        to_py_array(py, self.outputs.clone())
    }

    fn __len__(&self) -> usize {
        self.outputs.len()
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.outputs.clear();
    }
}

#[pymethods]
impl StatefulHtTrendline {
    #[new]
    fn new() -> Self {
        Self {
            inner: HilbertTransformTrendline::new(),
            outputs: Vec::new(),
        }
    }

    fn append(&mut self, input: f64) -> Option<f64> {
        push_option(&mut self.outputs, self.inner.append(input))
    }

    fn extend(&mut self, py: Python<'_>, input: PyReadonlyArray1<f64>) -> PyResult<()> {
        let input = input.as_slice()?;
        let outputs = &mut self.outputs;
        py.allow_threads(|| {
            outputs.extend(
                input
                    .iter()
                    .map(|&input| self.inner.append(input).unwrap_or(f64::NAN)),
            )
        });
        Ok(())
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn compute(&self, py: Python<'_>) -> Py<PyArray1<f64>> {
        to_py_array(py, self.outputs.clone())
    }

    fn __len__(&self) -> usize {
        self.outputs.len()
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.outputs.clear();
    }
}

#[pymethods]
impl StatefulAdx {
    #[new]
    #[pyo3(signature = (timeperiod=14))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: AverageDirectionalIndex::new(timeperiod).map_err(py_value_error)?,
            outputs: Vec::new(),
        })
    }

    fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        push_option(&mut self.outputs, self.inner.append(high, low, close))
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let high = high.as_slice()?;
        let low = low.as_slice()?;
        let close = close.as_slice()?;
        if high.len() != low.len() || high.len() != close.len() {
            return Err(PyValueError::new_err(
                "high, low, and close must have equal lengths",
            ));
        }
        let outputs = &mut self.outputs;
        py.allow_threads(|| self.inner.extend_slices_into(high, low, close, outputs));
        Ok(())
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn compute(&self, py: Python<'_>) -> Py<PyArray1<f64>> {
        to_py_array(py, self.outputs.clone())
    }

    fn __len__(&self) -> usize {
        self.outputs.len()
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.outputs.clear();
    }
}

#[pymethods]
impl StatefulAdxr {
    #[new]
    #[pyo3(signature = (timeperiod=14))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: AverageDirectionalIndexRating::new(timeperiod).map_err(py_value_error)?,
            outputs: Vec::new(),
        })
    }

    fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        push_option(&mut self.outputs, self.inner.append(high, low, close))
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let high = high.as_slice()?;
        let low = low.as_slice()?;
        let close = close.as_slice()?;
        if high.len() != low.len() || high.len() != close.len() {
            return Err(PyValueError::new_err(
                "high, low, and close must have equal lengths",
            ));
        }
        let outputs = &mut self.outputs;
        py.allow_threads(|| self.inner.extend_slices_into(high, low, close, outputs));
        Ok(())
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn compute(&self, py: Python<'_>) -> Py<PyArray1<f64>> {
        to_py_array(py, self.outputs.clone())
    }

    fn __len__(&self) -> usize {
        self.outputs.len()
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.outputs.clear();
    }
}

#[pymethods]
impl StatefulDx {
    #[new]
    #[pyo3(signature = (timeperiod=14))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: DirectionalMovementIndex::new(timeperiod).map_err(py_value_error)?,
            outputs: Vec::new(),
        })
    }

    fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        push_option(&mut self.outputs, self.inner.append(high, low, close))
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let high = high.as_slice()?;
        let low = low.as_slice()?;
        let close = close.as_slice()?;
        if high.len() != low.len() || high.len() != close.len() {
            return Err(PyValueError::new_err(
                "high, low, and close must have equal lengths",
            ));
        }
        let outputs = &mut self.outputs;
        py.allow_threads(|| {
            outputs.extend(
                high.iter()
                    .zip(low)
                    .zip(close)
                    .map(|((&high, &low), &close)| {
                        self.inner.append(high, low, close).unwrap_or(f64::NAN)
                    }),
            )
        });
        Ok(())
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn compute(&self, py: Python<'_>) -> Py<PyArray1<f64>> {
        to_py_array(py, self.outputs.clone())
    }

    fn __len__(&self) -> usize {
        self.outputs.len()
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.outputs.clear();
    }
}
