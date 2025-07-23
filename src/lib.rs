pub mod http;
pub mod secure;

use pyo3::prelude::*;

use crate::http::{extract_attribute_, extract_links_, extract_tag_, fetch_http_};

#[pymodule]
fn scrapr(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fetch_http, m)?)?;
    m.add_function(wrap_pyfunction!(extract_tag, m)?)?;
    m.add_function(wrap_pyfunction!(extract_attribute, m)?)?;
    m.add_function(wrap_pyfunction!(extract_links, m)?)?;

    Ok(())
}

#[pyfunction]
pub fn fetch_http(host: &str, path: &str) -> PyResult<String> {
    fetch_http_(host, path)
}

#[pyfunction]
pub fn extract_tag(text: &str, tag: &str) -> PyResult<Vec<String>> {
    extract_tag_(text, tag)
}

#[pyfunction]
pub fn extract_attribute(text: &str, tag: &str, attr: &str) -> PyResult<Vec<String>> {
    extract_attribute_(text, tag, attr)
}

#[pyfunction]
pub fn extract_links(text: &str) -> PyResult<Vec<String>> {
    extract_links_(text)
}
