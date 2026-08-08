use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::stream::{
    BarsSince, EntryExit, HighestSince, LowestSince, PositionHold, SignalDelay, ValueWhen,
};
macro_rules! one {
    ($name:ident,$inner:ty) => {
        #[pyclass]
        pub struct $name {
            inner: $inner,
            outputs: Vec<f64>,
        }
        #[pymethods]
        impl $name {
            #[new]
            fn new() -> Self {
                Self {
                    inner: <$inner>::new(),
                    outputs: Vec::new(),
                }
            }
            fn append(&mut self, condition: bool) -> Option<f64> {
                let v = self.inner.append(condition);
                self.outputs.push(v.unwrap_or(f64::NAN));
                v
            }
            fn extend(&mut self, py: Python<'_>, input: PyReadonlyArray1<bool>) -> PyResult<()> {
                let input = input.as_slice()?;
                py.allow_threads(|| {
                    for &v in input {
                        self.append(v);
                    }
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
        }
    };
}
one!(BarsSinceOperator, BarsSince);
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
        let v = self.inner.append(entry, exit);
        self.outputs.push(v);
        v
    }
    fn extend(
        &mut self,
        py: Python<'_>,
        entry: PyReadonlyArray1<bool>,
        exit: PyReadonlyArray1<bool>,
    ) -> PyResult<()> {
        let (e, x) = (entry.as_slice()?, exit.as_slice()?);
        if e.len() != x.len() {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "inputs must have equal lengths",
            ));
        }
        py.allow_threads(|| {
            for (&e, &x) in e.iter().zip(x) {
                self.append(e, x);
            }
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
}
#[pyclass]
pub struct PositionHoldOperator {
    inner: PositionHold,
    outputs: Vec<f64>,
}
#[pymethods]
impl PositionHoldOperator {
    #[new]
    fn new() -> Self {
        Self {
            inner: PositionHold::new(),
            outputs: Vec::new(),
        }
    }
    fn append(&mut self, input: f64) -> f64 {
        let v = self.inner.append(input);
        self.outputs.push(v);
        v
    }
    fn extend(&mut self, py: Python<'_>, input: PyReadonlyArray1<f64>) -> PyResult<()> {
        let input = input.as_slice()?;
        py.allow_threads(|| {
            for &v in input {
                self.append(v);
            }
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
}
macro_rules! two {
    ($name:ident,$inner:ty) => {
        #[pyclass]
        pub struct $name {
            inner: $inner,
            outputs: Vec<f64>,
        }
        #[pymethods]
        impl $name {
            #[new]
            fn new() -> Self {
                Self {
                    inner: <$inner>::new(),
                    outputs: Vec::new(),
                }
            }
            fn append(&mut self, condition: bool, input: f64) -> Option<f64> {
                let v = self.inner.append(condition, input);
                self.outputs.push(v.unwrap_or(f64::NAN));
                v
            }
            fn extend(
                &mut self,
                py: Python<'_>,
                condition: PyReadonlyArray1<bool>,
                input: PyReadonlyArray1<f64>,
            ) -> PyResult<()> {
                let (c, x) = (condition.as_slice()?, input.as_slice()?);
                if c.len() != x.len() {
                    return Err(pyo3::exceptions::PyValueError::new_err(
                        "inputs must have equal lengths",
                    ));
                }
                py.allow_threads(|| {
                    for (&c, &x) in c.iter().zip(x) {
                        self.append(c, x);
                    }
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
        }
    };
}
two!(ValueWhenOperator, ValueWhen);
two!(HighestSinceOperator, HighestSince);
two!(LowestSinceOperator, LowestSince);
#[pyclass]
pub struct SignalDelayOperator {
    inner: SignalDelay,
    outputs: Vec<f64>,
}
#[pymethods]
impl SignalDelayOperator {
    #[new]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: SignalDelay::new(timeperiod)
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?,
            outputs: Vec::new(),
        })
    }
    fn append(&mut self, input: f64) -> Option<f64> {
        let v = self.inner.append(input);
        self.outputs.push(v.unwrap_or(f64::NAN));
        v
    }
    fn extend(&mut self, py: Python<'_>, input: PyReadonlyArray1<f64>) -> PyResult<()> {
        let input = input.as_slice()?;
        py.allow_threads(|| {
            for &v in input {
                self.append(v);
            }
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
}
