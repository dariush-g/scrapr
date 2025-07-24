use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct RequestOptions {
    pub headers: HashMap<String, String>,
    pub cookies: HashMap<String, String>,
    pub query: HashMap<String, String>,
}

impl RequestOptions {
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
