use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::FibonacciRetracement as FibonacciRetracementState;

/// Python boundary for the canonical Rust Fibonacci retracement state.
#[pyclass]
pub struct FibonacciRetracement {
    inner: FibonacciRetracementState,
    level_zero: Vec<f64>,
    level_twenty_three_point_six: Vec<f64>,
    level_thirty_eight_point_two: Vec<f64>,
    level_fifty: Vec<f64>,
    level_sixty_one_point_eight: Vec<f64>,
    level_seventy_eight_point_six: Vec<f64>,
    level_one_hundred: Vec<f64>,
}

#[pymethods]
impl FibonacciRetracement {
    #[new]
    #[pyo3(signature = (window=120))]
    fn new(window: usize) -> PyResult<Self> {
        Ok(Self {
            inner: FibonacciRetracementState::new(window)
                .map_err(|error| PyValueError::new_err(error.to_string()))?,
            level_zero: Vec::new(),
            level_twenty_three_point_six: Vec::new(),
            level_thirty_eight_point_two: Vec::new(),
            level_fifty: Vec::new(),
            level_sixty_one_point_eight: Vec::new(),
            level_seventy_eight_point_six: Vec::new(),
            level_one_hundred: Vec::new(),
        })
    }

    fn append(&mut self, close: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let value = self.inner.append(close);
        self.level_zero.push(value.level_zero);
        self.level_twenty_three_point_six
            .push(value.level_twenty_three_point_six);
        self.level_thirty_eight_point_two
            .push(value.level_thirty_eight_point_two);
        self.level_fifty.push(value.level_fifty);
        self.level_sixty_one_point_eight
            .push(value.level_sixty_one_point_eight);
        self.level_seventy_eight_point_six
            .push(value.level_seventy_eight_point_six);
        self.level_one_hundred.push(value.level_one_hundred);
        (
            value.level_zero,
            value.level_twenty_three_point_six,
            value.level_thirty_eight_point_two,
            value.level_fifty,
            value.level_sixty_one_point_eight,
            value.level_seventy_eight_point_six,
            value.level_one_hundred,
        )
    }

    fn extend(&mut self, py: Python<'_>, close: PyReadonlyArray1<f64>) -> PyResult<()> {
        let close = close.as_slice()?;
        let inner = &mut self.inner;
        let level_zero = &mut self.level_zero;
        let level_twenty_three_point_six = &mut self.level_twenty_three_point_six;
        let level_thirty_eight_point_two = &mut self.level_thirty_eight_point_two;
        let level_fifty = &mut self.level_fifty;
        let level_sixty_one_point_eight = &mut self.level_sixty_one_point_eight;
        let level_seventy_eight_point_six = &mut self.level_seventy_eight_point_six;
        let level_one_hundred = &mut self.level_one_hundred;
        py.allow_threads(|| {
            inner.extend_slice_into(
                close,
                level_zero,
                level_twenty_three_point_six,
                level_thirty_eight_point_two,
                level_fifty,
                level_sixty_one_point_eight,
                level_seventy_eight_point_six,
                level_one_hundred,
            );
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
            PyArray1::from_vec(py, self.level_zero.clone()),
            PyArray1::from_vec(py, self.level_twenty_three_point_six.clone()),
            PyArray1::from_vec(py, self.level_thirty_eight_point_two.clone()),
            PyArray1::from_vec(py, self.level_fifty.clone()),
            PyArray1::from_vec(py, self.level_sixty_one_point_eight.clone()),
            PyArray1::from_vec(py, self.level_seventy_eight_point_six.clone()),
            PyArray1::from_vec(py, self.level_one_hundred.clone()),
        )
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64, f64, f64, f64, f64, f64)> {
        self.inner.value().map(|value| {
            (
                value.level_zero,
                value.level_twenty_three_point_six,
                value.level_thirty_eight_point_two,
                value.level_fifty,
                value.level_sixty_one_point_eight,
                value.level_seventy_eight_point_six,
                value.level_one_hundred,
            )
        })
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.level_zero.clear();
        self.level_twenty_three_point_six.clear();
        self.level_thirty_eight_point_two.clear();
        self.level_fifty.clear();
        self.level_sixty_one_point_eight.clear();
        self.level_seventy_eight_point_six.clear();
        self.level_one_hundred.clear();
    }

    fn __len__(&self) -> usize {
        self.level_zero.len()
    }
}
