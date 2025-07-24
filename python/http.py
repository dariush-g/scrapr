import scraprr

html = scraprr.fetch_http("example.com", "/")
assert "Example Domain" in html

html = "<p>Hello</p><div>skip</div><p>World</p>"
paras = scraprr.extract_tag(html, "p")
assert paras == ["Hello", "World"]

print("All tests passed!")

s