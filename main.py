import os
from typing import Optional

import uvicorn
from fastapi import FastAPI
from fastapi.responses import HTMLResponse
from fastapi.staticfiles import StaticFiles

from raw_api import API

app = FastAPI()
app.mount("/static", StaticFiles(directory="static"), name="static")
api = API()

CORE_TOOLS_URL = "https://5etools-mirror-1.github.io/data"


def handle_normal_block(content: str) -> str:
    return compile_component("kbd-md", ("{content}", content.title()))


def handle_note(content: str) -> str:
    return compile_component("quote", ("{content}", content))


def handle_item(content: str) -> str:
    return compile_component(
        "kbd-md", ("{content}", content[: content.find("|")].title())
    )


def handle_die_and_damage(content: str) -> str:
    return compile_component("kbd-md", ("{content}", content))


def handle_special_blocks(content: str) -> tuple[str, list[dict[str, int | str]]]:
    while content.find("{@") != -1:
        start_index = content.find("{@")
        end_index = content[start_index:].find("}") + start_index
        raw = content[start_index : end_index + 1]
        block_type = raw[raw.find("@") + 1 : raw.find(" ")]
        block_content = raw[raw.find(" ") + 1 : -1]

        element = ""
        match block_type:
            case "note":
                element = handle_note(block_content)
            case "item":
                element = handle_item(block_content)
            case "dice":
                element = handle_die_and_damage(block_content)
            case "damage":
                element = handle_die_and_damage(block_content)
            case _:
                element = handle_normal_block(block_content)

        content = content.replace(raw, element)
    return content


def compile_component(filename: str, *args) -> str:
    with open("./components/" + filename + ".html") as f:
        file = f.read()
        if len(args) == 0:
            return file
        for arg in args:
            file = file.replace(arg[0], arg[1])
        return handle_special_blocks(file)


def compile_components(filename: str, *args, to_list: bool = False) -> str | list:
    content = [] if to_list else ""
    with open("./components/" + filename + ".html") as f:
        file = f.read()
        if len(args) == 0:
            return file
        for i in range(0, len(args[0][1])):
            current_mod = file
            for arg in args:
                if to_list:
                    if arg[1][i][0] == "|" and arg[1][i][-1] == "|":
                        content.append(arg[1][i])
                    else:
                        content.append(current_mod.replace(arg[0], arg[1][i]))
                else:
                    if arg[1][i][0] == "|" and arg[1][i][-1] == "|":
                        content += " " + arg[1][i][1:-1] + " "
                    else:
                        content += current_mod.replace(arg[0], arg[1][i])
    return content


def title_list(list: list[str]) -> list[str]:
    for item in list:
        item = item.title()
    return list


def ability_thats_missing(list: list[str]) -> Optional[str]:
    all_abilities = ["str", "dex", "con", "int", "wis", "cha"]
    for ability in list:
        all_abilities.remove(ability)
    if len(all_abilities) == 1:
        return all_abilities[0]
    return None


def expand_ability_scores(race: dict, isRace: bool) -> list[str]:
    if isRace:
        ability = api.get_race_ability(race)
    else:
        ability = api.get_subrace_ability(race)
    ability_score_strs = []
    for ability_scores in ability:
        for ability_score in ability_scores:
            print(ability_score)
            if ability_score == "Err":
                print(f"race lineage: {race['lineage']}")
                if race["lineage"] == "VRGR":
                    return [
                        "Choose any +2; choose any other +1",
                        "|or|",
                        "Choose three different +1",
                    ]
                return ["ability was None"]
            if ability_score == "choose":
                try:
                    amount = ability_scores[ability_score]["amount"]
                except Exception:
                    amount = ability_scores[ability_score]["count"]
                abilities = ability_scores[ability_score]["from"]
                guarrenteed_ability = ability_thats_missing(abilities)
                if len(abilities) == 6:
                    abilities = "any"
                elif (
                    guarrenteed_ability is not None
                    and guarrenteed_ability in ability_scores.keys()
                ):
                    ability_score_strs.append(f"Choose any other {amount}")
                else:
                    abilities = title_list(abilities)
                    name = "Choose"
                    ability_score_strs.append(f"{name} {abilities} {amount} ")
            else:
                amount = ability_scores[ability_score]
                name = ability_score.title()
                ability_score_strs.append(f"{amount} {name}")
    return ability_score_strs


def expand_size(sizes: list[str]) -> str:
    str_sizes = sizes[0]
    for size in sizes[1:]:
        str_sizes += " " + size
    return str_sizes.replace("S", "Small").replace("M", "Medium").replace(" ", " or ")


def expand_speed(speed: int | dict) -> str:
    if isinstance(speed, int):
        return f"{speed} ft."
    else:
        speed_str = f"Walk {speed['walk']}ft"
        for key in speed.keys():
            if key != "walk":
                value = speed[key]
                if value is True:
                    value = speed["walk"]
                speed_str += f", {key.title()} {value}ft"
        return speed_str


def handle_slash(name: str) -> str:
    return name.replace("/", "^")


def parse_slash(name: str) -> str:
    return name.replace("^", "/")


