import requests

ip = input("Enter the IP address...")

data = requests.get(f"https://ipapi.co/{ip}/json/")
json = data.json()

print(json["country_name"])
