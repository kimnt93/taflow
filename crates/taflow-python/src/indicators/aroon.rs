use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::indicators::Aroon as State;

#[pyclass]
pub struct Aroon {
    inner: State,
    downs: Vec<f64>,
    ups: Vec<f64>,
}

#[pymethods]
impl Aroon {
    #[new]
    #[pyo3(signature = (timeperiod=14))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: State::new(timeperiod).map_err(|e| PyValueError::new_err(e.to_string()))?,
            downs: Vec::new(),
            ups: Vec::new(),
        })
    }
    fn append(&mut self, high: f64, low: f64) -> Option<(f64, f64)> {
        let value = self.inner.append(high, low).map(|v| (v.down, v.up));
        let (down, up) = value.unwrap_or((f64::NAN, f64::NAN));
        self.downs.push(down);
        self.ups.push(up);
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
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        py.allow_threads(|| {
            for (&h, &l) in high.iter().zip(low) {
                self.append(h, l);
            }
        });
        Ok(())
    }
    fn compute<'py>(
        &self,
        py: Python<'py>,
    ) -> (Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>) {
        (
            PyArray1::from_vec(py, self.downs.clone()),
            PyArray1::from_vec(py, self.ups.clone()),
        )
    }
    fn __len__(&self) -> usize {
        self.downs.len()
    }
    #[getter]
    fn value(&self) -> Option<(f64, f64)> {
        self.inner.value().map(|v| (v.down, v.up))
    }
    fn reset(&mut self) {
        self.inner.reset();
        self.downs.clear();
        self.ups.clear();
    }
}
