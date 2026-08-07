use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::stream::SignedPower;

/// Native persistent adapter for pointwise signed power.
#[pyclass]
pub struct SignedPowerOperator {
    inner: SignedPower,
    outputs: Vec<f64>,
}

#[pymethods]
impl SignedPowerOperator {
    #[new]
    fn new(exponent: f64) -> Self {
        Self {
            inner: SignedPower::new(exponent),
            outputs: Vec::new(),
        }
    }

    fn append(&mut self, input: f64) -> f64 {
        let value = self.inner.append(input);
        self.outputs.push(value);
        value
    }

    fn extend(&mut self, input: PyReadonlyArray1<f64>) -> PyResult<()> {
        for &value in input.as_slice()? {
            self.append(value);
        }
        Ok(())
    }

    fn compute<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        PyArray1::from_vec(py, self.outputs.clone())
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
