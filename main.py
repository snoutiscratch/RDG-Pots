import json
import os
import sys

print("A janky potion calculator!")
print("v1 : 2/19/23")

berriesList = [
    "abate",
    "accure",
    "algid",
    "ardor",
    "cavort",
    "fervor",
    "gambol",
    "ichor",
    "lurce",
    "luminous",
    "skew",
    "tenebrous",
    "theriac",
    "torrid",
    "vigor",
    "virulent"
]
hasList = []

potionsL = {}
with open("potions.json", "r") as f:
    potionsL = json.load(f)
    f.close()

def getPotions():
    for x in potionsL["potions"]:
        could = False
        couldI = False

        for potionIngredient in potionsL["potions"][x]["recipe"]:
            if potionIngredient in hasList:
                could = True
            if potionIngredient[x+1] in berriesList:
                couldI = True
        
            if could & couldI:
                print(potionsL["potions"][x]["name"])
                print(potionsL["potions"][x]["recipeReadable"])
                print("-"*34)

# Main
while True:
    print(" 1: Get Potions \n 2: Input berries \n 3: Remove berries \n 4: Clear Berries \n 5: List berries")
    action = input("action:")
    print("-"*34)

    if action == "1":
        print("Possible Potions:")
        getPotions()

    # Input berriesa
    if action == "2":
        print("Alright, input berries or type none to exit: ")
        berriesIn = (input("Wat berry? ")).lower()
        while berriesIn != "none":
            if (berriesIn in hasList):
                print("Ya already got one of those")
            elif berriesIn in berriesList:
                hasList.append(berriesIn)
                print("Added.")
            else:
                print("That no exist!!")
    
            berriesIn = (input("Wat berry? ")).lower()

    if action == "3":
        removeBerry = (input("Enter berries to remove, or none to exit: ")).lower()
        while removeBerry != "none":
            if removeBerry in hasList:
                hasList.remove(removeBerry)
                print("Removed")
            else:
                print("That ain't in there!")
            removeBerry = (input("Enter berries to remove, or none to exit: ")).lower()

    if action == "4":
        hasList.clear()
        print("Cleared Berries")

    if action == "5":
        print("You got:")
        for b in hasList:
            print(b)
    
    if action == "exit":
        break

    if action == "re":
        for x in range(69):
            print("\n") 
        print("Restart")
        os.execl(sys.executable, sys.executable, *sys.argv)

    print(("-"*11), ("="*6), "0", ("="*6), ("-"*11), sep="")