from pypdf import PdfReader

reader = PdfReader("input.pdf")

meta = reader.metadata

print("page length:", len(reader.pages))

# All of the following could be None!
print("author:", meta.author)
print("created in (software):", meta.creator)
print("producer:", meta.producer)
print("subject:", meta.subject)
print("title:", meta.title)
