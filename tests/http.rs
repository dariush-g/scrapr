use scraprr::prelude::*;

#[test]
fn test_simple_request_() {
    let body = fetch_http("example.com", "/").unwrap();
    assert!(body.contains("Example Domain"));
}

#[test]
fn test_extract_tag_() {
    let html = "<p>Hello</p><div>skip</div><p>World</p>";
    let result = extract_tag(html, "p").unwrap();
    assert_eq!(result, vec!["Hello", "World"]);
}
