use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::DetrendedPriceOscillator;

#[pyclass]
pub struct DetrendedPriceOscillatorOperator { inner: DetrendedPriceOscillator, values: Vec<f64> }

#[pymethods]
impl DetrendedPriceOscillatorOperator {
    #[new]
    #[pyo3(signature = (period=20))]
    fn new(period: usize) -> PyResult<Self> {
        Ok(Self { inner: DetrendedPriceOscillator::new(period).map_err(|error| PyValueError::new_err(error.to_string()))?, values: Vec::new() })
    }

    fn append(&mut self, close: f64) -> Option<f64> {
        let value = self.inner.append(close);
        self.values.push(value.unwrap_or(f64::NAN));
        value
    }

    fn extend(&mut self, close: PyReadonlyArray1<f64>) -> PyResult<()> {
        for &close in close.as_slice()? { self.append(close); }
        Ok(())
    }

    fn compute<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> { PyArray1::from_vec(py, self.values.clone()) }

    #[getter]
    fn value(&self) -> Option<f64> { self.inner.value() }

    fn reset(&mut self) { self.inner.reset(); self.values.clear(); }
}
