use scrapr::prelude::*;

#[test]
fn test_fetch_url() {
    let text = fetch_url("https://openai.com/").unwrap();
    let title = extract_tag(&text, "title").unwrap();

    assert_eq!(title, ["OpenAI"])
}
