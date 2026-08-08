use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Converts a Rust `Vec<f64>` into a Python NumPy array.
pub fn to_py_array(py: Python<'_>, data: Vec<f64>) -> Py<PyArray1<f64>> {
    PyArray1::from_vec(py, data).into()
}

/// Converts a Rust `Vec<i32>` into a Python NumPy array for candlestick data.
pub fn to_py_array_i32(py: Python<'_>, data: Vec<i32>) -> Py<PyArray1<i32>> {
    PyArray1::from_vec(py, data).into()
}