@app.get("/race_categories", response_class=HTMLResponse)
async def race_categories():
    return compile_components("race_category", ("{name}", api.get_unique_race_names()))


@app.get("/race_category_content/{name}", response_class=HTMLResponse)
async def race_category_content(name: str):
    content = ""
    for race in api.get_races(name):
        if api.race_has_subraces(race):
            content += compile_component(
                "race_dropdown", ("{name}", race["name"]), ("{source}", race["source"])
            )
        else:
            book = api.get_book_from_source(race["source"])
            book_name = race["source"] if book is None else book["name"]
            content += compile_component(
                "race_collapse",
                ("{name}", race["name"]),
                ("{handle_name}", handle_slash(race["name"])),
                ("{source}", race["source"]),
                ("{full_source}", book_name),
                ("{isRace}", "True"),
            )
    if content == "":
        return compile_component("loading-spinner")
    return content


@app.get("/traitTags/{name}/{source}/{isRace}", response_class=HTMLResponse)
async def racetags(name: str, source: str, isRace: str):
    isRace = isRace == "True"
    if isRace:
        traitTags = api.get_race_traitTags(api.get_race(name, source))
    else:
        traitTags = api.get_subrace_traitTags(api.get_subrace(name, source))
    return compile_components("badge-primary-md", ("{content}", traitTags))


@app.get("/isNPC/{handle_name}/{source}/{isRace}", response_class=HTMLResponse)
async def isNPC(handle_name: str, source: str, isRace: str):
    name = parse_slash(handle_name)
    isRace = isRace == "True"
    if isRace:
        traitTags = api.get_race_traitTags(api.get_race(name, source))
    else:
        traitTags = api.get_subrace_traitTags(api.get_subrace(name, source))
    if "NPC Race" in traitTags:
        return compile_component(
            "badge-primary-md",
            ("{content}", "NPC Race"),
        )
    return ""


@app.get("/subraces/{parent_name}/{parent_source}", response_class=HTMLResponse)
async def subraces(parent_name: str, parent_source: str):
    parent = api.get_race(parent_name, parent_source)
    element = ""
    if api.race_has_base(parent):
        book = api.get_book_from_source(parent_source)
        book_name = parent_source if book is None else book["name"]
        element = compile_component(
            "race_collapse",
            ("{name}", parent_name),
            ("{handle_name}", handle_slash(parent_name)),
            ("{source}", parent_source),
            ("{full_source}", book_name),
            ("{isRace}", "True"),
        )
    for subrace in api.get_race_subraces(parent):
        if "name" in subrace.keys():
            book = api.get_book_from_source(subrace["source"])
            book_name = subrace["source"] if book is None else book["name"]
            element += compile_component(
                "race_collapse",
                ("{name}", subrace["name"]),
                ("{handle_name}", handle_slash(subrace["name"])),
                ("{source}", subrace["source"]),
                ("{full_source}", book_name),
                ("{isRace}", "False"),
            )
    if element == "":
        return compile_component("loading-spinner")
    return element


@app.get("/race_details/{handle_name}/{source}/{isRace}", response_class=HTMLResponse)
async def race_details(handle_name: str, source: str, isRace: str):
    name = parse_slash(handle_name)
    isRace = isRace == "True"
    if isRace:
        object = api.get_race(name, source)
        entries = api.get_race_entries(object)
        size = expand_size(api.get_race_size(object))
        speed = expand_speed(api.get_race_speed(object))
    else:
        object = api.get_subrace(name, source)
        entries = api.get_subrace_entries(object)
        size = expand_size(api.get_race_size(api.get_subrace_parent(object)))
        speed = expand_speed(api.get_race_speed(api.get_subrace_parent(object)))
    ability_scores = compile_components(
        "kbd-md", ("{content}", expand_ability_scores(object, isRace))
    )

    traits = ""
    for entry in entries:
        if isinstance(entry, str):
            traits += compile_component("p", ("{content}", entry))
        else:
            content = ""
            for sub_entry in entry["entries"]:
                if isinstance(sub_entry, str):
                    content += compile_component("p", ("{content}", sub_entry))
                elif sub_entry["type"] == "table":
                    headers = compile_components(
                        "th", ("{content}", sub_entry["colLabels"])
                    )
                    rows_content = []
                    for row in sub_entry["rows"]:
                        rows_content.append(
                            compile_components("td", ("{content}", row))
                        )
                    rows = compile_components("tr", ("{content}", rows_content))
                    content += compile_component(
                        "table", ("{headers}", headers), ("{content}", rows)
                    )
                else:
                    print(f"{name} had an entry of a different type")
            traits += compile_component(
                "race_trait", ("{name}", entry["name"]), ("{content}", content)
            )
    # print(f"traits: {traits}")
    return compile_component(
        "race_traits",
        ("{name}", name),
        ("{source}", source),
        ("{ability_scores}", ability_scores),
        ("{size}", size),
        ("{speed}", speed),
        ("{traits}", traits),
    )


def serve():
    uvicorn.run(app)


if __name__ == "__main__":
    app.mount("/static", StaticFiles(directory="static"), name="static")
    # serve() # production
    os.system("uvicorn main:app --reload")  # dev
