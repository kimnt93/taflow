use crate::conversion::to_py_array;
use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::MinusDirectionalMovement as NativeMinusDirectionalMovement;

#[pyclass]
pub struct MinusDirectionalMovement {
    inner: NativeMinusDirectionalMovement,
    outputs: Vec<f64>,
}

#[pymethods]
impl MinusDirectionalMovement {
    #[new]
    #[pyo3(signature=(timeperiod=14))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: NativeMinusDirectionalMovement::new(timeperiod)
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
            outputs: Vec::new(),
        })
    }

    fn append(&mut self, high: f64, low: f64) -> Option<f64> {
        let value = self.inner.append(high, low);
        self.outputs.push(value.unwrap_or(f64::NAN));
        value
    }

    fn extend(&mut self, high: PyReadonlyArray1<f64>, low: PyReadonlyArray1<f64>) -> PyResult<()> {
        let high = high.as_slice()?;
        let low = low.as_slice()?;
        if high.len() != low.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        self.outputs.extend(
            high.iter()
                .zip(low)
                .map(|(&high, &low)| self.inner.append(high, low).unwrap_or(f64::NAN)),
        );
        Ok(())
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
        self.outputs.clear()
    }
}
