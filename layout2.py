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


def color_to_variable(color_name: str) -> str:
    match color_name:
        case "primary":
            return "oklch(var(--p))"
        case "primary-content":
            return "oklch(var(--pc))"
        case "secondary":
            return "oklch(var(--s))"
        case "secondary-content":
            return "oklch(var(--sc))"
        case "accent":
            return "oklch(var(--a))"
        case "accent-content":
            return "oklch(var(--ac))"
        case "neutral":
            return "oklch(var(--n))"
        case "neutral-content":
            return "oklch(var(--nc))"
        case "base-100":
            return "oklch(var(--b1))"
        case "base-200":
            return "oklch(var(--b2))"
        case "base-300":
            return "oklch(var(--b3))"
        case "base-content":
            return "oklch(var(--bc))"
        case "info":
            return "oklch(var(--in))"
        case "success":
            return "oklch(var(--inc))"
        case "warning":
            return "oklch(var(--wa))"
        case "warning-content":
            return "oklch(var(--wac))"
        case "error":
            return "oklch(var(--er))"
        case "error-content":
            return "oklch(var(--erc))"
        case _:
            return "oklch(var(--bc))"


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


@app.get("/assets/")


@app.get("/player/name", response_class=HTMLResponse)
async def player_name():
    return "George"


@app.get("/player/race", response_class=HTMLResponse)
async def player_race():
    return "Human"


@app.get("/player/level", response_class=HTMLResponse)
async def player_level():
    return "13"


@app.get("/player/hitpoints/max", response_class=HTMLResponse)
async def player_hitpoints_max():
    return "304"


@app.get("/player/hitpoints/current", response_class=HTMLResponse)
async def player_hitpoints_current():
    return "250"


@app.get("/player/hitpoints/percent", response_class=HTMLResponse)
async def player_hitpoints_percent():
    return str(round((250 / 304) * 100)) + "%"


@app.get("/player/hitpoints/temp", response_class=HTMLResponse)
async def player_hitpoints_temp():
    return "12"


@app.get("/player/armorclass", response_class=HTMLResponse)
async def player_armorclass():
    return "20"


@app.get("/player/initiative", response_class=HTMLResponse)
async def player_initiative():
    return "4"


@app.get("/player/speed/walking", response_class=HTMLResponse)
async def player_speed_walking():
    return "30"


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
    return compile_component("skill-status-proficient")


@app.get("/player/skills/{skill}/bonus", response_class=HTMLResponse)
def player_skill_bonus(skill: str):
    return "5"


@app.get("/player/senses/perception", response_class=HTMLResponse)
def player_senses_perception():
    return "18"


@app.get("/player/senses/investigation", response_class=HTMLResponse)
def player_senses_perception():
    return "18"


@app.get("/player/senses/insight", response_class=HTMLResponse)
def player_senses_perception():
    return "18"


@app.get("/player/proficiencies/armor", response_class=HTMLResponse)
def player_proficiencies_armor():
    return "None"


@app.get("/player/proficiencies/weapons", response_class=HTMLResponse)
def player_proficiencies_weapons():
    return "Crossbow, Light, Dagger, Dart, Quarterstaff, Sling"


@app.get("/player/proficiencies/tools", response_class=HTMLResponse)
def player_proficiencies_tools():
    return "Tinkerers' Tools, Artisan Tools"


@app.get("/player/proficiencies/languages", response_class=HTMLResponse)
def player_proficiencies_languages():
    return "Abyssal, Celestial, Draconic"


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
