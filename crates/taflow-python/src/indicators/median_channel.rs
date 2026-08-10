use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::indicators::{MedianChannel as State, MedianChannelValue};
#[pyclass]
pub struct MedianChannel {
    inner: State,
    upper: Vec<f64>,
    middle: Vec<f64>,
    lower: Vec<f64>,
}
#[pymethods]
impl MedianChannel {
    #[new]
    #[pyo3(signature=(period=20,multiplier=2.0))]
    fn new(period: usize, multiplier: f64) -> PyResult<Self> {
        Ok(Self {
            inner: State::new(period, multiplier)
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?,
            upper: Vec::new(),
            middle: Vec::new(),
            lower: Vec::new(),
        })
    }
    fn append(&mut self, x: f64) -> Option<(f64, f64, f64)> {
        let z = self.inner.append(x);
        let v = z.unwrap_or(MedianChannelValue {
            upper: f64::NAN,
            middle: f64::NAN,
            lower: f64::NAN,
        });
        self.upper.push(v.upper);
        self.middle.push(v.middle);
        self.lower.push(v.lower);
        z.map(|v| (v.upper, v.middle, v.lower))
    }
    fn extend(&mut self, py: Python<'_>, x: PyReadonlyArray1<f64>) -> PyResult<()> {
        let x = x.as_slice()?;
        py.allow_threads(|| {
            for &v in x {
                self.append(v);
            }
        });
        Ok(())
    }
    fn compute<'py>(
        &self,
        py: Python<'py>,
    ) -> (
        Bound<'py, PyArray1<f64>>,
        Bound<'py, PyArray1<f64>>,
        Bound<'py, PyArray1<f64>>,
    ) {
        (
            PyArray1::from_vec(py, self.upper.clone()),
            PyArray1::from_vec(py, self.middle.clone()),
            PyArray1::from_vec(py, self.lower.clone()),
        )
    }
    #[getter]
    fn value(&self) -> Option<(f64, f64, f64)> {
        self.inner.value().map(|v| (v.upper, v.middle, v.lower))
    }
    fn reset(&mut self) {
        self.inner.reset();
        self.upper.clear();
        self.middle.clear();
        self.lower.clear();
    }
    fn __len__(&self) -> usize {
        self.upper.len()
    }
}
