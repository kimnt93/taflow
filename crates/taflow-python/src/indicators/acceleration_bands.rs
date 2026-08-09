use crate::conversion::to_py_array;
use crate::state_api::{extend_from_options, push_option, py_value_error};
use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::indicators;

#[pyclass]
pub struct AccelerationBands {
    inner: taflow::indicators::AccelerationBands,
    uppers: Vec<f64>,
    middles: Vec<f64>,
    lowers: Vec<f64>,
}

#[pymethods]
impl AccelerationBands {
    #[new]
    #[pyo3(signature = (timeperiod=20))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: taflow::indicators::AccelerationBands::new(timeperiod)
                .map_err(py_value_error)?,
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
