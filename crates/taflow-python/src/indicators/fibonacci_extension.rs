use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::indicators::{FibonacciExtension as State, FibonacciExtensionValue};

#[pyclass]
pub struct FibonacciExtension {
    inner: State,
    extension_1272: Vec<f64>,
    extension_1414: Vec<f64>,
    extension_1618: Vec<f64>,
    extension_2000: Vec<f64>,
    extension_2618: Vec<f64>,
}

#[pymethods]
impl FibonacciExtension {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            inner: State::new()
                .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.to_string()))?,
            extension_1272: Vec::new(),
            extension_1414: Vec::new(),
            extension_1618: Vec::new(),
            extension_2000: Vec::new(),
            extension_2618: Vec::new(),
        })
    }

    fn append(&mut self, high: f64, low: f64) -> Option<(f64, f64, f64, f64, f64)> {
        let result = self.inner.append(high, low);
        let value = result.unwrap_or(FibonacciExtensionValue {
            extension_1272: f64::NAN,
            extension_1414: f64::NAN,
            extension_1618: f64::NAN,
            extension_2000: f64::NAN,
            extension_2618: f64::NAN,
        });
        self.extension_1272.push(value.extension_1272);
        self.extension_1414.push(value.extension_1414);
        self.extension_1618.push(value.extension_1618);
        self.extension_2000.push(value.extension_2000);
        self.extension_2618.push(value.extension_2618);
        result.map(|value| {
            (
                value.extension_1272,
                value.extension_1414,
                value.extension_1618,
                value.extension_2000,
                value.extension_2618,
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
    ) {
        (
            PyArray1::from_vec(py, self.extension_1272.clone()),
            PyArray1::from_vec(py, self.extension_1414.clone()),
            PyArray1::from_vec(py, self.extension_1618.clone()),
            PyArray1::from_vec(py, self.extension_2000.clone()),
            PyArray1::from_vec(py, self.extension_2618.clone()),
        )
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64, f64, f64, f64)> {
        self.inner.value().map(|value| {
            (
                value.extension_1272,
                value.extension_1414,
                value.extension_1618,
                value.extension_2000,
                value.extension_2618,
            )
        })
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.extension_1272.clear();
        self.extension_1414.clear();
        self.extension_1618.clear();
        self.extension_2000.clear();
        self.extension_2618.clear();
    }

    fn __len__(&self) -> usize {
        self.inner.len()
    }
}
