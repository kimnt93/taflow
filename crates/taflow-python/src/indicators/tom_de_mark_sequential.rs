use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::indicators::{TomDeMarkSequential as State, TomDeMarkSequentialValue};

/// Python boundary for the canonical Rust DeMark setup state.
#[pyclass]
pub struct TomDeMarkSequential {
    inner: State,
    buy: Vec<i32>,
    sell: Vec<i32>,
}

#[pymethods]
impl TomDeMarkSequential {
    #[new]
    fn new() -> Self {
        Self {
            inner: State::new(),
            buy: Vec::new(),
            sell: Vec::new(),
        }
    }

    fn append(&mut self, close: f64) -> Option<(i32, i32)> {
        let value = self.inner.append(close);
        let output = self.inner.outputs();
        self.buy.push(output.buy);
        self.sell.push(output.sell);
        value.map(|value| (value.buy, value.sell))
    }

    fn extend(&mut self, close: PyReadonlyArray1<f64>) -> PyResult<()> {
        self.inner
            .extend_slice_into(close.as_slice()?, &mut self.buy, &mut self.sell);
        Ok(())
    }

    fn compute<'py>(
        &self,
        py: Python<'py>,
    ) -> (Bound<'py, PyArray1<i32>>, Bound<'py, PyArray1<i32>>) {
        (
            PyArray1::from_vec(py, self.buy.clone()),
            PyArray1::from_vec(py, self.sell.clone()),
        )
    }

    #[getter]
    fn value(&self) -> Option<(i32, i32)> {
        self.inner
            .value()
            .map(|value: TomDeMarkSequentialValue| (value.buy, value.sell))
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.buy.clear();
        self.sell.clear();
    }

    fn __len__(&self) -> usize {
        self.buy.len()
    }
}
