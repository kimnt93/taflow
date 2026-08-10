use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::indicators::{DoubleBollingerBands as State, DoubleBollingerBandsValue};

#[pyclass]
pub struct DoubleBollingerBands {
    inner: State,
    upper_outer: Vec<f64>,
    upper_inner: Vec<f64>,
    middle: Vec<f64>,
    lower_inner: Vec<f64>,
    lower_outer: Vec<f64>,
}

#[pymethods]
impl DoubleBollingerBands {
    #[new]
    fn new(period: usize, inner_multiplier: f64, outer_multiplier: f64) -> PyResult<Self> {
        Ok(Self {
            inner: State::new(period, inner_multiplier, outer_multiplier)
                .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.to_string()))?,
            upper_outer: Vec::new(),
            upper_inner: Vec::new(),
            middle: Vec::new(),
            lower_inner: Vec::new(),
            lower_outer: Vec::new(),
        })
    }

    fn append(&mut self, input: f64) -> Option<(f64, f64, f64, f64, f64)> {
        let result = self.inner.append(input);
        let value = result.unwrap_or(DoubleBollingerBandsValue {
            upper_outer: f64::NAN,
            upper_inner: f64::NAN,
            middle: f64::NAN,
            lower_inner: f64::NAN,
            lower_outer: f64::NAN,
        });
        self.upper_outer.push(value.upper_outer);
        self.upper_inner.push(value.upper_inner);
        self.middle.push(value.middle);
        self.lower_inner.push(value.lower_inner);
        self.lower_outer.push(value.lower_outer);
        result.map(|value| {
            (
                value.upper_outer,
                value.upper_inner,
                value.middle,
                value.lower_inner,
                value.lower_outer,
            )
        })
    }

    fn extend(&mut self, py: Python<'_>, input: PyReadonlyArray1<f64>) -> PyResult<()> {
        let input = input.as_slice()?;
        py.allow_threads(|| {
            for &value in input {
                self.append(value);
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
        Bound<'py, PyArray1<f64>>,
        Bound<'py, PyArray1<f64>>,
    ) {
        (
            PyArray1::from_vec(py, self.upper_outer.clone()),
            PyArray1::from_vec(py, self.upper_inner.clone()),
            PyArray1::from_vec(py, self.middle.clone()),
            PyArray1::from_vec(py, self.lower_inner.clone()),
            PyArray1::from_vec(py, self.lower_outer.clone()),
        )
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64, f64, f64, f64)> {
        self.inner.value().map(|value| {
            (
                value.upper_outer,
                value.upper_inner,
                value.middle,
                value.lower_inner,
                value.lower_outer,
            )
        })
    }
    fn reset(&mut self) {
        self.inner.reset();
        self.upper_outer.clear();
        self.upper_inner.clear();
        self.middle.clear();
        self.lower_inner.clear();
        self.lower_outer.clear();
    }
    fn __len__(&self) -> usize {
        self.middle.len()
    }
}
