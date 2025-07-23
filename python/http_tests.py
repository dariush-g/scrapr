import scrapr

html = scrapr.fetch_http("example.com", "/")
assert "Example Domain" in html

html = "<p>Hello</p><div>skip</div><p>World</p>"
paras = scrapr.extract_tag(html, "p")
assert paras == ["Hello", "World"]

print("All tests passed!")

