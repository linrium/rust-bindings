use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(Debug, Deserialize, Serialize)]
struct Ip {
    #[pyo3(get, set)]
    origin: String,
}

#[pyfunction]
fn rust_sleep(py: Python) -> PyResult<Bound<PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        let ip = reqwest::get("https://httpbin.org/ip")
            .await
            .unwrap()
            .json::<Ip>()
            .await
            .unwrap();

        Ok(ip)
    })
}

/// A Python module implemented in Rust.
#[pymodule]
fn python_sdk(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rust_sleep, m)?)?;
    Ok(())
}
