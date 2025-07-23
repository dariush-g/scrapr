import scrapr

opts = scrapr.RequestOptions(
    headers={"User-Agent": "MyBot/1.0"},
    cookies={"sessionid": "abc123"},
    query={"q": "rust", "page": "2"}
)

html = scrapr.fetch_http_with_options("example.com", "/", opts)
# print(html)



opts = scrapr.RequestOptions(
    headers={"User-Agent": "MyScraper/1.0"},
    cookies={"sessionid": "abc123"},
    query={"q": "tls"}
)

html = scrapr.fetch_https_with_options("en.wikipedia.org", "/wiki/TLS", opts)
print(html)


opts = scrapr.RequestOptions(
    headers={},
    cookies={},
    query={"q": "rust programming"}
)


response = scrapr.fetch_http_with_options(
    "example.com",
    "/search",
    opts
)

print(response)