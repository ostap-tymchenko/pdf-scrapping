from PyPDF2 import PdfReader
  
# creating a pdf reader object
reader = PdfReader('input.pdf')
  
# getting a specific page from the pdf file
a = []

# a.append(reader.pages)
# print(a)
for page in reader.pages:
    a.append(page.extract_text())

for line in a:
    print(line)
