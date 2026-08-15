use crate::conversion::to_py_array;
use crate::state_api::push_option;
use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyclass]
pub struct ParabolicSarExtended {
    inner: taflow::indicators::ParabolicSarExtended,
    outputs: Vec<f64>,
}

#[pymethods]
impl ParabolicSarExtended {
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
            inner: taflow::indicators::ParabolicSarExtended::new(
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
        py.allow_threads(|| self.inner.extend_slice_into(high, low, &mut self.outputs));
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
