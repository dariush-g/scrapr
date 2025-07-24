use scrapr::prelude::*;

fn main() {
    let text = fetch_url("https://openai.com/").unwrap();
    let title = extract_tag(&text, "title").unwrap();

    println!("{title:?}");
}
