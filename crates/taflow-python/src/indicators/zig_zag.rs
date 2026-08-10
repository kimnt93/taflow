use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::indicators::{ZigZag as State, ZigZagValue};
#[pyclass]
pub struct ZigZag {
    inner: State,
    swing: Vec<f64>,
    direction: Vec<f64>,
}
#[pymethods]
impl ZigZag {
    #[new]
    #[pyo3(signature=(threshold=0.05))]
    fn new(threshold: f64) -> PyResult<Self> {
        Ok(Self {
            inner: State::new(threshold)
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?,
            swing: Vec::new(),
            direction: Vec::new(),
        })
    }
    fn append(&mut self, high: f64, low: f64) -> Option<(f64, f64)> {
        let x = self.inner.append(high, low);
        let value = x.unwrap_or(ZigZagValue {
            swing: f64::NAN,
            direction: f64::NAN,
        });
        self.swing.push(value.swing);
        self.direction.push(value.direction);
        x.map(|value| (value.swing, value.direction))
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
                "high and low must have equal lengths",
            ));
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
            PyArray1::from_vec(py, self.swing.clone()),
            PyArray1::from_vec(py, self.direction.clone()),
        )
    }
    #[getter]
    fn value(&self) -> Option<(f64, f64)> {
        self.inner
            .value()
            .map(|value| (value.swing, value.direction))
    }
    fn reset(&mut self) {
        self.inner.reset();
        self.swing.clear();
        self.direction.clear();
    }
    fn __len__(&self) -> usize {
        self.swing.len()
    }
}
