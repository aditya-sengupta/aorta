use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::types::PyDict;
use pyo3::types::IntoPyDict;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn cwd() -> PyResult<String> {
    Python::with_gil(|py| {
        let locals = [("os", py.import("os")?)].into_py_dict(py);
        let code = "os.getcwd()";
        let cwd: String = py.eval(code, None, Some(&locals))?.extract()?;

        Ok(format!("Python says we are in {}", cwd))
    })
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn soar(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(cwd, m)?)?;
    Ok(())
}