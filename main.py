from pypdf import PdfReader

reader = PdfReader("input.pdf")
meta = reader.metadata

# print("page length:", len(reader.pages))
# print("author:", meta.author)
# print("created in (software):", meta.creator)
# print("producer:", meta.producer)
# print("subject:", meta.subject)
# print("title:", meta.title)

a = []

for page in reader.pages:
    a.append(page.extract_text())

# for line in a:
#     print("\n\n\n\n")
#     print(line)

def find_substring_in_array(substring, array):
    return "\n".join(s for s in array if substring.lower() in s.lower())

print(find_substring_in_array("Sales tax @", a))
print(len(a))
