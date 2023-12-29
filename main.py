import os
from typing import Optional
import requests
from fastapi.responses import FileResponse, HTMLResponse
import uvicorn
from fastapi import FastAPI
from fastapi.staticfiles import StaticFiles

app = FastAPI()
app.mount("/static", StaticFiles(directory="static"), name="static")  # production

CORE_TOOLS_URL = "https://5etools-mirror-1.github.io/data"


def get_classes(exclude_sources: list[str] = []):
    classes_url = CORE_TOOLS_URL + "/class/"
    class_addresses = requests.get(classes_url).json()
    classes = []
    for class_address in class_addresses:
        class_object = requests.get(classes_url + class_addresses[class_address]).json()
        if class_object["class"][0]["source"] not in exclude_sources:
            classes.append(class_object)
    print(classes)


def compile_component(filename: str, *args) -> str:
    with open("./components/" + filename + ".html") as f:
        file = f.read()
        for arg in args:
            file = file.replace(arg[0], arg[1])
        return file


def expand_source(source: str) -> Optional[str]:
    books = requests.get("https://5etools-mirror-1.github.io/data/books.json").json()
    for book in books["book"]:
        if book["source"] == source:
            return book["name"]
    return source


def race_traittags(name: str, source: str, isRace: bool):
    content = requests.get(CORE_TOOLS_URL + "/races.json").json()
    if isRace:
        content = content["race"]
    else:
        content = content["subrace"]

    for obj in content:
        if "name" in obj.keys() and obj["name"] == name and obj["source"] == source:
            return (
                []
                if "traitTags" not in obj.keys() or obj["traitTags"] is None
                else obj["traitTags"]
            )
    return ["not_found"]


def handle_slash(name: str) -> str:
    return name.replace("/", "^")


def parse_slash(name: str) -> str:
    return name.replace("^", "/")


@app.get("/init", response_class=HTMLResponse)
async def init():
    return compile_component("welcome", ("{name}", "Garrett"))


@app.get("/races", response_class=HTMLResponse)
async def races():
    races = requests.get(CORE_TOOLS_URL + "/races.json").json()["race"]
    elements = ""
    marked_races = []
    for race in races:
        if race["name"] not in marked_races:
            elements += compile_component("race_dropdown", ("{name}", race["name"]))
            marked_races.append(race["name"])
    return elements


@app.get("/test_thing/{thing}")
async def thing(thing: str):
    return handle_slash(thing)


@app.get("test_thing_2/{thing}")
async def thing2(thing: str):
    return parse_slash(thing)


@app.get("/racetags/{name}/{source}/{isRace}", response_class=HTMLResponse)
async def racetags(name: str, source: str, isRace: str):
    elements = ""
    for tag in race_traittags(parse_slash(name), source, isRace == "True"):
        elements += compile_component(
            "badge",
            ("{size}", "md"),
            ("{color}", "primary"),
            ("{content}", tag),
        )
    return elements


@app.get("/isNPC/{name}/{source}/{isRace}", response_class=HTMLResponse)
async def isNPC(name: str, source: str, isRace: str):
    if "NPC Race" in race_traittags(parse_slash(name), source, isRace == "True"):
        return compile_component(
            "badge",
            ("{size}", "md"),
            ("{color}", "primary"),
            ("{content}", "NPC Race"),
        )
    return ""


@app.get("/subraces/{parent_name}", response_class=HTMLResponse)
async def subraces(parent_name: str):
    subraces = requests.get(CORE_TOOLS_URL + "/races.json").json()["subrace"]
    races = requests.get(CORE_TOOLS_URL + "/races.json").json()["race"]
    elements = ""
    for race in races:
        if race["name"] == parent_name:
            elements += compile_component(
                "race_collapse",
                ("{name}", race["name"]),
                ("{handle_name}", handle_slash(race["name"])),
                ("{source}", race["source"]),
                ("{full_source}", expand_source(race["source"])),
                ("{isRace}", str(True)),
            )
    for subrace in subraces:
        if subrace["raceName"] == parent_name:
            try:
                elements += compile_component(
                    "race_collapse",
                    ("{name}", subrace["name"]),
                    ("{handle_name}", handle_slash(subrace["name"])),
                    ("{source}", subrace["source"]),
                    ("{full_source}", expand_source(subrace["source"])),
                    ("{isRace}", str(False)),
                )
            except:
                # base race
                pass
                # print(subrace.keys())
                # print(subrace)
    return elements


def serve():
    uvicorn.run(app)


if __name__ == "__main__":
    # serve() # production
    os.system("uvicorn main:app --reload")  # dev
