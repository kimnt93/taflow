use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::stream::KeltnerChannels;

#[pyclass]
pub struct KeltnerChannelsOperator {
    inner: KeltnerChannels,
    upper: Vec<f64>,
    middle: Vec<f64>,
    lower: Vec<f64>,
}

#[pymethods]
impl KeltnerChannelsOperator {
    #[new]
    fn new(timeperiod: usize, multiplier: f64) -> PyResult<Self> {
        Ok(Self {
            inner: KeltnerChannels::new(timeperiod, multiplier)
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?,
            upper: Vec::new(),
            middle: Vec::new(),
            lower: Vec::new(),
        })
    }
    fn append(&mut self, high: f64, low: f64, close: f64) -> Option<(f64, f64, f64)> {
        let value = self.inner.append(high, low, close);
        if let Some(value) = value {
            self.upper.push(value.upper);
            self.middle.push(value.middle);
            self.lower.push(value.lower);
            Some((value.upper, value.middle, value.lower))
        } else {
            self.upper.push(f64::NAN);
            self.middle.push(f64::NAN);
            self.lower.push(f64::NAN);
            None
        }
    }
    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (high, low, close) = (high.as_slice()?, low.as_slice()?, close.as_slice()?);
        if high.len() != low.len() || high.len() != close.len() {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "inputs must have equal lengths",
            ));
        }
        py.allow_threads(|| {
            high.iter().zip(low).zip(close).for_each(|((&h, &l), &c)| {
                self.append(h, l, c);
            })
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
