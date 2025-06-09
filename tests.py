import json

potionsL = {}
with open("potions.json", "r") as f:
    potionsL = json.load(f)
    f.close()
x = 0
while x <10:
    print(potionsL["potions"][x]["recipie"])
    x+=1