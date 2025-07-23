import scrapr

text = scrapr.fetch_url("https://www.cs.hmc.edu/~dodds/demo.html")

tag = scrapr.extract_tag(text, "ul")

print(tag)

