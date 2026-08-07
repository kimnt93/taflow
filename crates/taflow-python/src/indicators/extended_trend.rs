use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::{
    ArnaudLegouxMovingAverage, HullMovingAverage, VolumeWeightedMovingAverage,
    ZeroLagExponentialMovingAverage,
};

macro_rules! unary {
    ($name:ident, $inner:ty) => {
        #[pyclass]
        pub struct $name {
            inner: $inner,
            outputs: Vec<f64>,
        }
        #[pymethods]
        impl $name {
            #[new]
            fn new(timeperiod: usize) -> PyResult<Self> {
                Ok(Self {
                    inner: <$inner>::new(timeperiod)
                        .map_err(|e| PyValueError::new_err(e.to_string()))?,
                    outputs: Vec::new(),
                })
            }
            fn append(&mut self, input: f64) -> Option<f64> {
                let v = self.inner.append(input);
                self.outputs.push(v.unwrap_or(f64::NAN));
                v
            }
            fn extend(&mut self, input: PyReadonlyArray1<f64>) -> PyResult<()> {
                for &v in input.as_slice()? {
                    self.append(v);
                }
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
unary!(HmaOperator, HullMovingAverage);
unary!(ZlemaOperator, ZeroLagExponentialMovingAverage);

#[pyclass]
pub struct VwmaOperator {
    inner: VolumeWeightedMovingAverage,
    outputs: Vec<f64>,
}
#[pymethods]
impl VwmaOperator {
    #[new]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: VolumeWeightedMovingAverage::new(timeperiod)
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
            outputs: Vec::new(),
        })
    }
    fn append(&mut self, price: f64, volume: f64) -> Option<f64> {
        let v = self.inner.append(price, volume);
        self.outputs.push(v.unwrap_or(f64::NAN));
        v
    }
    fn extend(
        &mut self,
        price: PyReadonlyArray1<f64>,
        volume: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (p, v) = (price.as_slice()?, volume.as_slice()?);
        if p.len() != v.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        for (&p, &v) in p.iter().zip(v) {
            self.append(p, v);
        }
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
pub struct AlmaOperator {
    inner: ArnaudLegouxMovingAverage,
    outputs: Vec<f64>,
}
#[pymethods]
impl AlmaOperator {
    #[new]
    fn new(timeperiod: usize, offset: f64, sigma: f64) -> PyResult<Self> {
        Ok(Self {
            inner: ArnaudLegouxMovingAverage::new(timeperiod, offset, sigma)
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
            outputs: Vec::new(),
        })
    }
    fn append(&mut self, input: f64) -> Option<f64> {
        let v = self.inner.append(input);
        self.outputs.push(v.unwrap_or(f64::NAN));
        v
    }
    fn extend(&mut self, input: PyReadonlyArray1<f64>) -> PyResult<()> {
        for &v in input.as_slice()? {
            self.append(v);
        }
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
