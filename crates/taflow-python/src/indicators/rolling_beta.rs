use crate::conversion::to_py_array;
use crate::state_api::{push_option, py_value_error};
use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::indicators::RollingBeta as State;

#[pyclass]
pub struct RollingBeta {
    inner: State,
    outputs: Vec<f64>,
}

#[pymethods]
impl RollingBeta {
    #[new]
    #[pyo3(signature = (timeperiod=5))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: State::new(timeperiod).map_err(py_value_error)?,
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
            return Err(pyo3::exceptions::PyValueError::new_err(
                "inputs must have equal lengths",
            ));
        }
        py.allow_threads(|| {
            self.inner
                .extend_slices_into(input0, input1, &mut self.outputs)
        })
        .map_err(py_value_error)
    }
    fn compute(&self, py: Python<'_>) -> Py<PyArray1<f64>> {
        to_py_array(py, self.outputs.clone())
    }
    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }
    fn reset(&mut self) {
        self.inner.reset();
        self.outputs.clear();
    }
    fn __len__(&self) -> usize {
        self.outputs.len()
    }
}
