import json
from sys import argv
from os import listdir

if __name__ == "__main__":
    standart = {
        "units": [
            {
                "name": "blue print name",
                "workflow": {
                    "authentication": "pre-executor",
                    "script1.lua": "script",
                    "script2.lua": "script"


                }
            }
        ]
    }
    for _ in range(0, int(argv[1])):
        file = open(f"blueprint{len(listdir())}.json", "w")
        json.dump(standart,file)
        file.close()
