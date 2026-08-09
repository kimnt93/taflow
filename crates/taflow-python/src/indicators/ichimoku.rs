use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::indicators::Ichimoku;

#[pyclass]
pub struct IchimokuOperator {
    inner: Ichimoku,
    tenkan_sen: Vec<f64>,
    kijun_sen: Vec<f64>,
    span_a: Vec<f64>,
    span_b: Vec<f64>,
    chikou_span: Vec<f64>,
}

#[pymethods]
impl IchimokuOperator {
    #[new]
    #[pyo3(signature = (tenkan=9, kijun=26, senkou=52))]
    fn new(tenkan: usize, kijun: usize, senkou: usize) -> PyResult<Self> {
        Ok(Self {
            inner: Ichimoku::new(tenkan, kijun, senkou)
                .map_err(|error| PyValueError::new_err(error.to_string()))?,
            tenkan_sen: Vec::new(),
            kijun_sen: Vec::new(),
            span_a: Vec::new(),
            span_b: Vec::new(),
            chikou_span: Vec::new(),
        })
    }

    fn append(&mut self, high: f64, low: f64, close: f64) -> (f64, f64, f64, f64, f64) {
        let value = self.inner.append(high, low, close);
        self.tenkan_sen.push(value.tenkan_sen);
        self.kijun_sen.push(value.kijun_sen);
        self.span_a.push(value.span_a);
        self.span_b.push(value.span_b);
        self.chikou_span.push(value.chikou_span);
        (
            value.tenkan_sen,
            value.kijun_sen,
            value.span_a,
            value.span_b,
            value.chikou_span,
        )
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (high, low, close) = (high.as_slice()?, low.as_slice()?, close.as_slice()?);
        if high.len() != low.len() || high.len() != close.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        py.allow_threads(|| {
            for ((&high, &low), &close) in high.iter().zip(low).zip(close) {
                self.append(high, low, close);
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
            PyArray1::from_vec(py, self.tenkan_sen.clone()),
            PyArray1::from_vec(py, self.kijun_sen.clone()),
            PyArray1::from_vec(py, self.span_a.clone()),
            PyArray1::from_vec(py, self.span_b.clone()),
            PyArray1::from_vec(py, self.chikou_span.clone()),
        )
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64, f64, f64, f64)> {
        self.inner.value().map(|value| {
            (
                value.tenkan_sen,
                value.kijun_sen,
                value.span_a,
                value.span_b,
                value.chikou_span,
            )
        })
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.tenkan_sen.clear();
        self.kijun_sen.clear();
        self.span_a.clear();
        self.span_b.clear();
        self.chikou_span.clear();
    }
}
