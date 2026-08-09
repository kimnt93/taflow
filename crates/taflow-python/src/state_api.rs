//! PyO3 adapters for the incremental core API.

use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::{
    self, AverageDirectionalIndex, AverageDirectionalIndexRating,
    AverageTrueRange as CoreAverageTrueRange, DirectionalMovementIndex, FastStochasticOscillator,
    MesaAdaptiveMovingAverage, NormalizedAverageTrueRange as CoreNormalizedAverageTrueRange,
    StochasticOscillator, StochasticRelativeStrengthIndex, StreamingIndicator,
    TrueRange as CoreTrueRange, VariablePeriodMovingAverage as CoreVariablePeriodMovingAverage,
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

scalar_state_class!(StatefulMax, stream::RollingMax, 30);
scalar_state_class!(StatefulMaxindex, stream::RollingArgmax, 30);
scalar_state_class!(StatefulMin, stream::RollingMin, 30);
scalar_state_class!(StatefulMinindex, stream::RollingArgmin, 30);
scalar_state_class!(StatefulLinearreg, stream::Linearreg, 14);
scalar_state_class!(StatefulLinearregSlope, stream::LinearregSlope, 14);
scalar_state_class!(StatefulLinearregIntercept, stream::LinearregIntercept, 14);
scalar_state_class!(StatefulLinearregAngle, stream::LinearregAngle, 14);
scalar_state_class!(StatefulTsf, stream::Tsf, 14);

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
pub struct VariablePeriodMovingAverage {
    inner: CoreVariablePeriodMovingAverage,
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
