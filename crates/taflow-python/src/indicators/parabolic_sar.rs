use crate::conversion::to_py_array;
use crate::state_api::{extend_from_options, push_option, py_value_error};
use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::indicators;

#[pyclass]
pub struct ParabolicSar {
    inner: taflow::indicators::ParabolicSar,
    outputs: Vec<f64>,
}

#[pymethods]
impl ParabolicSar {
    #[new]
    #[pyo3(signature = (acceleration=0.02, maximum=0.2))]
    fn new(acceleration: f64, maximum: f64) -> Self {
        Self {
            inner: taflow::indicators::ParabolicSar::new(acceleration, maximum),
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
