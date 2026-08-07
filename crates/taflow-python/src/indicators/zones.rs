use numpy::PyArray1;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::ActiveZoneList;

#[pyclass]
pub struct ActiveZoneListOperator {
    inner: ActiveZoneList,
}

#[pymethods]
impl ActiveZoneListOperator {
    #[new]
    fn new(capacity: usize) -> PyResult<Self> {
        Ok(Self { inner: ActiveZoneList::new(capacity).map_err(|error| PyValueError::new_err(error.to_string()))? })
    }

    fn add(&mut self, top: f64, bottom: f64, flags: u32) -> usize {
        self.inner.add(top, bottom, flags)
    }

    #[pyo3(signature = (price, max_age=None))]
    fn advance<'py>(
        &mut self,
        py: Python<'py>,
        price: f64,
        max_age: Option<usize>,
    ) -> Bound<'py, PyArray1<bool>> {
        PyArray1::from_vec(py, self.inner.advance(price, max_age))
    }

    #[getter]
    fn size(&self) -> usize { self.inner.zones().len() }

    fn reset(&mut self) { self.inner.reset(); }
}
