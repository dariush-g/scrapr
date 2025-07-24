use scrapr::prelude::*;

#[test]
fn test_fetch_https_() {
    if let Ok(text) = fetch_https("example.com", "/") {
        let e_d = extract_tag(&text, "title").unwrap();
        assert_eq!(e_d, vec!["Example Domain"]);
    }
}
