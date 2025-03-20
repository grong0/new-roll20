import json
import datetime
from typing import Any
from src.DAO.MiscClasses import Time, Range, Components, Duration, ScalingLevelDice


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
    entries_higher_levels: str  # TODO: Make a class for this | is a dict[str, list[str] | str] in the json
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
                self.scaling_level_dice = [ScalingLevelDice(scaling) for scaling in object["scalingLevelDice"]]
            elif object["scalingLevelDice"] is dict:
                self.scaling_level_dice = [ScalingLevelDice(object["scalingLevelDice"])]
        else:
            self.scaling_level_dice = []
        self.damage_inflict = object["damageInflict"] if "damageInflict" in object.keys() else []
        self.saving_throw = object["savingThrow"] if "savingThrow" in object.keys() else ""
        self.misc_tags = object["miscTags"] if "miscTags" in object.keys() else []
        self.area_tags = object["areaTags"] if "areaTags" in object.keys() else []
        self.other_sources = object["otherSources"] if "otherSources" in object.keys() else []
        self.entries_higher_levels = object["entriesHigherLevels"] if "entriesHigherLevels" in object.keys() else ""
        self.ritual = object["meta"]["ritual"] if "meta" in object.keys() else False
        self.condition_inflict = object["conditionInflict"] if "conditionInflict" in object.keys() else []
        self.affects_creature_type = object["affectsCreatureType"] if "affectsCreatureType" in object.keys() else []
        self.damage_resist = object["damageResist"] if "damageResist" in object.keys() else []

    def as_dict(self) -> dict[str, Any]:
        """This is effectivaly read only as it creates a copy

        Returns:
            dict[str, Any]: the class as a dict
        """

        copy = self.__dict__.copy()
        copy["time"] = copy['time'].__dict__
        copy["range"] = copy['range'].__dict__
        copy["components"] = copy['components'].__dict__
        copy["duration"] = copy['duration'].__dict__
        copy["scaling_level_dice"] = [scale.__dict__ for scale in copy['scaling_level_dice']]
        return copy


class Spells:
    spells: dict[str, dict[str, Spell]]

    def __init__(self) -> None:
        self.spells = {}

        with open("data/raw/spells/index.json") as f:
            index = json.load(f)

            for source in index.keys():
                self.spells[source] = {}

                with open(f"data/raw/spells/{index[source]}") as f:
                    spells = json.load(f)

                    for spell in spells["spell"]:
                        self.spells[source][spell["name"]] = Spell(spell)

    def as_dict(self) -> dict[str, Any]:
        """This is effectivaly read only as it creates a copy

        Returns:
            dict[str, Any]: the class as a dict
        """

        copy = self.__dict__.copy()
        for source in copy['spells'].keys():
            for spell_name in copy['spells'][source].keys():
                copy['spells'][source][spell_name] = copy['spells'][source][spell_name].as_dict()
        return copy


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
