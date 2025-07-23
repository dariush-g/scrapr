import scrapr

html = scrapr.fetch_https("https://www.wikipedia.org/", "/")

print(html)


