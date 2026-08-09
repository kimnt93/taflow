use crate::conversion::to_py_array;
use crate::state_api::{extend_from_options, push_option, py_value_error};
use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::indicators;

#[pyclass]
pub struct RollingMinMax {
    inner: taflow::indicators::RollingMinMax,
    minimums: Vec<f64>,
    maximums: Vec<f64>,
}

#[pymethods]
impl RollingMinMax {
    #[new]
    #[pyo3(signature = (timeperiod=30))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: taflow::indicators::RollingMinMax::new(timeperiod).map_err(py_value_error)?,
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
