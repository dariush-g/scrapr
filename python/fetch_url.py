import scrapr

text = scrapr.fetch_url("https://www.cs.hmc.edu/~dodds/demo.html")

tag = scrapr.extract_tag(text, "ul")

print(tag)


opts = scrapr.RequestOptions(
    headers={"User-Agent": "XYZ/1.0"},
    cookies={"sessionid": "abc123"},
    query={"q": "Shrek"}
)

text = scrapr.fetch_url_with_options("https://html.duckduckgo.com/html", opts)

print(text)