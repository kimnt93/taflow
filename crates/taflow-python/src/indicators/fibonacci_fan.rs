use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::indicators::{FibonacciFan as State, FibonacciFanValue};

#[pyclass]
pub struct FibonacciFan {
    inner: State,
    fan_382: Vec<f64>,
    fan_500: Vec<f64>,
    fan_618: Vec<f64>,
}

#[pymethods]
impl FibonacciFan {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            inner: State::new()
                .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.to_string()))?,
            fan_382: Vec::new(),
            fan_500: Vec::new(),
            fan_618: Vec::new(),
        })
    }

    fn append(&mut self, high: f64, low: f64) -> Option<(f64, f64, f64)> {
        let result = self.inner.append(high, low);
        let value = result.unwrap_or(FibonacciFanValue {
            fan_382: f64::NAN,
            fan_500: f64::NAN,
            fan_618: f64::NAN,
        });
        self.fan_382.push(value.fan_382);
        self.fan_500.push(value.fan_500);
        self.fan_618.push(value.fan_618);
        result.map(|value| (value.fan_382, value.fan_500, value.fan_618))
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
    ) {
        (
            PyArray1::from_vec(py, self.fan_382.clone()),
            PyArray1::from_vec(py, self.fan_500.clone()),
            PyArray1::from_vec(py, self.fan_618.clone()),
        )
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64, f64)> {
        self.inner
            .value()
            .map(|value| (value.fan_382, value.fan_500, value.fan_618))
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.fan_382.clear();
        self.fan_500.clear();
        self.fan_618.clear();
    }

    fn __len__(&self) -> usize {
        self.inner.len()
    }
}
