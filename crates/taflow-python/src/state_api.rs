//! PyO3 adapters for the incremental core API.

use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::{
    self, Atr, Dema, Ema, Imi, Macd, MacdFix, Mama, Midpoint, Midprice, Mom, Natr, Roc, Rocp, Rocr,
    Rocr100, Rsi, Sma, Stoch, Stochf, Stochrsi, StreamingIndicator, Tema, Trange, Trima, Wma,
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
scalar_state_class!(StatefulMidpoint, Midpoint, 14);
scalar_state_class!(StatefulMax, stream::Max, 30);
scalar_state_class!(StatefulMaxindex, stream::Maxindex, 30);
scalar_state_class!(StatefulMin, stream::Min, 30);
scalar_state_class!(StatefulMinindex, stream::Minindex, 30);
scalar_state_class!(StatefulSum, stream::Sum, 30);
scalar_state_class!(StatefulAvgdev, stream::Avgdev, 14);
scalar_state_class!(StatefulCmo, stream::Cmo, 14);
scalar_state_class!(StatefulKama, stream::Kama, 30);
scalar_state_class!(StatefulLinearreg, stream::Linearreg, 14);
scalar_state_class!(StatefulLinearregSlope, stream::LinearregSlope, 14);
scalar_state_class!(StatefulLinearregIntercept, stream::LinearregIntercept, 14);
scalar_state_class!(StatefulLinearregAngle, stream::LinearregAngle, 14);
scalar_state_class!(StatefulTsf, stream::Tsf, 14);

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

deviation_state_class!(StatefulVar, Var);
deviation_state_class!(StatefulStddev, Stddev);

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

bivariate_statistic_class!(StatefulBeta, Beta);
bivariate_statistic_class!(StatefulCorrel, Correl);

#[pyclass]
pub struct StatefulAd {
    inner: stream::Ad,
}

#[pymethods]
impl StatefulAd {
    #[new]
    fn new() -> Self {
        Self {
            inner: stream::Ad::new(),
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
    inner: stream::Adosc,
}

#[pymethods]
impl StatefulAdosc {
    #[new]
    #[pyo3(signature = (fastperiod=3, slowperiod=10))]
    fn new(fastperiod: usize, slowperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: stream::Adosc::new(fastperiod, slowperiod).map_err(py_value_error)?,
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
    inner: stream::Obv,
}

#[pymethods]
impl StatefulObv {
    #[new]
    fn new() -> Self {
        Self {
            inner: stream::Obv::new(),
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
    inner: stream::Bop,
}

#[pymethods]
impl StatefulBop {
    #[new]
    fn new() -> Self {
        Self {
            inner: stream::Bop::new(),
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
    inner: stream::Willr,
}

#[pymethods]
impl StatefulWillr {
    #[new]
    #[pyo3(signature = (timeperiod=14))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: stream::Willr::new(timeperiod).map_err(py_value_error)?,
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
    inner: stream::Aroonosc,
}

#[pymethods]
impl StatefulAroonosc {
    #[new]
    #[pyo3(signature = (timeperiod=14))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: stream::Aroonosc::new(timeperiod).map_err(py_value_error)?,
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
    inner: stream::Minmax,
}

#[pymethods]
impl StatefulMinmax {
    #[new]
    #[pyo3(signature = (timeperiod=30))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: stream::Minmax::new(timeperiod).map_err(py_value_error)?,
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
    inner: stream::Minmaxindex,
}

#[pymethods]
impl StatefulMinmaxindex {
    #[new]
    #[pyo3(signature = (timeperiod=30))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: stream::Minmaxindex::new(timeperiod).map_err(py_value_error)?,
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
    inner: stream::Avgprice,
}

#[pymethods]
impl StatefulAvgprice {
    #[new]
    fn new() -> Self {
        Self {
            inner: stream::Avgprice::new(),
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
    inner: Midprice,
}

#[pymethods]
impl StatefulMidprice {
    #[new]
    #[pyo3(signature = (timeperiod=14))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: Midprice::new(timeperiod).map_err(py_value_error)?,
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
    inner: Atr,
}

#[pymethods]
impl StatefulAtr {
    #[new]
    #[pyo3(signature = (timeperiod=14))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: Atr::new(timeperiod).map_err(py_value_error)?,
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
    inner: Trange,
}

#[pymethods]
impl StatefulTrange {
    #[new]
    fn new() -> Self {
        Self {
            inner: Trange::new(),
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
    inner: Natr,
}

#[pymethods]
impl StatefulNatr {
    #[new]
    #[pyo3(signature = (timeperiod=14))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: Natr::new(timeperiod).map_err(py_value_error)?,
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
