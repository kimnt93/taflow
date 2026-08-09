use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::stream::ChaikinVolatility;

#[pyclass]
pub struct ChaikinVolatilityOperator {
    inner: ChaikinVolatility,
    outputs: Vec<f64>,
}

#[pymethods]
impl ChaikinVolatilityOperator {
    #[new]
    fn new(timeperiod: usize, roc_period: usize) -> PyResult<Self> {
        Ok(Self {
            inner: ChaikinVolatility::new(timeperiod, roc_period)
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?,
            outputs: Vec::new(),
        })
    }
    fn append(&mut self, high: f64, low: f64) -> Option<f64> {
        let value = self.inner.append(high, low);
        self.outputs.push(value.unwrap_or(f64::NAN));
        value
    }
    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (high, low) = (high.as_slice()?, low.as_slice()?);
        if high.len() != low.len() {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "inputs must have equal lengths",
            ));
        }
        py.allow_threads(|| {
            high.iter().zip(low).for_each(|(&h, &l)| {
                self.append(h, l);
            })
        });
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
    fn __len__(&self) -> usize {
        self.outputs.len()
    }
}
