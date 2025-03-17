import json
import datetime
from typing import Any


class Time:
    quantity: int
    unit: str

    def __init__(self, object: dict) -> None:
        self.quantity = object["number"]
        self.unit = object["unit"]


class Range:
    type: str
    form: str
    amount: int

    def __init__(self, object: dict) -> None:
        self.type = object["type"]
        if self.type == "special":
            self.form = ""
            self.amount = -1
            return
        self.form = object["distance"]["type"]
        if self.form not in ["self", "touch", "sight", "unlimited"]:
            self.amount = object["distance"]["amount"]
            return
        self.amount = -1


class Components:
    v: bool
    s: bool
    m: str
    r: str

    def __init__(self, object: dict) -> None:
        # impl 1
        # v = "v" in object.keys()
        # s = "s" in object.keys()
        # m = object["m"] if "m" in object.keys() else ""

        # impl 2
        self.v = False
        self.s = False
        self.m = ""
        self.r = ""

        for key in object.keys():
            if key == "v":
                self.v = True
            elif key == "s":
                self.s = True
            elif key == "m":
                self.m = object["m"]
            elif key == "r":
                self.r = object["r"]
            else:
                print(key)


class Duration:
    type: str
    unit: str
    amount: int
    concentration: bool
    ends: list[str]

    def __init__(self, object: dict) -> None:
        self.type = object["type"]
        self.ends = []
        self.concentration = (
            object["concentration"] if "concentration" in object.keys() else False
        )
        if "duration" in object.keys():
            self.unit = object["duration"]["type"]
            self.amount = object["duration"]["amount"]
        else:
            self.amount = 0
            self.unit = ""
        self.ends = object["ends"] if "ends" in object.keys() else []


class ScalingLevelDice:
    type: str
    scaling: dict[int, str]

    def __init__(self, object: dict) -> None:
        self.type = object["label"][: object["label"].find(" ")]
        self.scaling = object["scaling"]


class Spell:
    name: str
    source: str
    page: int
    srd: bool
    level: int
    school: str
    time: Time
    range: Range
    components: Components
    duration: Duration
    entries: list[dict[str, list[str] | str]] | str
    scaling_level_dice: list[ScalingLevelDice]
    damage_inflict: list[str]
    saving_throw: str  # is a list[str] in the json
    misc_tags: list[str]
    area_tags: list[str]
    other_sources: list[str]
    entries_higher_levels: str  # is a dict[str, list[str] | str] in the json
    ritual: bool
    condition_inflict: list[str]
    affects_creature_type: list[str]
    damage_resist: list[str]

    def __init__(self, object: dict[str, Any]) -> None:
        self.name = object["name"]
        self.source = object["source"]
        self.page = object["page"] if "page" in object.keys() else -1
        self.srd = object["srd"] if "srd" in object.keys() else False
        self.level = object["level"]
        self.school = object["school"]
        self.time = Time(object["time"][0])
        self.range = Range(object["range"])
        self.components = Components(object["components"])
        self.duration = Duration(object["duration"][0])
        self.entries = object["entries"]
        if "scalingLevelDice" in object.keys():
            if object["scalingLevelDice"] is list:
                self.scaling_level_dice = [
                    ScalingLevelDice(scaling) for scaling in object["scalingLevelDice"]
                ]
            elif object["scalingLevelDice"] is dict:
                self.scaling_level_dice = [ScalingLevelDice(object["scalingLevelDice"])]
        else:
            self.scaling_level_dice = []
        self.damage_inflict = (
            object["damageInflict"] if "damageInflict" in object.keys() else []
        )
        self.saving_throw = (
            object["savingThrow"] if "savingThrow" in object.keys() else ""
        )
        self.misc_tags = object["miscTags"] if "miscTags" in object.keys() else []
        self.area_tags = object["areaTags"] if "areaTags" in object.keys() else []
        self.other_sources = (
            object["otherSources"] if "otherSources" in object.keys() else []
        )
        self.entries_higher_levels = (
            object["entriesHigherLevels"]
            if "entriesHigherLevels" in object.keys()
            else ""
        )
        self.ritual = object["meta"]["ritual"] if "meta" in object.keys() else False
        self.condition_inflict = (
            object["conditionInflict"] if "conditionInflict" in object.keys() else []
        )
        self.affects_creature_type = (
            object["affectsCreatureType"]
            if "affectsCreatureType" in object.keys()
            else []
        )
        self.damage_resist = (
            object["damageResist"] if "damageResist" in object.keys() else []
        )


class Spells:
    spells: dict[str, dict[str, Spell]]

    def __init__(self) -> None:
        self.spells = {}

        with open("../data/raw/spells/index.json") as f:
            index = json.load(f)

            for source in index.keys():
                self.spells[source] = {}

                with open(f"../data/raw/spells/{index[source]}") as f:
                    spells = json.load(f)

                    for spell in spells["spell"]:
                        self.spells[source][spell["name"]] = Spell(spell)


if __name__ == "__main__":
    times = []
    count = 1000
    for _ in range(0, count):
        start = datetime.datetime.now()
        spells = Spells()
        end = datetime.datetime.now()
        times.append((end - start).microseconds)
        del spells

    average_time = sum(times) / count

    print(f"average time was {average_time} microseconds")
