import os
import uvicorn
from fastapi import FastAPI
from fastapi.responses import HTMLResponse, FileResponse
from fastapi.staticfiles import StaticFiles


app = FastAPI()
app.mount("/static", StaticFiles(directory="static"), name="static")
app.mount("/assets", StaticFiles(directory="assets"), name="assets")


def read_file(filename: str):
    with open("./static/" + filename + ".html") as f:
        return f.read()


def compile_component(filename: str, *args) -> str:
    with open("./components/" + filename + ".html") as f:
        file = f.read()
        for arg in args:
            file = file.replace(arg[0], arg[1])
        return file


def get_asset(filename: str, group: str):
    with open("./assets/" + group + "/" + filename + ".svg") as f:
        return f.read()


def get_class_color(class_name: str):
    match class_name.lower():
        case "artificer":
            return "sky"
        case "barbarian":
            return "red"
        case "bard":
            return "amber"
        case "blood hunter":
            return "rose"
        case "cleric":
            return "yellow"
        case "druid":
            return "lime"
        case "fighter":
            return "green"
        case "monk":
            return "orange"
        case "paladin":
            return "violet"
        case "ranger":
            return "emerald"
        case "sorcerer":
            return "blue"
        case "warlock":
            return "purple"
        case "wizard":
            return "indigo"
        case _:
            return "primary"


def ability_score_to_modifier(value: int):
    return (value - 10) // 2


@app.get("/", response_class=HTMLResponse)
async def root():
    return read_file("layout2")


@app.get("/title-card", response_class=HTMLResponse)
async def title_card():
    return compile_component("title-card")


# @app.get("/player")
# async def player():
#     pass


# @app.get("/player/avatar", response_class=FileResponse)
# async def player_avatar():
#     with open("./assets/avatar.png") as f:
#         return f


@app.get("/player/name", response_class=HTMLResponse)
async def player_name():
    return "George"


@app.get("/player/race", response_class=HTMLResponse)
async def player_race():
    return "Human"


@app.get("/player/level", response_class=HTMLResponse)
async def player_level():
    return "13"


@app.get("/player/classes/{class_name}/level", response_class=HTMLResponse)
async def player_class_level(class_name: str):
    return "13"


@app.get("/player/classes", response_class=HTMLResponse)
async def player_classes():
    classes = [
        "artificer",
        # "barbarian",
        # "bard",
        # "cleric",
        # "druid",
        # "fighter",
        # "monk",
        # "paladin",
        # "ranger",
        # "rogue",
        # "sorcerer",
        # "warlock",
        # "wizard",
    ]

    content = ""
    for player_class in classes:
        content += compile_component(
            "class-badge",
            ("{color}", get_class_color(player_class)),
            ("{class}", player_class.capitalize()),
            ("{class_lower}", player_class),
        )
    return content


@app.get("/player/abilities/{ability}/score", response_class=HTMLResponse)
def player_attribute(ability: str):
    return str(20)


@app.get("/player/abilities/{ability}/modifier", response_class=HTMLResponse)
def player_attribute(ability: str):
    return str(ability_score_to_modifier(20))


@app.get("/player/skills/{skill}/status", response_class=HTMLResponse)
def player_skill_status(skill: str):
    return compile_component("skill-status-proficient")  #


@app.get("/player/skills/{skill}/bonus", response_class=HTMLResponse)
def player_skill_bonus(skill: str):
    return "5"


@app.get("/player/actions/action", response_class=HTMLResponse)
def player_actions_action():
    actions = [
        {
            "name": "Ray of Frost",
            "type": "Cantrip - Wizard",
            "range": 60,
            "hitdc": 11,
            "damage": {"dice": "4d8", "type": "cold"},
            "notes": "V/S",
        },
        {
            "name": "Unarmed Strike",
            "type": "Melee Attack",
            "range": 5,
            "hitdc": 5,
            "damage": {"dice": "0", "type": "bludgeoning"},
            "notes": "",
        },
        {
            "name": "Melf's Minute Meteors",
            "type": "3rd Level - Wizard",
            "range": "Self",
            "hitdc": "DEX 14",
            "damage": {"dice": "2d6", "type": "fire"},
            "notes": "Concentration, V/S/M",
        },
    ]

    content = ""
    for action in actions:
        content += compile_component(
            "actions-workspace-attack",
            ("{name}", action["name"]),
            ("{type}", action["type"]),
            ("{range}", str(action["range"])),
            ("{hitdc}", str(action["hitdc"])),
            ("{damage}", action["damage"]["dice"]),
            ("{damage_type}", action["damage"]["type"]),
            ("{notes}", action["notes"]),
        )
    return content


