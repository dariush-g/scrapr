import scraprr

text = scraprr.fetch_url("https://www.cs.hmc.edu/~dodds/demo.html")

tag = scraprr.extract_tag(text, "ul")

print(tag)


opts = scraprr.RequestOptions(
    headers={"User-Agent": "XYZ/1.0"},
    cookies={"sessionid": "abc123"},
    query={"q": "Shrek"}
)

text = scraprr.fetch_url_with_options("https://openai.com/", opts)
title = scraprr.extract_tag(text, "title")
print(title)