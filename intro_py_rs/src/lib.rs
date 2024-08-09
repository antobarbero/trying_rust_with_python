use pyo3::prelude::*;
use pyo3::exceptions;

#[pyfunction]
fn hello_world(name: String) -> PyResult<String> {
    if name.is_empty() {
        Err(exceptions::PyValueError::new_err("name cannot be empty"))
    } else {
        let result: String = format!("Hello {} from rust", name).to_string();
        Ok(result)
    }
}

#[pyfunction]
fn fibonacci(n: u128) -> PyResult<u128> {
    Ok(fibonacci_r(n))
}

fn fibonacci_r(n: u128) -> u128 {
    if n == 0 { return 0 }
    else if n == 1 { return 1 }
    else {
        return fibonacci_r(n-1) + fibonacci_r(n-2)
    }
}

#[pymodule]
fn intro_py_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello_world, m)?)?;
    m.add_function(wrap_pyfunction!(fibonacci, m)?)?;
    Ok(())
}
