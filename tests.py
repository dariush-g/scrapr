import scrapr

# smoke-test the raw HTTP fetch
html = scrapr.fetch_http("example.com", "/")
assert "Example Domain" in html

# test your tag extractor
html = "<p>Hello</p><div>skip</div><p>World</p>"
paras = scrapr.extract_tag(html, "p")
assert paras == ["Hello", "World"]

print("All tests passed!")

