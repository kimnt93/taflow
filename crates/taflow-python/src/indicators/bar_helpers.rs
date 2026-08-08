use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::stream::{GapDown, GapUp, HigherHigh, InsideBar, LowerLow, OutsideBar};
macro_rules! bar {
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
            fn append(&mut self, high: f64, low: f64) -> Option<f64> {
                let v = self.inner.append(high, low);
                self.outputs.push(v.unwrap_or(f64::NAN));
                v
            }
            fn extend(
                &mut self,
                py: Python<'_>,
                high: PyReadonlyArray1<f64>,
                low: PyReadonlyArray1<f64>,
            ) -> PyResult<()> {
                let high = high.as_slice()?;
                let low = low.as_slice()?;
                py.allow_threads(|| {
                    for (&h, &l) in high.iter().zip(low) {
                        self.append(h, l);
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
bar!(HigherHighOperator, HigherHigh);
bar!(LowerLowOperator, LowerLow);
bar!(InsideBarOperator, InsideBar);
bar!(OutsideBarOperator, OutsideBar);
bar!(GapUpOperator, GapUp);
bar!(GapDownOperator, GapDown);