@app.get("/player/actions/bonusaction", response_class=HTMLResponse)
def player_actions_bonusaction():
    bonus_actions = [
        {
            "name": "Melf's Minute Meteors",
            "type": "3rd Level - Wizard",
            "range": "Self",
            "hitdc": "DEX 14",
            "damage": {"dice": "2d6", "type": "fire"},
            "notes": "Concentration, V/S/M",
        }
    ]

    content = ""
    for bonus_action in bonus_actions:
        content += compile_component(
            "actions-workspace-attack",
            ("{name}", bonus_action["name"]),
            ("{type}", bonus_action["type"]),
            ("{range}", str(bonus_action["range"])),
            ("{hitdc}", str(bonus_action["hitdc"])),
            ("{damage}", bonus_action["damage"]["dice"]),
            ("{damage_type}", bonus_action["damage"]["type"]),
            ("{notes}", bonus_action["notes"]),
        )
    return content


@app.get("/player/actions/reaction", response_class=HTMLResponse)
def player_actions_reaction():
    reactions = [
        {
            "name": "",
            "type": "",
            "range": "",
            "hitdc": "",
            "damage": {"dice": "", "type": ""},
            "notes": "",
        }
    ]

    content = ""
    for reaction in reactions:
        content += compile_component(
            "actions-workspace-attack",
            ("{name}", reaction["name"]),
            ("{type}", reaction["type"]),
            ("{range}", str(reaction["range"])),
            ("{hitdc}", str(reaction["hitdc"])),
            ("{damage}", reaction["damage"]["dice"]),
            ("{damage_type}", reaction["damage"]["type"]),
            ("{notes}", reaction["notes"]),
        )
    return content


@app.get("/player/actions/other", response_class=HTMLResponse)
def player_actions_other():
    other_actions = [
        {
            "name": "",
            "type": "",
            "range": "",
            "hitdc": "",
            "damage": {"dice": "", "type": ""},
            "notes": "",
        }
    ]

    content = ""
    for action in other_actions:
        content += compile_component(
            "actions-workspace-attack",
            ("{name}", action["name"]),
            ("{type}", action["type"]),
            ("{range}", str(action["range"])),
            ("{hitdc}", str(action["hitdc"])),
            ("{damage}", action["damage"]["dice"]),
            ("{damage_type}", action["damage"]["type"]),
            ("{notes}", action["notes"]),
        )
    return content


@app.get("/player/actions/limiteduse", response_class=HTMLResponse)
def player_actions_limiteduse():
    limiteduse_actions = [
        {
            "name": "",
            "type": "",
            "range": "",
            "hitdc": "",
            "damage": {"dice": "", "type": ""},
            "notes": "",
        }
    ]

    content = ""
    for action in limiteduse_actions:
        content += compile_component(
            "actions-workspace-attack",
            ("{name}", action["name"]),
            ("{type}", action["type"]),
            ("{range}", str(action["range"])),
            ("{hitdc}", str(action["hitdc"])),
            ("{damage}", action["damage"]["dice"]),
            ("{damage_type}", action["damage"]["type"]),
            ("{notes}", action["notes"]),
        )
    return content


if __name__ == "__main__":
    app.mount("/static", StaticFiles(directory="static"), name="static")
    app.mount("/assets", StaticFiles(directory="assets"), name="assets")
    os.system(
        "uvicorn layout2:app --reload --reload-dir static --reload-include output.css"
    )  # dev
    # uvicorn.run(app, host="127.0.0.1", port=8000) #production
