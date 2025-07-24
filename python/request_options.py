import scraprr

opts = scraprr.RequestOptions(
    headers={"User-Agent": "MyBot/1.0"},
    cookies={"sessionid": "abc123"},
    query={"q": "rust", "page": "2"}
)

html = scraprr.fetch_http_with_options("example.com", "/", opts)
# print(html)


opts = scraprr.RequestOptions(
    headers={"User-Agent": "XYZ/1.0"},
    cookies={"sessionid": "abc123"},
    query={"q": "tls"}
)

html = scraprr.fetch_https_with_options("en.wikipedia.org", "/wiki/TLS", opts)
print(html)


opts = scraprr.RequestOptions(
    headers={},
    cookies={},
    query={"q": "rust programming"}
)


response = scraprr.fetch_http_with_options(
    "example.com",
    "/search",
    opts
)

print(response)