use std::{
    io::{Read, Write},
    net::TcpStream,
};

use pyo3::prelude::*;

pub fn fetch_http_base(host: &str, path: &str) -> PyResult<String> {
    let mut stream = TcpStream::connect((host, 80))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyException, _>(e.to_string()))?;

    let request = format!("GET {path} HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\n\r\n");

    stream.write_all(request.as_bytes()).unwrap();

    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();

    if let Some(body) = response.split("\r\n\r\n").nth(1) {
        // print!("{body}");

        Ok(body.to_string())
    } else {
        Err(PyErr::new::<pyo3::exceptions::PyException, _>(
            "No body found",
        ))
    }
}

#[pyfunction]
pub fn fetch_http(host: &str, path: &str) -> PyResult<String> {
    let mut stream = TcpStream::connect((host, 80))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyException, _>(e.to_string()))?;

    let request = format!("GET {path} HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\n\r\n");

    stream.write_all(request.as_bytes()).unwrap();

    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();

    if let Some(body) = response.split("\r\n\r\n").nth(1) {
        Ok(body.to_string())
    } else {
        Err(PyErr::new::<pyo3::exceptions::PyException, _>(
            "No body found",
        ))
    }
}

#[pyfunction]
pub fn extract_tag(text: &str, tag: &str) -> PyResult<Vec<String>> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let mut results = Vec::new();

    let mut start = 0;
    while let Some(open_idx) = text[start..].find(&open) {
        let start_idx = start + open_idx + open.len();
        if let Some(close_idx) = text[start_idx..].find(&close) {
            let end_idx = start_idx + close_idx;
            results.push(text[start_idx..end_idx].trim().to_string());
            start = end_idx + close.len();
        } else {
            break;
        }
    }

    Ok(results)
}

#[pyfunction]
pub fn extract_tag_base(text: &str, tag: &str) -> PyResult<Vec<String>> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let mut results = Vec::new();

    let mut start = 0;
    while let Some(open_idx) = text[start..].find(&open) {
        let start_idx = start + open_idx + open.len();
        if let Some(close_idx) = text[start_idx..].find(&close) {
            let end_idx = start_idx + close_idx;
            results.push(text[start_idx..end_idx].trim().to_string());
            start = end_idx + close.len();
        } else {
            break;
        }
    }

    Ok(results)
}

#[pymodule]
fn scrapr(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fetch_http, m)?)?;
    m.add_function(wrap_pyfunction!(extract_tag, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_request_base() {
        let body = fetch_http("example.com", "/").unwrap();
        assert!(body.contains("Example Domain"));
    }

    #[test]
    fn test_extract_tag_base() {
        let html = "<p>Hello</p><div>skip</div><p>World</p>";
        let result = extract_tag(html, "p").unwrap();
        assert_eq!(result, vec!["Hello", "World"]);
    }
}
