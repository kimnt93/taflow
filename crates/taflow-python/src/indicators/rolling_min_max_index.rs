use crate::conversion::to_py_array;
use crate::state_api::{extend_from_options, push_option, py_value_error};
use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::indicators;

#[pyclass]
pub struct RollingMinMaxIndex {
    inner: taflow::indicators::RollingMinMaxIndex,
    minimums: Vec<f64>,
    maximums: Vec<f64>,
}

#[pymethods]
impl RollingMinMaxIndex {
    #[new]
    #[pyo3(signature = (timeperiod=30))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: taflow::indicators::RollingMinMaxIndex::new(timeperiod)
                .map_err(py_value_error)?,
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
