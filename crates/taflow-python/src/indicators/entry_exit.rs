use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::stream::EntryExit;

#[pyclass]
pub struct EntryExitOperator {
    inner: EntryExit,
    outputs: Vec<f64>,
}

#[pymethods]
impl EntryExitOperator {
    #[new]
    fn new() -> Self {
        Self {
            inner: EntryExit::new(),
            outputs: Vec::new(),
        }
    }
    fn append(&mut self, entry: bool, exit: bool) -> f64 {
        let value = self.inner.append(entry, exit);
        self.outputs.push(value);
        value
    }
    fn extend(
        &mut self,
        py: Python<'_>,
        entry: PyReadonlyArray1<bool>,
        exit: PyReadonlyArray1<bool>,
    ) -> PyResult<()> {
        let (entry, exit) = (entry.as_slice()?, exit.as_slice()?);
        if entry.len() != exit.len() {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "inputs must have equal lengths",
            ));
        }
        py.allow_threads(|| {
            entry.iter().zip(exit).for_each(|(&e, &x)| {
                self.append(e, x);
            })
        });
        Ok(())
    }
    fn compute<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        PyArray1::from_vec(py, self.outputs.clone())
    }
    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }
    fn reset(&mut self) {
        self.inner.reset();
        self.outputs.clear();
    }
    fn __len__(&self) -> usize {
        self.outputs.len()
    }
}
