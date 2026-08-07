use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::GarmanKlassYangZhang;

#[pyclass]
pub struct GkYangZhangOperator {
    inner: GarmanKlassYangZhang,
    output: Vec<f64>,
}

#[pymethods]
impl GkYangZhangOperator {
    #[new]
    #[pyo3(signature = (timeperiod=20))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: GarmanKlassYangZhang::new(timeperiod)
                .map_err(|error| PyValueError::new_err(error.to_string()))?,
            output: Vec::new(),
        })
    }

    fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> Option<f64> {
        let value = self.inner.append(open, high, low, close);
        self.output.push(value.unwrap_or(f64::NAN));
        value
    }

    fn extend(
        &mut self,
        open: PyReadonlyArray1<f64>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (open, high, low, close) = (
            open.as_slice()?,
            high.as_slice()?,
            low.as_slice()?,
            close.as_slice()?,
        );
        if open.len() != high.len() || high.len() != low.len() || low.len() != close.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        for (((&open, &high), &low), &close) in open.iter().zip(high).zip(low).zip(close) {
            self.append(open, high, low, close);
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
