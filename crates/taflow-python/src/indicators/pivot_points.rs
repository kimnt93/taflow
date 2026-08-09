use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::indicators::{PivotPoints as State, PivotPointsValue};

#[pyclass]
pub struct PivotPoints {
    inner: State,
    levels: [Vec<f64>; 5],
}

#[pymethods]
impl PivotPoints {
    #[new]
    fn new() -> Self {
        Self {
            inner: State::new(),
            levels: std::array::from_fn(|_| Vec::new()),
        }
    }
    fn append(
        &mut self,
        high: f64,
        low: f64,
        close: f64,
        anchor: bool,
    ) -> (f64, f64, f64, f64, f64) {
        let value = self.inner.append(high, low, close, anchor);
        let values = [
            value.pivot,
            value.resistance_one,
            value.support_one,
            value.support_two,
            value.resistance_two,
        ];
        for (index, item) in values.iter().enumerate() {
            self.levels[index].push(*item);
        }
        (values[0], values[1], values[2], values[3], values[4])
    }
    fn extend(
        &mut self,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
        anchor: PyReadonlyArray1<bool>,
    ) -> PyResult<()> {
        let (high, low, close, anchor) = (
            high.as_slice()?,
            low.as_slice()?,
            close.as_slice()?,
            anchor.as_slice()?,
        );
        self.inner
            .extend_slice_into(high, low, close, anchor, &mut self.levels)
            .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.to_string()))
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
            PyArray1::from_vec(py, self.levels[0].clone()),
            PyArray1::from_vec(py, self.levels[1].clone()),
            PyArray1::from_vec(py, self.levels[2].clone()),
            PyArray1::from_vec(py, self.levels[3].clone()),
            PyArray1::from_vec(py, self.levels[4].clone()),
        )
    }
    #[getter]
    fn value(&self) -> Option<(f64, f64, f64, f64, f64)> {
        self.inner.value().map(|value: PivotPointsValue| {
            (
                value.pivot,
                value.resistance_one,
                value.support_one,
                value.support_two,
                value.resistance_two,
            )
        })
    }
    fn reset(&mut self) {
        self.inner.reset();
        for level in &mut self.levels {
            level.clear();
        }
    }
    fn __len__(&self) -> usize {
        self.levels[0].len()
    }
}
