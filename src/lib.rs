extern crate ndarray;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::types::IntoPyDict;
use numpy::{PyArray1,PyArray2};
use std::time::{Duration, Instant};
use std::thread::sleep;

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

#[pyfunction]
fn record_images(name: String, duration: f64, dt: f64) -> Vec<u128> {
    let nsteps = (duration / dt).ceil() as usize;
    let mut times = vec![0; nsteps];
    let duration_interval = Duration::from_millis((duration * 1000.0) as u64);

    Python::with_gil(|py| {
        let _aoloop = PyModule::import(py, &name);
        let mut i = 0;
        let tstart = Instant::now();
        while tstart.elapsed() <= duration_interval {
            //let mut img: &PyArray2<f64> = aoloop.getattr("getim").call1((0.001,)).downcast::<PyArray2<f64>>();
            times[i] = tstart.elapsed().as_micros();
            sleep(Duration::from_millis(10));
            i += 1;
        }
    println!("{}", i);
    println!("{:?}", times);
    return times
    })
}

#[pymodule]
fn soar(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(cwd, m)?)?;
    m.add_function(wrap_pyfunction!(call_aoloop, m)?)?;
    m.add_function(wrap_pyfunction!(record_images, m)?)?;
    Ok(())
}