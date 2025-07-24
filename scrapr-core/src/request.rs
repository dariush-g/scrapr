use std::collections::HashMap;

/// Options for customizing an HTTP request.
///
/// Includes optional headers, cookies, and query parameters.
#[derive(Clone, Debug, Default)]
pub struct RequestOptions {
    /// Custom HTTP headers to send with the request.
    pub headers: HashMap<String, String>,

    /// Key-value pairs representing cookies.
    pub cookies: HashMap<String, String>,

    /// Query string parameters to include in the URL.
    pub query: HashMap<String, String>,
}

impl RequestOptions {
    /// Creates a new `RequestOptions` instance from optional headers, cookies, and query params.
    ///
    /// # Arguments
    ///
    /// * `headers` - Optional map of header names to values.
    /// * `cookies` - Optional map of cookie names to values.
    /// * `query` - Optional map of query parameter names to values.
    pub fn new(
        headers: Option<HashMap<String, String>>,
        cookies: Option<HashMap<String, String>>,
        query: Option<HashMap<String, String>>,
    ) -> Self {
        RequestOptions {
            headers: headers.unwrap_or_default(),
            cookies: cookies.unwrap_or_default(),
            query: query.unwrap_or_default(),
        }
    }
}

/// Constructs a URL string with an optional base, path, and query parameters.
///
/// # Arguments
///
/// * `base` - Optional base URL prefix (can be empty).
/// * `path` - The resource path (e.g., `/index.html`).
/// * `query` - A map of query parameters to append to the URL.
///
/// # Returns
///
/// A full URL with query string if applicable.
pub fn build_url(base: &str, path: &str, query: &HashMap<String, String>) -> String {
    let mut url = format!("{base}{path}");
    if !query.is_empty() {
        let query_string = query
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&");
        url.push('?');
        url.push_str(&query_string);
    }
    url
}

/// Formats HTTP headers into a request-ready string.
///
/// # Arguments
///
/// * `host` - The host name to include in the `Host` header.
/// * `options` - The `RequestOptions` containing custom headers and cookies.
///
/// # Returns
///
/// A formatted string of HTTP headers.
pub fn format_headers(host: &str, options: &RequestOptions) -> String {
    let mut headers = vec![
        format!("Host: {host}"),
        "Connection: close".to_string(),
        "User-Agent: Scraper/0.1".to_string(),
    ];

    for (k, v) in &options.headers {
        headers.push(format!("{k}: {v}"));
    }

    if !options.cookies.is_empty() {
        // Cookies should be formatted as `key=value` pairs separated by "; "
        let cookie_string = options
            .cookies
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join("; ");
        headers.push(format!("Cookie: {cookie_string}"));
    }

    headers.join("\r\n")
}
