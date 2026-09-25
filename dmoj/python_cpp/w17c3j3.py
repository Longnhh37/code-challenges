# wc17c3j3
import re

text = input()

pattern = r"^(?=(?:.*[a-z]){3,})(?=(?:.*[A-Z]){2,})(?=(?:.*\d){1,})[a-zA-Z0-9]{8,12}$"

if re.match(pattern, text):
    print("Valid")
else:
    print("Invalid")

