# scrapr-rs

**scrapr-rs** is a library for scraping HTML from the web.

---

## Functions

- **Fetch** HTML from an HTTP or HTTPS site
- **Extract** specific tags (`<ul>`, `<li>`, `<div>`, etc.)
- **Requests** with headers, cookies, and query strings

---

## Examples

```rust
use scrapr::{fetch_url, fetch_url_with_options, RequestOptions, extract_tag};

fn main() {
    // Basic GET request
    let html = fetch_url("http://localhost:8000/demo.html");
    println!("Raw HTML:\n{}", html);

    // Extract the first <ul> tag and its contents
    let tag = extract_tag(&html, "ul");
    println!("First <ul> tag:\n{}", tag);

    // Custom headers, cookies, and query parameters
    let opts = RequestOptions {
        headers: Some({
            let mut h = std::collections::HashMap::new();
            h.insert("User-Agent".into(), "scrapr/0.1".into());
            h
        }),
        cookies: Some({
            let mut c = std::collections::HashMap::new();
            c.insert("sessionid".into(), "abc123".into());
            c
        }),
        query: Some({
            let mut q = std::collections::HashMap::new();
            q.insert("q".into(), "Rust programming".into());
            q
        }),
    };

    let response = fetch_url_with_options("https://www.wikipedia.org", opts);
    println!("Wikipedia page HTML:\n{}", response);
}
```

```python
import scrapr_rs

opts = scrapr.RequestOptions(
    headers={"User-Agent": "XYZ/1.0"},
    cookies={"sessionid": "abc123"},
    query={"q": "Shrek"}
)

text = scrapr.fetch_url_with_options("https://html.duckduckgo.com/html", opts)

print(text)
```
