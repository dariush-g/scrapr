use crate::request::*;
use anyhow::{Result, anyhow};
use native_tls::TlsConnector;
use std::{
    io::{Read, Write},
    net::TcpStream,
};

pub fn fetch_http(host: &str, path: &str) -> Result<String> {
    let mut stream =
        TcpStream::connect((host, 80)).map_err(|e| anyhow!("Failed to connect: {e}"))?;

    let request = format!("GET {path} HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\n\r\n");
    stream.write_all(request.as_bytes())?;

    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    response
        .split("\r\n\r\n")
        .nth(1)
        .map(|body| body.to_string())
        .ok_or_else(|| anyhow!("No body found"))
}

pub fn fetch_http_with_options(host: &str, path: &str, options: RequestOptions) -> Result<String> {
    let mut stream = TcpStream::connect((host, 80))?;

    let url = build_url("", path, &options.query);
    let headers = format_headers(host, &options);
    let request = format!("GET {} HTTP/1.1\r\n{}\r\n\r\n", url, headers);

    stream.write_all(request.as_bytes())?;

    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    response
        .split("\r\n\r\n")
        .nth(1)
        .map(|body| body.to_string())
        .ok_or_else(|| anyhow!("No body found"))
}

pub fn extract_attribute(text: &str, tag: &str, attr: &str) -> Result<Vec<String>> {
    let mut results = Vec::new();
    let open_tag = format!("<{tag}");
    let attr_eq = format!("{attr}=\"");

    let mut start = 0;
    while let Some(tag_start) = text[start..].find(&open_tag) {
        let pos = start + tag_start;
        if let Some(attr_start) = text[pos..].find(&attr_eq) {
            let val_start = pos + attr_start + attr_eq.len();
            if let Some(val_end) = text[val_start..].find('"') {
                results.push(text[val_start..val_start + val_end].to_string());
                start = val_start + val_end;
            } else {
                break;
            }
        } else {
            start = pos + open_tag.len();
        }
    }

    Ok(results)
}

pub fn extract_links(text: &str) -> Result<Vec<String>> {
    extract_attribute(text, "a", "href")
}

pub fn extract_tag(text: &str, tag: &str) -> Result<Vec<String>> {
    let mut results = Vec::new();
    let tag_lower = tag.to_lowercase();

    let mut start = 0;
    while let Some(open_pos) = text[start..].find('<') {
        let open_tag_start = start + open_pos;
        if let Some(open_tag_end) = text[open_tag_start..].find('>') {
            let tag_content_start = open_tag_start + open_tag_end + 1;

            let open_tag_full = &text[open_tag_start + 1..open_tag_start + open_tag_end];
            let tag_name = open_tag_full
                .split_whitespace()
                .next()
                .unwrap_or("")
                .to_lowercase();

            if tag_name == tag_lower {
                let close_tag = format!("</{tag}>", tag = tag);
                if let Some(close_pos) = text[tag_content_start..].to_lowercase().find(&close_tag) {
                    let content = &text[tag_content_start..tag_content_start + close_pos];
                    results.push(content.trim().to_string());
                    start = tag_content_start + close_pos + close_tag.len();
                    continue;
                }
            }

            start = open_tag_start + 1;
        } else {
            break;
        }
    }

    Ok(results)
}

pub fn fetch_https(host: &str, path: &str) -> Result<String> {
    let tcp = TcpStream::connect((host, 443)).map_err(|e| anyhow!("TLS connect failed: {e}"))?;

    let connector = TlsConnector::new()?;
    let mut stream = connector.connect(host, tcp)?;

    let request = format!("GET {path} HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\n\r\n");
    stream.write_all(request.as_bytes())?;

    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    response
        .split("\r\n\r\n")
        .nth(1)
        .map(|body| body.to_string())
        .ok_or_else(|| anyhow!("No body found"))
}

pub fn fetch_https_with_options(host: &str, path: &str, options: RequestOptions) -> Result<String> {
    let tcp = TcpStream::connect((host, 443))?;
    let connector = TlsConnector::new()?;
    let mut stream = connector.connect(host, tcp)?;

    let url = build_url("", path, &options.query);
    let headers = format_headers(host, &options);
    let request = format!("GET {} HTTP/1.1\r\n{}\r\n\r\n", url, headers);

    stream.write_all(request.as_bytes())?;

    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    response
        .split("\r\n\r\n")
        .nth(1)
        .map(|body| body.to_string())
        .ok_or_else(|| anyhow!("No body found"))
}

// pub fn fetch_http_(host: &str, path: &str) -> PyResult<String> {
//     let mut stream = TcpStream::connect((host, 80))
//         .map_err(|e| PyErr::new::<pyo3::exceptions::PyException, _>(e.to_string()))?;

//     stream.set_read_timeout(Some(std::time::Duration::from_secs(5)))
//         .map_err(|e| PyErr::new::<pyo3::exceptions::PyException, _>(e.to_string()))?;
//     stream.set_write_timeout(Some(std::time::Duration::from_secs(5)))
//         .map_err(|e| PyErr::new::<pyo3::exceptions::PyException, _>(e.to_string()))?;

//     let request = format!("GET {path} HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\n\r\n");

//     stream.write_all(request.as_bytes())
//         .map_err(|e| PyErr::new::<pyo3::exceptions::PyException, _>(e.to_string()))?;

//     let mut response = String::new();
//     stream.read_to_string(&mut response)
//         .map_err(|e| PyErr::new::<pyo3::exceptions::PyException, _>(e.to_string()))?;

//     // Check HTTP status
//     let status_line = response.lines().next().unwrap_or_default();
//     if !status_line.contains("200 OK") {
//         return Err(PyErr::new::<pyo3::exceptions::PyException, _>(format!("HTTP error: {}", status_line)));
//     }

//     if let Some(body) = response.split("\r\n\r\n").nth(1) {
//         Ok(body.to_string())
//     } else {
//         Err(PyErr::new::<pyo3::exceptions::PyException, _>("No body found"))
//     }
// }
