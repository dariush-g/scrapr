pub mod protocols;
pub mod request;
pub mod url;

use pyo3::prelude::*;

use crate::protocols::*;
use crate::request::*;
use crate::url::*;

#[pymodule]
fn scrapr(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fetch_http, m)?)?;
    m.add_function(wrap_pyfunction!(extract_tag, m)?)?;
    m.add_function(wrap_pyfunction!(extract_attribute, m)?)?;
    m.add_function(wrap_pyfunction!(extract_links, m)?)?;
    m.add_function(wrap_pyfunction!(fetch_https, m)?)?;
    m.add_function(wrap_pyfunction!(fetch_url, m)?)?;
    m.add_function(wrap_pyfunction!(fetch_http_with_options, m)?)?;
    m.add_function(wrap_pyfunction!(fetch_https_with_options, m)?)?;
    m.add_function(wrap_pyfunction!(fetch_url_with_options, m)?)?;

    m.add_class::<RequestOptions>()?;

    Ok(())
}
