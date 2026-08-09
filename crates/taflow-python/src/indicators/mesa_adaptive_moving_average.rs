use crate::conversion::to_py_array;
use crate::state_api::{extend_from_options, push_option, py_value_error};
use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::indicators;
use taflow::indicators::MesaAdaptiveMovingAverage as State;
use taflow::stream::StreamingIndicator;

#[pyclass]
pub struct MesaAdaptiveMovingAverage {
    inner: State,
    mamas: Vec<f64>,
    famas: Vec<f64>,
}

#[pymethods]
impl MesaAdaptiveMovingAverage {
    #[new]
    #[pyo3(signature = (fastlimit=0.5, slowlimit=0.05))]
    fn new(fastlimit: f64, slowlimit: f64) -> PyResult<Self> {
        Ok(Self {
            inner: State::new(fastlimit, slowlimit).map_err(py_value_error)?,
            mamas: Vec::new(),
            famas: Vec::new(),
        })
    }

    fn append(&mut self, input: f64) -> Option<(f64, f64)> {
        let value = self
            .inner
            .append(input)
            .map(|value| (value.mama, value.fama));
        let (mama, fama) = value.unwrap_or((f64::NAN, f64::NAN));
        self.mamas.push(mama);
        self.famas.push(fama);
        value
    }

    fn extend(&mut self, py: Python<'_>, input: PyReadonlyArray1<f64>) -> PyResult<()> {
        let input = input.as_slice()?;
        let (mamas, famas) = (&mut self.mamas, &mut self.famas);
        py.allow_threads(|| {
            mamas.reserve(input.len());
            famas.reserve(input.len());
            for input in input.iter().copied() {
                if let Some(value) = self.inner.append(input) {
                    mamas.push(value.mama);
                    famas.push(value.fama);
                } else {
                    mamas.push(f64::NAN);
                    famas.push(f64::NAN);
                }
            }
        });
        Ok(())
    }

    fn compute(&self, py: Python<'_>) -> (Py<PyArray1<f64>>, Py<PyArray1<f64>>) {
        (
            to_py_array(py, self.mamas.clone()),
            to_py_array(py, self.famas.clone()),
        )
    }

    fn __len__(&self) -> usize {
        self.mamas.len()
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64)> {
        self.inner.value().map(|value| (value.mama, value.fama))
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.mamas.clear();
        self.famas.clear();
    }
}
