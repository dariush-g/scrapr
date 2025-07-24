use native_tls::TlsConnector;
use std::io::{Read, Write};
use std::net::TcpStream;

use crate::request::{RequestOptions, build_url, format_headers};

/// Represents a parsed URL with its components.
pub struct ParsedUrl<'a> {
    pub scheme: &'a str,
    pub host: &'a str,
    pub port: u16,
    pub path: String,
}

impl<'a> ParsedUrl<'a> {
    /// Parses a URL string into its components: scheme, host, port, and path.
    pub fn parse_url(url: &'a str) -> Result<ParsedUrl<'a>, String> {
        let scheme_split: Vec<&str> = url.splitn(2, "://").collect();

        if scheme_split.len() != 2 {
            return Err("URL missing scheme".to_string());
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
                _ => return Err(format!("Unknown scheme: {}", scheme)),
            },
        };

        Ok(ParsedUrl {
            scheme,
            host,
            port,
            path: if path.is_empty() {
                "/".to_string()
            } else {
                format!("/{}", path)
            },
        })
    }
}

/// Fetches the raw response body of a GET request to the given URL.
/// Supports HTTP and HTTPS.
pub fn fetch_url(url: &str) -> Result<String, String> {
    let parsed = ParsedUrl::parse_url(url)?;

    let request = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        parsed.path, parsed.host
    );

    let mut response = String::new();

    if parsed.scheme == "https" {
        let connector = TlsConnector::new().map_err(|e| format!("TLS error: {}", e))?;
        let stream = TcpStream::connect((parsed.host, parsed.port))
            .map_err(|e| format!("Connection error: {}", e))?;
        let mut tls_stream = connector
            .connect(parsed.host, stream)
            .map_err(|e| format!("TLS connection error: {}", e))?;

        tls_stream
            .write_all(request.as_bytes())
            .map_err(|e| format!("Write error: {}", e))?;
        tls_stream
            .read_to_string(&mut response)
            .map_err(|e| format!("Read error: {}", e))?;
    } else {
        let mut stream = TcpStream::connect((parsed.host, parsed.port))
            .map_err(|e| format!("Connection error: {}", e))?;
        stream
            .write_all(request.as_bytes())
            .map_err(|e| format!("Write error: {}", e))?;
        stream
            .read_to_string(&mut response)
            .map_err(|e| format!("Read error: {}", e))?;
    }

    if let Some(body) = response.split("\r\n\r\n").nth(1) {
        Ok(body.to_string())
    } else {
        Err("No body found in response".to_string())
    }
}

/// Fetches the response body of a GET request with additional request options (headers, cookies, query).
///
/// # Arguments
/// - `url`: The base URL to request.
/// - `opts`: Additional request options.
///
/// # Returns
/// Raw body of the HTTP response, or an error string.
pub fn fetch_url_with_options(url: &str, opts: RequestOptions) -> Result<String, String> {
    let parsed = ParsedUrl::parse_url(url)?;

    let full_url = build_url("", &parsed.path, &opts.query);
    let headers = format_headers(parsed.host, &opts);
    let request = format!("GET {} HTTP/1.1\r\n{}\r\n\r\n", full_url, headers);

    let mut response = String::new();

    match parsed.scheme {
        "https" => {
            let connector = TlsConnector::new().map_err(|e| format!("TLS error: {}", e))?;
            let stream = TcpStream::connect((parsed.host, parsed.port))
                .map_err(|e| format!("Connection error: {}", e))?;
            let mut tls_stream = connector
                .connect(parsed.host, stream)
                .map_err(|e| format!("TLS connection error: {}", e))?;

            tls_stream
                .write_all(request.as_bytes())
                .map_err(|e| format!("Write error: {}", e))?;
            tls_stream
                .read_to_string(&mut response)
                .map_err(|e| format!("Read error: {}", e))?;
        }
        "http" => {
            let mut stream = TcpStream::connect((parsed.host, parsed.port))
                .map_err(|e| format!("Connection error: {}", e))?;
            stream
                .write_all(request.as_bytes())
                .map_err(|e| format!("Write error: {}", e))?;
            stream
                .read_to_string(&mut response)
                .map_err(|e| format!("Read error: {}", e))?;
        }
        _ => return Err(format!("Unsupported scheme: {}", parsed.scheme)),
    }

    if let Some(body) = response.split("\r\n\r\n").nth(1) {
        Ok(body.to_string())
    } else {
        Err("No body found in response".to_string())
    }
}
