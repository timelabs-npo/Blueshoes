import sys
from bs4 import BeautifulSoup

with open("/Users/sa/.gemini/antigravity-ide/brain/2ed41831-9ffb-4716-89af-eea92fc4cc5d/.system_generated/steps/5/content.md", "r") as f:
    html_content = f.read()

soup = BeautifulSoup(html_content, "html.parser")
for element in soup.find_all("h3"):
    print("Offer:", element.text)
    sibling = element.find_next_sibling()
    while sibling and sibling.name not in ["h3", "h2", "h1"]:
        if sibling.name in ["p", "ul", "div"]:
            text = sibling.get_text(separator=" ", strip=True)
            if text:
                print("  ", text)
        sibling = sibling.find_next_sibling()
    print("-" * 40)
