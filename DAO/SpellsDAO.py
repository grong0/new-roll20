import json
from typing import Any


class Time:
    def __init__(self, object: dict) -> None:
        pass


class Range:
    def __init__(self, object: dict) -> None:
        pass


class Components:
    def __init__(self, object: dict) -> None:
        pass


class Duration:
    def __init__(self, object: dict) -> None:
        pass


class ScalingLevelDice:
    def __init__(self, object: dict) -> None:
        pass


class Spell:
    name: str
    soruce: str
    srd: bool
    level: int
    school: str
    time: Time
    range: Range
    components: Components
    duration: Duration
    entries: list[dict[str, list[str] | str]] | str
    scaling_level_dice: ScalingLevelDice
    damage_inflict: list[str]
    saving_throw: str # is a list[str] in the json
    misc_tags: list[str]
    area_tags: list[str]
    entries_higher_levels: str # is a dict[str, list[str] | str] in the json
    ritual: bool
    condition_inflict: list[str]
    affects_creature_type: list[str]
    damage_resist: list[str]

    def __init__(self, object: dict[str, Any]) -> None:
        self.name = object["name"]
        self.source = object["source"]


class Spells:
    spells: dict[str, dict[str, Spell]]

    def __init__(self) -> None:
        self.spells = {}

        with open("../data/spells/index.json") as f:
            index = json.load(f)

            for source in index.keys():
                self.spells[source] = {}

                with open(f"../data/spells/{index[index]}") as f:
                    spells = json.load(f)

                    for spell in spells["spell"]
                        self.spells[source][spell["name"]] = Spell(spell)
