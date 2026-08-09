use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::indicators::{PremiumDiscount as State, PremiumDiscountValue};

#[pyclass]
pub struct PremiumDiscount {
    inner: State,
    zones: Vec<i32>,
    equilibrium: Vec<f64>,
}

#[pymethods]
impl PremiumDiscount {
    #[new]
    #[pyo3(signature = (window=20))]
    fn new(window: usize) -> PyResult<Self> {
        Ok(Self {
            inner: State::new(window)
                .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.to_string()))?,
            zones: Vec::new(),
            equilibrium: Vec::new(),
        })
    }

    fn append(&mut self, close: f64) -> (i32, f64) {
        let value = self.inner.append(close);
        self.zones.push(value.zone);
        self.equilibrium.push(value.equilibrium);
        (value.zone, value.equilibrium)
    }

    fn extend(&mut self, py: Python<'_>, close: PyReadonlyArray1<f64>) -> PyResult<()> {
        let close = close.as_slice()?;
        let (inner, zones, equilibrium) = (&mut self.inner, &mut self.zones, &mut self.equilibrium);
        py.allow_threads(|| inner.extend_slice_into(close, zones, equilibrium));
        Ok(())
    }

    fn compute<'py>(
        &self,
        py: Python<'py>,
    ) -> (Bound<'py, PyArray1<i32>>, Bound<'py, PyArray1<f64>>) {
        (
            PyArray1::from_vec(py, self.zones.clone()),
            PyArray1::from_vec(py, self.equilibrium.clone()),
        )
    }

    #[getter]
    fn value(&self) -> Option<(i32, f64)> {
        self.inner
            .value()
            .map(|value: PremiumDiscountValue| (value.zone, value.equilibrium))
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.zones.clear();
        self.equilibrium.clear();
    }
    fn __len__(&self) -> usize {
        self.zones.len()
    }
}
