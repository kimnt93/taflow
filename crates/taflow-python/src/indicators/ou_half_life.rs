use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::OrnsteinUhlenbeckHalfLife;

#[pyclass]
pub struct OrnsteinUhlenbeckHalfLifeOperator {
    inner: OrnsteinUhlenbeckHalfLife,
    output: Vec<f64>,
}

#[pymethods]
impl OrnsteinUhlenbeckHalfLifeOperator {
    #[new]
    #[pyo3(signature = (timeperiod=20))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: OrnsteinUhlenbeckHalfLife::new(timeperiod)
                .map_err(|error| PyValueError::new_err(error.to_string()))?,
            output: Vec::new(),
        })
    }

    fn append(&mut self, price: f64) -> Option<f64> {
        let value = self.inner.append(price);
        self.output.push(value.unwrap_or(f64::NAN));
        value
    }

    fn extend(&mut self, price: PyReadonlyArray1<f64>) -> PyResult<()> {
        for &price in price.as_slice()? {
            self.append(price);
        }
        Ok(())
    }

    fn compute<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        PyArray1::from_vec(py, self.output.clone())
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.output.clear();
    }
}
