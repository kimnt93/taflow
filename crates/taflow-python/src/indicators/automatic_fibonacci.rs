use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::indicators::{AutomaticFibonacci as State, AutomaticFibonacciValue};

#[pyclass]
pub struct AutomaticFibonacci {
    inner: State,
    level_000: Vec<f64>,
    level_236: Vec<f64>,
    level_382: Vec<f64>,
    level_500: Vec<f64>,
    level_618: Vec<f64>,
    level_786: Vec<f64>,
    level_100: Vec<f64>,
}

#[pymethods]
impl AutomaticFibonacci {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            inner: State::new()
                .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.to_string()))?,
            level_000: Vec::new(),
            level_236: Vec::new(),
            level_382: Vec::new(),
            level_500: Vec::new(),
            level_618: Vec::new(),
            level_786: Vec::new(),
            level_100: Vec::new(),
        })
    }

    fn append(&mut self, high: f64, low: f64) -> Option<(f64, f64, f64, f64, f64, f64, f64)> {
        let result = self.inner.append(high, low);
        let value = result.unwrap_or(AutomaticFibonacciValue {
            level_000: f64::NAN,
            level_236: f64::NAN,
            level_382: f64::NAN,
            level_500: f64::NAN,
            level_618: f64::NAN,
            level_786: f64::NAN,
            level_100: f64::NAN,
        });
        self.level_000.push(value.level_000);
        self.level_236.push(value.level_236);
        self.level_382.push(value.level_382);
        self.level_500.push(value.level_500);
        self.level_618.push(value.level_618);
        self.level_786.push(value.level_786);
        self.level_100.push(value.level_100);
        result.map(|value| {
            (
                value.level_000,
                value.level_236,
                value.level_382,
                value.level_500,
                value.level_618,
                value.level_786,
                value.level_100,
            )
        })
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let high = high.as_slice()?;
        let low = low.as_slice()?;
        if high.len() != low.len() {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "high and low inputs must have equal lengths",
            ));
        }
        py.allow_threads(|| {
            for index in 0..high.len() {
                self.append(high[index], low[index]);
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
        Bound<'py, PyArray1<f64>>,
        Bound<'py, PyArray1<f64>>,
    ) {
        (
            PyArray1::from_vec(py, self.level_000.clone()),
            PyArray1::from_vec(py, self.level_236.clone()),
            PyArray1::from_vec(py, self.level_382.clone()),
            PyArray1::from_vec(py, self.level_500.clone()),
            PyArray1::from_vec(py, self.level_618.clone()),
            PyArray1::from_vec(py, self.level_786.clone()),
            PyArray1::from_vec(py, self.level_100.clone()),
        )
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64, f64, f64, f64, f64, f64)> {
        self.inner.value().map(|value| {
            (
                value.level_000,
                value.level_236,
                value.level_382,
                value.level_500,
                value.level_618,
                value.level_786,
                value.level_100,
            )
        })
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.level_000.clear();
        self.level_236.clear();
        self.level_382.clear();
        self.level_500.clear();
        self.level_618.clear();
        self.level_786.clear();
        self.level_100.clear();
    }

    fn __len__(&self) -> usize {
        self.inner.len()
    }
}
