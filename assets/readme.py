import subprocess
import json
import requests 
result = subprocess.check_output(["cloc", "--vcs=git", "--json"], text=True)
cloc_data = json.loads(result)
total = sum((cloc_data["SUM"]["comment"], cloc_data["SUM"]["code"]))
res = requests.get(f"https://img.shields.io/badge/{total}-lines-green?style=flat")
with open("assets/lines.svg", "wb") as f:
    f.write(res.content)

