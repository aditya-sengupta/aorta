extern crate ndarray;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::types::IntoPyDict;
use numpy::{PyArray1,PyArray2};

#[pyfunction]
fn cwd() -> PyResult<String> {
    Python::with_gil(|py| {
        let locals = [("os", py.import("os")?)].into_py_dict(py);
        let code = "os.getcwd()";
        let cwd: String = py.eval(code, None, Some(&locals))?.extract()?;

        Ok(format!("Python says we are in {}", cwd))
    })
}

// Take in the name of an AO loop and call all the functions we'd need to call while closing the loop
#[pyfunction]
fn call_aoloop(name: String) -> PyResult<()> {
    Python::with_gil(|py| {
        println!("Trying to import {}", name);
        let aoloop = PyModule::import(py, &name)?;
        let img: &PyArray2<f64> = aoloop.getattr("getim")?.call1((0.001,))?.downcast::<PyArray2<f64>>()?;
        let zcoeffs: &PyArray1<f64> = aoloop.getattr("measure")?.call1((img,))?.downcast::<PyArray1<f64>>()?;
        let u: &PyArray1<f64> = aoloop.getattr("control")?.call1((zcoeffs,))?.downcast::<PyArray1<f64>>()?;
        let dmc: &PyArray2<f64> = aoloop.getattr("control_to_dmc")?.call1((u,))?.downcast::<PyArray2<f64>>()?;
        let _dmcaf: &PyArray2<f64> = aoloop.getattr("applydmc")?.call1((dmc,))?.downcast::<PyArray2<f64>>()?;
        Ok(())
    })
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn soar(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(cwd, m)?)?;
    m.add_function(wrap_pyfunction!(call_aoloop, m)?)?;
    Ok(())
}