use pyo3::exceptions::PyException;
use pyo3::{PyErr, PyResult, pyfunction};

use native_tls::TlsConnector;
use std::io::{Read, Write};
use std::net::TcpStream;

use crate::request::{RequestOptions, build_url, format_headers};

// use crate::protocols::{fetch_http_, fetch_https_};

pub struct ParsedUrl<'a> {
    scheme: &'a str,
    host: &'a str,
    port: u16,
    path: String,
}

impl ParsedUrl<'_> {
    fn parse_url(url: &'_ str) -> Result<ParsedUrl<'_>, String> {
        let scheme_split: Vec<&str> = url.splitn(2, "://").collect();

        if scheme_split.len() != 2 {
            return Err("Url missing scheme".to_string());
        }

        let scheme = scheme_split[0];
        let rest = scheme_split[1];

        let mut host_and_path = rest.splitn(2, '/');

        let host_port = host_and_path.next().unwrap_or("");
        let path = host_and_path.next().unwrap_or("");

        let mut host_split = host_port.splitn(2, ':');
        let host = host_split.next().unwrap();
        let port = match host_split.next() {
            Some(port_str) => port_str
                .parse::<u16>()
                .map_err(|_| "Invalid port".to_string())?,
            None => match scheme {
                "http" => 80,
                "https" => 443,
                _ => return Err(format!("Unknown scheme: {scheme}")),
            },
        };

        Ok(ParsedUrl {
            scheme,
            host,
            port,
            path: if path.is_empty() {
                "/".to_string()
            } else {
                format!("/{path}")
            },
        })
    }
}

// pub fn fetch_url_(url: &str) -> PyResult<String> {
//     if let Ok(parsed) = parse_url(url) {
//         match parsed.scheme {
//             "http" => return fetch_http_(parsed.host, &parsed.path),
//             "https" => return fetch_https_(parsed.host, &parsed.path),
//             _ => {
//                 return Err(PyErr::new::<pyo3::exceptions::PyException, _>(
//                     "Error fetching url from host | Error with port",
//                 ));
//             }
//         }
//     }

//     Err(PyErr::new::<pyo3::exceptions::PyException, _>(
//         "Error fetching url from host",
//     ))
// }

#[pyfunction]
pub fn fetch_url(url: &str) -> PyResult<String> {
    let parsed =
        ParsedUrl::parse_url(url).map_err(|e| PyErr::new::<pyo3::exceptions::PyException, _>(e))?;

    let request = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        parsed.path, parsed.host
    );

    let mut response = String::new();

    if parsed.scheme == "https" {
        let connector = TlsConnector::new()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyException, _>(e.to_string()))?;

        let stream = TcpStream::connect((parsed.host, parsed.port))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyException, _>(e.to_string()))?;

        let mut tls_stream = connector
            .connect(parsed.host, stream)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyException, _>(e.to_string()))?;

        tls_stream.write_all(request.as_bytes()).unwrap();
        tls_stream.read_to_string(&mut response).unwrap();
    } else if parsed.scheme == "http" {
        let mut stream = TcpStream::connect((parsed.host, parsed.port))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyException, _>(e.to_string()))?;

        stream.write_all(request.as_bytes()).unwrap();
        stream.read_to_string(&mut response).unwrap();
    } else {
        return Err(PyErr::new::<pyo3::exceptions::PyException, _>(format!(
            "Unsupported scheme: {}",
            parsed.scheme
        )));
    }

    if let Some(body) = response.split("\r\n\r\n").nth(1) {
        Ok(body.to_string())
    } else {
        Err(PyErr::new::<pyo3::exceptions::PyException, _>(
            "No body found",
        ))
    }
}

#[pyfunction]
pub fn fetch_url_with_options(url: &str, opts: RequestOptions) -> PyResult<String> {
    let parsed = ParsedUrl::parse_url(url).map_err(|e| PyErr::new::<PyException, _>(e))?;

    let full_url = build_url("", &parsed.path, &opts.query);

    let headers = format_headers(&parsed.host, &opts);
    let request = format!("GET {full_url} HTTP/1.1\r\n{headers}\r\n\r\n");

    let mut response = String::new();

    match parsed.scheme {
        "https" => {
            let connector =
                TlsConnector::new().map_err(|e| PyErr::new::<PyException, _>(e.to_string()))?;

            let stream = TcpStream::connect((parsed.host, parsed.port))
                .map_err(|e| PyErr::new::<PyException, _>(e.to_string()))?;

            let mut tls_stream = connector
                .connect(parsed.host, stream)
                .map_err(|e| PyErr::new::<PyException, _>(e.to_string()))?;

            tls_stream.write_all(request.as_bytes()).unwrap();
            tls_stream.read_to_string(&mut response).unwrap();
        }
        "http" => {
            let mut stream = TcpStream::connect((parsed.host, parsed.port))
                .map_err(|e| PyErr::new::<PyException, _>(e.to_string()))?;

            stream.write_all(request.as_bytes()).unwrap();
            stream.read_to_string(&mut response).unwrap();
        }
        _ => {
            return Err(PyErr::new::<PyException, _>(format!(
                "Unsupported scheme: {}",
                parsed.scheme
            )));
        }
    }

    if let Some(body) = response.split("\r\n\r\n").nth(1) {
        Ok(body.to_string())
    } else {
        Err(PyErr::new::<PyException, _>("No body found"))
    }
}
